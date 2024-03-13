use crate::{log::*, utils::config::Configuration, SherlockModule};
use actix::{Actor, Addr, Context, Handler, Message, StreamHandler};
use actix_broker::{BrokerIssue, BrokerSubscribe, SystemBroker};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SherlockMessageEvent;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SherlockMessage {
    #[serde(rename = "type")]
    msg_type: String,
    // TODO: make these enums of empty structs instead of options
    data: Option<serde_json::Value>,
    context: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
pub struct SherlockMessageInternalWrapper {
    id: usize,
    message: SherlockMessage,
}

impl Actor for SherlockMessageEvent {
    type Context = Context<Self>;
}

impl Handler<SherlockMessageInternalWrapper> for SherlockMessageEvent {
    type Result = ();

    fn handle(
        &mut self,
        msg: SherlockMessageInternalWrapper,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        // log::trace!("SherlockMessageEvent recv message => {:?}", msg);
        self.issue_async::<SystemBroker, _>(msg);
    }
}

fn on_ready() {
    debug!("Message bus service started!")
}

fn on_stopping() {
    debug!("Message bus is shutting down...");
}

/// Define HTTP actor
#[derive(Clone)]
struct MessageBus {
    event: web::Data<Addr<SherlockMessageEvent>>,
    id: usize,
}

impl Actor for MessageBus {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<SystemBroker, SherlockMessageInternalWrapper>(ctx);
    }
}

impl Handler<SherlockMessageInternalWrapper> for MessageBus {
    type Result = ();

    fn handle(&mut self, item: SherlockMessageInternalWrapper, ctx: &mut Self::Context) {
        // log::trace!("{:?}", item);
        // self.issue_async::<SystemBroker, _>(item.clone());

        if item.id != self.id {
            if let Ok(json) = serde_json::to_string(&item.message) {
                ctx.text(json);
            } else {
                warn!("could not serialize message to json string.");
            }
        }
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MessageBus {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                // ctx.text(text.clone());
                if let Ok(message) = serde_json::from_str(&text.to_string()) {
                    self.event.do_send(SherlockMessageInternalWrapper {
                        id: self.id,
                        message,
                    })
                } else {
                    ctx.text("{\"response\":\"malformed JSON message.\"}")
                }
            }
            Ok(ws::Message::Binary(_bin)) => ctx.text("{\"response\":\"not yet implemented\"}"), // ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(
    data: web::Data<Addr<SherlockMessageEvent>>,
    counter: web::Data<Mutex<usize>>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        MessageBus {
            event: data,
            id: *counter.lock().unwrap(),
        },
        &req,
        stream,
    );
    // info!("{:?}", resp);
    (*counter.lock().unwrap()) += 1;

    resp
}

#[actix_web::main]
async fn start(configs: Configuration) -> anyhow::Result<()> {
    let msg_event_addr = web::Data::new(SherlockMessageEvent.start());
    let counter = web::Data::new(Mutex::new(0_usize));

    HttpServer::new(move || {
        App::new()
            .app_data(msg_event_addr.clone())
            .app_data(counter.clone())
            .route(&configs.websocket.route, web::get().to(index))
    })
    .bind((configs.websocket.host, configs.websocket.port))?
    .run()
    .await?;

    Ok(())
}

pub fn start_message_bus() -> anyhow::Result<()> {
    logger_init(SherlockModule::MessageBus);

    debug!("Loading message bus configs");

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
            error!("message-bus failed to start: {e}");
            r.store(false, Ordering::SeqCst);
        }
    });

    on_ready();

    while running.load(Ordering::SeqCst) {
        // debug!("waiting...");
        std::thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
