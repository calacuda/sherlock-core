use log::error;
use sherlock::skills::start_skill_runner;

fn main() {
    if let Err(e) = start_skill_runner() {
        error!("encountered error when running the skills runner: {e}");
    }
}
