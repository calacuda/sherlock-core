use crate::{
    log::logger_init,
    message_bus::{
        self,
        messages::{empty_map, SherlockMessage, SherlockMessageType},
    },
    utils::config::Configuration,
    SherlockModule,
};
use anyhow::bail;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, warn};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    fs::read_dir,
    path::PathBuf,
    process::Stdio,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    usize,
};
use tokio::{
    io::AsyncWriteExt,
    process::{Child, ChildStderr, ChildStdin, ChildStdout, Command},
    time::Duration,
};
use tokio_tungstenite::connect_async;

#[derive(Clone, Debug, Deserialize)]
struct SkillMetadata {
    name: String,
    executable: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
struct SkillVariable {
    name: String,
    #[serde(default = "tautology")]
    optional: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct SkillFunc {
    variables: Vec<SkillVariable>,
}

#[derive(Clone, Debug, Deserialize)]
struct SkillConfig {
    metadata: SkillMetadata,
    functions: HashMap<String, SkillFunc>,
}

pub struct Skill {
    dir: PathBuf,
    config_file: PathBuf,
    exec: PathBuf,
    proc: Child,
    send: ChildStdin,
    recv: ChildStdout,
    log: ChildStderr,
    metadata: SkillMetadata,
    functions: HashMap<String, SkillFunc>,
}

impl Skill {
    pub fn load(dir: &PathBuf) -> anyhow::Result<Self> {
        let config_file = dir.join("skill.toml");
        let skill_conf: SkillConfig = Figment::new()
            .merge(Toml::file(config_file.clone()))
            .extract()?;
        let exec = skill_conf.metadata.executable.clone();
        let mut proc = Command::new(dir.join(exec.clone()))
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;
        let send = proc
            .stdin
            .take()
            .map_or_else(|| bail!("no stdin"), |value| Ok(value))?;
        let recv = proc
            .stdout
            .take()
            .map_or_else(|| bail!("no stdout"), |value| Ok(value))?;
        let log = proc
            .stderr
            .take()
            .map_or_else(|| bail!("no stderr"), |value| Ok(value))?;

        Ok(Self {
            dir: dir.clone(),
            config_file,
            exec,
            proc,
            send,
            recv,
            log,
            metadata: skill_conf.metadata,
            functions: skill_conf.functions,
        })
    }
}

fn extract_from(var_names: &[SkillVariable], utterance: &str) -> HashMap<String, String> {
    // build regex from var_names
    //
}

struct Skills {
    skills: Vec<Skill>,
    active_skill: Option<(usize, String)>,
}

impl Skills {
    pub fn new(skills: Vec<PathBuf>) -> Self {
        Self {
            skills: skills
                .iter()
                .filter_map(|dir| {
                    Skill::load(dir).map_or_else(
                        |e| {
                            warn!(
                                "failed to load skill in dir: {}. got error {e}",
                                dir.to_string_lossy()
                            );
                            None
                        },
                        |val| Some(val),
                    )
                })
                .collect(),
            active_skill: None,
        }
    }

    async fn parse_utterance(&mut self, utterance_data: Value) -> Vec<SherlockMessage> {
        let mut messages = Vec::new();

        if let Some(utter) = utterance_data.get("utterance") {
            let utterance = utter.to_string().to_lowercase();

            if let Some((i, f)) = self.active_skill.clone() {
                let function_name = f.clone();
                let function_data = self.skills[i].functions.get(&f);

                if let Some(func_dat) = function_data {
                    // parse variables
                    let var_names = &func_dat.variables;
                    let variables = extract_from(var_names, &utterance);
                    let vars = serde_json::to_string(&variables).unwrap();

                    // send variables to skill at self.skills[i]
                    self.skills[i].send.write(format!("{vars}\n").as_bytes());
                } else {
                    self.active_skill = None;
                    let skill_name = self.skills[i].metadata.name.clone();

                    messages.push(SherlockMessage {
                            msg_type: SherlockMessageType::Speak,
                            data: json!( {"utterance": format!("the function {function_name} from skill {skill_name} does not exist"), "lang": "en-us"} ),
                            context: empty_map(),
                        });
                }
            } else {
                if !self.assign_i(&utterance).await {
                    messages.push(SherlockMessage {
                            msg_type: SherlockMessageType::Speak,
                            data: json!( {"utterance": format!("unknown or not installed skill"), "lang": "en-us"} ),
                            context: empty_map(),
                        });
                }
            }
        }

        messages
    }

    async fn assign_i(&mut self, utterance: &str) -> bool {
        if utterance.starts_with("skill") || utterance.ends_with("skill") {
            let skill_name = utterance.replace("skill", "");
            let (skill_name, func_name) = if skill_name.contains(" dot ") {
                skill_name.split_once(" dot ").unwrap()
            } else {
                skill_name.split_once(".").unwrap()
            };

            self.skills.iter().enumerate().for_each(|(i, skill)| {
                if skill.metadata.name == skill_name && skill.functions.contains_key(func_name) {
                    self.active_skill = Some((i, func_name.to_string()));
                }
            });

            self.active_skill.is_some()
        } else {
            false
        }
    }
}

// impl Index<String> for Skills {
//     type Output = Skill;
//
//     fn index(&self, index: String) -> &Self::Output {
//
//     }
// }
//
// impl IndexMut<String> for Skills {
//     fn index_mut(&mut self, index: String) -> &mut Self::Output {}
// }

fn tautology() -> bool {
    true
}

#[tokio::main]
async fn event_loop() -> anyhow::Result<()> {
    let configs = Configuration::get();

    // read all directories from skills dir
    let found_skill_dirs = read_dir(&configs.skills.dir)?
        .filter_map(|directory| directory.map_or_else(|_| None, |dir| Some(dir.path())))
        .collect();

    let url = format!(
        "ws://{}:{}/core",
        configs.websocket.host, configs.websocket.port
    );

    let mut skills = Skills::new(found_skill_dirs);

    debug!("connecting to messagebus at \'{}\'.", url);
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    while let Some(message) = ws_stream.next().await {
        if let Ok(msg) = message
            && msg.is_text()
        {
            if let Ok(message) = serde_json::from_str::<SherlockMessage>(&msg.to_string())
                && message.msg_type == SherlockMessageType::RecognizerLoopUtterance
            {
                for send_msg in skills.parse_utterance(message.data).await.iter() {
                    if let Ok(json_str) = serde_json::to_string(send_msg) {
                        if let Err(e) = ws_stream.send(tungstenite::Message::Text(json_str)).await {
                            error!("could not send a skill message to the message-bus: {e}");
                        }
                    } else {
                        error!("failed to serialise json message from: {:?}", send_msg);
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn start_skill_runner() -> anyhow::Result<()> {
    logger_init(SherlockModule::Skills);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let r2 = running.clone();

    ctrlc::set_handler(move || {
        debug!("Stopping skill runner.");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // let mb_handle = spawn(async move {
    //
    //
    //     while let Some(message) = ws_stream.next().await {
    //         if let Ok(msg) = message
    //             && msg.is_text()
    //         {
    //             if let Ok(sherlock_message) =
    //                 serde_json::from_str::<SherlockMessage>(&msg.to_string())
    //                 && sherlock_message.msg_type == SherlockMessageType::RecognizerLoopUtterance
    //             {
    //                 if let Err(e) = tx.send(sherlock_message).await {
    //                     error!(
    //                         "skills messagebus error, could not communicate with skill chooser: {e}"
    //                     )
    //                 }
    //             }
    //         }
    //     }
    // });
    // TODO: connect to message_bus and recv commands
    // TODO: wait for recognizer_loop:utterance messages then parse and send message to skills
    // runner thread.

    std::thread::spawn(move || {
        if let Err(e) = event_loop() {
            error!("Error running skills event loop: {e}");
        }

        r2.store(false, Ordering::SeqCst);
    });

    while running.load(Ordering::SeqCst) {
        // debug!("waiting...");
        std::thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
