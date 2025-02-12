use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RepositoryConfig {
    pub file_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub repository: RepositoryConfig,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        Config::builder()
            .add_source(File::with_name("config"))
            .build()?
            .try_deserialize::<Settings>()
    }
}
