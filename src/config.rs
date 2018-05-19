use std::fs::File;
use std::path::{Path};
use serde_yaml::from_reader;

error_chain! {
    errors {
        ConfigFileNotFound {
            description("config file not found")
            display("config file not found")
        }
        ConfigReadFailed {
            description("failed to read config file")
            display("failed to read config file")
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub token: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(filepath: P) -> Result<Self> {
        let f = File::open(filepath).chain_err(|| ErrorKind::ConfigFileNotFound)?;
        let config: Config = from_reader(f).chain_err(|| ErrorKind::ConfigReadFailed)?;
        Ok(config)
    }
}
