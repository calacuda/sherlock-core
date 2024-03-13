use log::error;
use sherlock::message_bus::start_message_bus;

fn main() {
    if let Err(e) = start_message_bus() {
        error!("{:?}", e);
    }
}
