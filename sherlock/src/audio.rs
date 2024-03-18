use crate::{
    message_bus::messages::{SherlockMessage, SherlockMessageType},
    utils::config::Configuration,
};
use actix::{Actor, Addr, Context, Handler, Message, StreamHandler};
use actix_broker::{BrokerIssue, BrokerSubscribe, SystemBroker};
use actix_web::{error, http::StatusCode, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use futures_util::StreamExt;
use log::*;
use serde::{Deserialize, Serialize};
use std::{
    net::TcpStream,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio_tungstenite::connect_async;
use tungstenite::{connect, stream::MaybeTlsStream, WebSocket};

/// Define HTTP actor
struct IntakeServer {
    bus: WebSocket<MaybeTlsStream<TcpStream>>,
    msg_recv_event: web::Data<Addr<MsgRecvEvent>>,
    last_message: Option<SherlockMessage>,
}

#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
pub struct MsgRecv {
    msg: SherlockMessage,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MsgRecvEvent;

impl Actor for MsgRecvEvent {
    type Context = Context<Self>;
}

impl Handler<MsgRecv> for MsgRecvEvent {
    type Result = ();

    fn handle(&mut self, msg: MsgRecv, _ctx: &mut Self::Context) -> Self::Result {
        // log::trace!("SherlockMessageEvent recv message => {:?}", msg);
        self.issue_async::<SystemBroker, _>(msg);
    }
}

impl Actor for IntakeServer {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<SystemBroker, MsgRecv>(ctx);
    }
}

impl Handler<MsgRecv> for IntakeServer {
    type Result = ();

    fn handle(&mut self, item: MsgRecv, ctx: &mut Self::Context) {
        // debug!("connection {} recv'ed a message", self.id);
        if let Ok(json) = serde_json::to_string(&item.msg) {
            let same_message = if let Some(msg) = self.last_message.clone()
                && msg == item.msg
            {
                true
            } else {
                false
            };

            if !same_message || self.last_message.is_none() {
                ctx.text(json);
            }
        } else {
            warn!("could not serialize message to json string.");
        }
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for IntakeServer {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                // ctx.text(text.clone());
                match serde_json::from_str::<SherlockMessage>(&text.to_string()) {
                    Ok(message)
                        if message.msg_type == SherlockMessageType::RecognizerLoopUtterance =>
                    {
                        // debug!("connection {} sent a message", self.id);
                        if let Err(e) = self.bus.send(tungstenite::Message::Text(text.into())) {
                            warn!("could not communicate with internal message-bus: {e}");
                        } else {
                            self.last_message = Some(message);
                        }
                    }
                    Err(e) => {
                        warn!("received a message that doesn't follow Sherlock Message specifications. Did you serialize it using from a SherlockMessage struct/object?. serializetion error: {e}");
                        ctx.text("{\"response\":\"malformed JSON message.\"}")
                    }
                    _ => {}
                }
            }
            Ok(ws::Message::Binary(_bin)) => {
                ctx.text("{\"response\":\"binary messages/responces are not yet implemented\"}")
            } // ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(
    configs: web::Data<Configuration>,
    msg_event_addr: web::Data<Addr<MsgRecvEvent>>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let url = format!(
        "ws://{}:{}/core",
        configs.websocket.host, configs.websocket.port
    );

    if let Ok((ws_stream, _)) = connect(url) {
        let resp = ws::start(
            IntakeServer {
                bus: ws_stream,
                msg_recv_event: msg_event_addr,
                // id,
                last_message: None,
            },
            &req,
            stream,
        );

        resp
    } else {
        // "could not connect to message-bus!"
        Err(error::InternalError::new(
            "could not connect to message-bus!",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into())
    }
}

// TODO: Make this a struct that comunications with the intake server and handles ALL comunicaitons with
// the message bus.
async fn rx_message_bus(
    configs: Arc<Configuration>,
    msg_event_addr: web::Data<Addr<MsgRecvEvent>>,
) {
    let url = format!(
        "ws://{}:{}/core",
        configs.websocket.host, configs.websocket.port
    );

    debug!("connecting to messagebus at \'{}\'.", url);
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    // let (_, read) = ws_stream.split();

    while let Some(message) = ws_stream.next().await {
        if let Ok(msg) = message
            && msg.is_text()
        {
            if let Ok(sherlock_message) = serde_json::from_str::<SherlockMessage>(&format!("{msg}"))
            {
                msg_event_addr.do_send(MsgRecv {
                    msg: sherlock_message,
                });
            }
        }
    }
}

#[actix_web::main]
async fn start(configs: Configuration) -> bool {
    let config_data = web::Data::new(Configuration::get());
    let cfg_dat = (*config_data).clone();
    let msg_event_addr = web::Data::new(MsgRecvEvent.start());
    let mea = msg_event_addr.clone();

    let message_bus_rx = actix_rt::spawn(async move { rx_message_bus(cfg_dat, mea.clone()).await });

    let server_error = Arc::new(AtomicBool::new(false));
    let se = server_error.clone();

    let http_server = actix_rt::spawn(async move {
        match HttpServer::new(move || {
            App::new()
                .app_data(config_data.clone())
                .app_data(msg_event_addr.clone())
                .route(&configs.websocket.route, web::get().to(index))
        })
        .bind((configs.intake.host.clone(), configs.intake.port))
        {
            Ok(server) => {
                if let Err(e) = server.run().await {
                    error!("Intake failed to start: {e}");
                    se.store(true, Ordering::SeqCst);
                }
            }
            Err(e) => {
                error!(
                    "Intake failed to start bid the configured address of {}:{}: {e}",
                    configs.intake.host, configs.intake.port
                );
                se.store(true, Ordering::SeqCst);
            }
        }
    });

    if let (_, Err(e)) = tokio::join!(http_server, message_bus_rx) {
        error!("error starting Audio server: {e}");
        server_error.store(true, Ordering::SeqCst);
    }

    !server_error.load(Ordering::SeqCst)
    // Ok(())
}

fn on_ready() {
    debug!("Audio service started!")
}

fn on_stopping() {
    debug!("Audio service is shutting down...");
}

pub fn start_intake_server() {
    debug!("Loading intake server configs");

    let configs = Configuration::get();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        on_stopping();
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let r = running.clone();

    std::thread::spawn(move || {
        // if let Err(_) = start(configs) {
        // error!("Intake failed to start: {e}");
        r.store(start(configs), Ordering::SeqCst);
        // }
    });

    // std::thread::spawn(move || {
    //     if let Err(e) = start(configs) {
    //         error!("Intake failed to start: {e}");
    //         r.store(false, Ordering::SeqCst);
    //     }
    // });

    on_ready();

    while running.load(Ordering::SeqCst) {
        // debug!("waiting...");
        std::thread::sleep(Duration::from_secs(1));
    }
}
