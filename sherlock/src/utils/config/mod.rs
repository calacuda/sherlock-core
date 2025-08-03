use figment::{
    providers::{Format, Toml},
    Figment,
};
use log::error;
use serde::Deserialize;

pub mod intake;
pub mod skills;
pub mod websocket;

#[derive(Default, Clone, Deserialize, Debug)]
pub struct Configuration {
    #[serde(rename = "message_bus")]
    pub websocket: websocket::Websocket,
    pub intake: intake::Intake,
    pub skills: skills::Skills,
}

impl Configuration {
    pub fn get() -> Self {
        if let Ok(dirs) = xdg::BaseDirectories::new() {
            if let Ok(config_file) = dirs.place_config_file("sherlock/sherlock.toml") {
                if !config_file.exists() {
                    let _ = std::fs::File::create(config_file.clone());
                }

                match Figment::new()
                    .merge(Toml::file(config_file.clone()))
                    .extract()
                {
                    Ok(config) => config,
                    Err(e) => {
                        error!(
                            "could not load config file at \"{}\". got error: {e}",
                            config_file.to_string_lossy()
                        );
                        Self::default()
                    }
                }
            } else {
                error!("unable to find/create config directory.");
                Self::default()
            }
        } else {
            error!("unknown xdg directories. can't load config file.");
            Self::default()
        }
    }
}
