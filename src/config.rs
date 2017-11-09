// We're using TOML for the configuration format
extern crate toml;

// Bring back std into the namespace, remember: we're on the "config" namespace here.
use ::std;

// Config struct should be automatically deserializable
#[derive(Deserialize)]
pub struct Config {
    require_https: bool,
    pub jenkins: JenkinsConfig
}

#[derive(Deserialize)]
pub struct JenkinsConfig {
    pub server: String,
}

// Our custom error type here, an enum describing that either an IO error or a Parse error happened.
#[derive(Debug)]
pub enum ReadConfigError {
    IO(std::io::Error),
    Parse(toml::de::Error),
}

// From implementations for our error type, for ergonimics with the ?-syntax
impl From<std::io::Error> for ReadConfigError {
    fn from(err: std::io::Error) -> Self {
        ReadConfigError::IO(err)
    }
} 

impl From<toml::de::Error> for ReadConfigError {
    fn from(err: toml::de::Error) -> Self {
        ReadConfigError::Parse(err)
    }
} 

// Function to read the config at a certain path
pub fn read_config(path: &str) -> Result<Config, ReadConfigError> {
    use std::io::Read;

    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let cfg = toml::from_str::<Config>(&contents)?;
    return Ok(cfg);
}