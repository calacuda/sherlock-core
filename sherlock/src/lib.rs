use std::fmt::Display;

pub mod log;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SherlockModule {
    /// represents the "audio" intake, AKA the websockets input that recieves user uteranceses
    Audio,
    /// the message-bus that is used to pass messages between sherlock components
    MessageBus,
    /// the skill picker, dispacher and the Skills them selves
    Skills,
    /// the module reponsible for making sherlock speek. (ie. mimic3, or other TTS engines.)
    Voice,
}

impl Display for SherlockModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Audio => write!(f, "audio"),
            Self::MessageBus => write!(f, "bus"),
            Self::Skills => write!(f, "skills"),
            Self::Voice => write!(f, "voice"),
        }
    }
}
