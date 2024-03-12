use crate::{log::*, utils::config::Configuration, SherlockModule};
use actix::{Actor, Addr, Context, Handler, Message, StreamHandler};
use actix_broker::{BrokerIssue, BrokerSubscribe, SystemBroker};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SherlockMessageEvent;

#[derive(Clone, Debug, Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct SherlockMessage {
    hello: String,
}

impl Actor for SherlockMessageEvent {
    type Context = Context<Self>;
}

impl Handler<SherlockMessage> for SherlockMessageEvent {
    type Result = ();

    fn handle(&mut self, msg: SherlockMessage, _ctx: &mut Self::Context) -> Self::Result {
        log::trace!("SherlockMessageEvent recv message => {:?}", msg);
        self.issue_async::<SystemBroker, _>(msg);
    }
}

fn on_ready() {
    debug!("Message bus service started!")
}

fn on_stopping() {
    debug!("Message bus is shutting down...");
}

// TODO: stop from sending the message to the sender.
/// Define HTTP actor
#[derive(Clone)]
struct MessageBus {
    event: web::Data<Addr<SherlockMessageEvent>>,
}

impl Actor for MessageBus {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<SystemBroker, SherlockMessage>(ctx);
    }
}

impl Handler<SherlockMessage> for MessageBus {
    type Result = ();

    fn handle(&mut self, item: SherlockMessage, ctx: &mut Self::Context) {
        log::trace!("{:?}", item);
        self.issue_async::<SystemBroker, _>(item.clone());

        if let Ok(json) = serde_json::to_string(&item) {
            ctx.text(json);
        } else {
            warn!("could not serialize message to json string.");
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
                self.event
                    .do_send(serde_json::from_str(&text.to_string()).unwrap());
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(
    data: web::Data<Addr<SherlockMessageEvent>>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(MessageBus { event: data }, &req, stream);
    info!("{:?}", resp);
    resp
}

#[actix_web::main]
pub async fn start_message_bus() -> anyhow::Result<()> {
    logger_init(SherlockModule::MessageBus);

    debug!("Loading message bus configs");

    let configs = Configuration::get().websocket;

    let msg_event_addr = web::Data::new(SherlockMessageEvent.start());

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        on_stopping();
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    HttpServer::new(move || {
        on_ready();

        App::new()
            .app_data(msg_event_addr.clone())
            .route(&configs.route, web::get().to(index))
    })
    .bind((configs.host, configs.port))?
    .run()
    .await?;

    while running.load(Ordering::SeqCst) {
        std::thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
