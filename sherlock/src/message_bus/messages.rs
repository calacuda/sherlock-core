use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SherlockMessage {
    #[serde(rename = "type")]
    msg_type: SherlockMessageType,
    #[serde(default = "empty_map")]
    data: HashMap<String, Value>,
    #[serde(default = "empty_map")]
    context: HashMap<String, Value>,
}

fn empty_map() -> HashMap<String, Value> {
    HashMap::new()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SherlockMessageType {
    /// Request to speak utterance
    #[serde(rename = "speak")]
    Speak,
    /// Speak message data as audio, emitted by the TTS Node.
    #[serde(rename = "spoken")]
    Spoken,
    /// Internet connection is now available (only generated on initial connection)
    #[serde(rename = "mycroft.internet.connected")]
    MycroftInternetConnected,
    /// Sent by start-up sequence when everything is ready for user interaction
    #[serde(rename = "mycroft.ready")]
    MycroftReady,
    /// Stop command (e.g. button pressed)
    #[serde(rename = "mycroft.stop")]
    MycroftStop,
}
