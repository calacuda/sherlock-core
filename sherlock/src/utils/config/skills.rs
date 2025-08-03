use serde::Deserialize;
use std::path::PathBuf;

pub const SKILLS_DIIR: &str = "/opt/sherlock/skills";

#[derive(Clone, Deserialize, Debug)]
pub struct Skills {
    #[serde(default = "default_skill_dir")]
    pub dir: PathBuf,
}

fn default_skill_dir() -> PathBuf {
    PathBuf::from(SKILLS_DIIR)
}

impl Default for Skills {
    fn default() -> Self {
        Self {
            dir: default_skill_dir(),
        }
    }
}
