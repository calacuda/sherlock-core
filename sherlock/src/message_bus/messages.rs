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
    #[serde(rename = "speak")]
    Speak,
    #[serde(rename = "spoken")]
    Spoken,
    #[serde(rename = "mycroft.internet.connected")]
    MycroftInternetConnected,
    #[serde(rename = "mycroft.ready")]
    MycroftReady,
}
