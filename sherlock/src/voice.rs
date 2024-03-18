use crate::{
    log::logger_init,
    message_bus::messages::{empty_map, SherlockMessage, SherlockMessageType},
    utils::config::Configuration,
    SherlockModule,
};
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use log::*;
use rodio::Source;
use serde_json::json;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::process::Command;
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};

struct Wav {
    iter: Vec<i16>,
    i: usize,
}

impl Iterator for Wav {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.iter.get(self.i).map(|val| *val);
        self.i += 1;

        res
    }
}

impl Source for Wav {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        22_050
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

#[tokio::main]
pub async fn start_voice_node() -> anyhow::Result<()> {
    logger_init(SherlockModule::Voice);
    let configs = Configuration::get();
    let url = format!(
        "ws://{}:{}/core",
        configs.websocket.host, configs.websocket.port
    );

    debug!("connecting to messagebus at \'{}\'.", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    // TODO: take host from config file
    let mimic_handle = tokio::spawn(
        Command::new("mimic3-server")
            .arg("--host")
            .arg("127.0.0.1")
            .output(),
    );

    debug!("mimic3-server started");
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();

    // Handle incoming messages in a separate task
    let read_handle = tokio::spawn(handle_incoming_messages(ws_stream, handle));

    tokio::try_join!(read_handle, mimic_handle)?.1?;

    Ok(())
}

async fn handle_incoming_messages(
    // mut read: SplitStream<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>>,
    // mut write: SplitSink<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>, Message>,
    mut ws_stream: WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    handle: rodio::OutputStreamHandle,
) {
    let sink = rodio::Sink::try_new(&handle).unwrap();

    while let Some(message) = ws_stream.next().await {
        match message {
            Ok(Message::Text(msg)) => {
                if let Ok(message) = serde_json::from_str::<SherlockMessage>(&msg)
                    && message.msg_type == SherlockMessageType::Speak
                {
                    if let Some(ut) = message.data.get("utterance") {
                        if let Ok(samples) = tts(&ut.to_string()).await {
                            sink.append(Wav {
                                iter: samples,
                                i: 0,
                            });
                            debug!("speaking: {}", ut);
                        } else {
                            warn!("failed to get wav bytes from mimic3-server.")
                        }
                    } else {
                        warn!("speak message had no uterance to speak.");
                    }
                }
            }
            Err(e) => warn!("Error receiving message: {}", e),
            _ => {}
        }
    }
}

async fn tts(ut: &str) -> anyhow::Result<Vec<i16>> {
    Ok(reqwest::Client::new()
        .post("http://127.0.0.1:59125/api/tts")
        .body(ut.to_string())
        .send()
        .await?
        .bytes()
        .await?
        .to_vec()
        .chunks(2)
        .map(|b_s| i16::from_le_bytes([b_s[0], b_s[1]]))
        .collect())
}

async fn speak(
    mut write: SplitSink<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>, Message>,
    utterance: &str,
) -> anyhow::Result<()> {
    let msg = serde_json::to_string(&SherlockMessage {
        msg_type: SherlockMessageType::Speak,
        data: json!({"utterance": utterance}),
        context: empty_map(),
    })?;

    write.send(Message::Text(msg)).await?;

    Ok(())
}
