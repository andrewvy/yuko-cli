extern crate config;
extern crate shellexpand;

use config::{Config, File, FileFormat, ConfigError};

#[derive(Debug, Deserialize)]
pub struct API {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    debug: bool,
    pub api: API
}

impl Settings {
    pub fn new(config_file: &str) -> Result<Self, ConfigError> {
        let mut c = Config::new();

        c.set_default("debug", false)?;

        let file = shellexpand::tilde(config_file);

        c.merge(File::new(&file, FileFormat::Json))?;

        c.try_into()
    }
}
