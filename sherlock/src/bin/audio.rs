use sherlock::{audio::start_intake_server, log::logger_init, SherlockModule};

fn main() {
    logger_init(SherlockModule::Audio);

    start_intake_server();
}
