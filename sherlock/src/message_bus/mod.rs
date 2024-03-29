use crate::{log::*, utils::config::Configuration, SherlockModule};
use actix::{Actor, Addr, Context, Handler, Message, StreamHandler};
use actix_broker::{BrokerIssue, BrokerSubscribe, SystemBroker};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use messages::SherlockMessage;
use serde::{Deserialize, Serialize};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use uuid::Uuid;

pub mod messages;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SherlockMessageEvent;

#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
pub struct SherlockMessageInternalWrapper {
    id: Uuid,
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
    id: Uuid,
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
        if item.id != self.id {
            debug!("connection {} recv'ed a message", self.id);
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
                    debug!("connection {} sent a message", self.id);
                    self.event.do_send(SherlockMessageInternalWrapper {
                        id: self.id,
                        message,
                    })
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
    data: web::Data<Addr<SherlockMessageEvent>>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let id = Uuid::new_v4();
    let resp = ws::start(MessageBus { event: data, id }, &req, stream);
    // info!("{:?}", resp);
    info!("ID => {id}");

    resp
}

#[actix_web::main]
async fn start(configs: Configuration) -> anyhow::Result<()> {
    let msg_event_addr = web::Data::new(SherlockMessageEvent.start());

    HttpServer::new(move || {
        App::new()
            .app_data(msg_event_addr.clone())
            .route(&configs.websocket.route, web::get().to(index))
    })
    .bind((configs.websocket.host, configs.websocket.port))?
    .run()
    .await?;

    Ok(())
}

pub fn start_message_bus() {
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
}
