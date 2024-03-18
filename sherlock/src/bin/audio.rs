#![feature(let_chains)]
use std::{
    net::TcpStream,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use actix::{Actor, AsyncContext, Handler, Message, StreamHandler};
use actix_web::{error, http::StatusCode, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use log::*;
use sherlock::{
    log::logger_init,
    message_bus::messages::{SherlockMessage, SherlockMessageType},
    utils::config::Configuration,
    SherlockModule,
};
use tungstenite::{connect, stream::MaybeTlsStream, WebSocket};
use uuid::Uuid;

/// Define HTTP actor
struct IntakeServer {
    bus: WebSocket<MaybeTlsStream<TcpStream>>,
    id: Uuid,
}

impl Actor for IntakeServer {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // TODO: read messages from message-bus and send speak messages to client
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for IntakeServer {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                // ctx.text(text.clone());
                if let Ok(message) = serde_json::from_str::<SherlockMessage>(&text.to_string())
                    && message.msg_type == SherlockMessageType::RecognizerLoopUtterance
                {
                    // debug!("connection {} sent a message", self.id);
                    if let Err(e) = self.bus.send(tungstenite::Message::Text(text.into())) {
                        warn!("could not communicate with internal message-bus: {e}");
                    }
                } else {
                    warn!("received a message that doesn't follow Sherlock Message specifications. Did you serialize it using from a SherlockMessage struct/object?");
                    ctx.text("{\"response\":\"malformed JSON message.\"}")
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
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let id = Uuid::new_v4();
    let url = format!(
        "ws://{}:{}/core",
        configs.websocket.host, configs.websocket.port
    );

    if let Ok((ws_stream, _)) = connect(url) {
        let resp = ws::start(IntakeServer { bus: ws_stream, id }, &req, stream);
        // info!("{:?}", resp);
        info!("ID => {id}");

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

#[actix_web::main]
async fn start(configs: Configuration) -> anyhow::Result<()> {
    let msg_event_addr = web::Data::new(Configuration::get());

    HttpServer::new(move || {
        App::new()
            .app_data(msg_event_addr.clone())
            .route(&configs.websocket.route, web::get().to(index))
    })
    .bind((configs.intake.host, configs.intake.port))?
    .run()
    .await?;

    Ok(())
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
        if let Err(e) = start(configs) {
            error!("Intake failed to start: {e}");
            r.store(false, Ordering::SeqCst);
        }
    });

    on_ready();

    while running.load(Ordering::SeqCst) {
        // debug!("waiting...");
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    logger_init(SherlockModule::Audio);

    // info!("starts the main ingress server that excepts http/websocat requests, and returns the text/TTS speech.");
    start_intake_server();
}
