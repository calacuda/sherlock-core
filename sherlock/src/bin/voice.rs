use log::error;
use sherlock::voice::start_voice_node;

fn main() {
    if let Err(e) = start_voice_node() {
        error!("starting voice node failed with error: {e}");
    }
}
