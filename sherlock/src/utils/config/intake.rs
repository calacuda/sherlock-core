use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Intake {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_route")]
    pub route: String,
    /// TODO: make point to ssl cert file in the future
    #[serde(default = "default_ssl")]
    pub ssl: bool,
}

fn default_host() -> String {
    "127.0.0.1".into()
}

fn default_port() -> u16 {
    8080
}

fn default_route() -> String {
    "/sherlock".into()
}

fn default_ssl() -> bool {
    false
}

impl Default for Intake {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: 8080,
            route: "/sherlock".to_string(),
            ssl: false,
        }
    }
}
