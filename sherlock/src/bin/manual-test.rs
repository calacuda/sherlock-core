use log::*;
use sherlock::{log::*, message_bus::start_message_bus, SherlockModule};

fn main() {
    logger_init(SherlockModule::MessageBus);

    println!("** Start **");
    trace!("log based logging test");
    debug!("log based logging test");
    info!("log based logging test");
    warn!("log based logging test");
    error!("log based logging test");
    println!("** End **");

    println!();

    println!("** Message Bus test Start **");
    if let Err(e) = start_message_bus() {
        error!("{:?}", e);
    }
    println!("** Message Bus test End **");
}
