use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{create_dir_all, read_to_string, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
  pub file_repository: Option<String>,
  pub db_host: Option<String>,
  pub db_username: Option<String>,
  pub db_password: Option<String>,
}

impl Config {
  pub fn new() -> Self {
    Config {
      file_repository: None,
      db_host: None,
      db_username: None,
      db_password: None,
    }
  }

  pub fn load() -> Result<Config, Box<dyn Error>> {
    let file_path = Config::file_path();
    let config_file = read_to_string(file_path)?;
    let config = toml::from_str(&config_file)?;
    Ok(config)
  }

  pub fn load_or_new() -> Result<Config, Box<dyn Error>> {
    let file_path = Config::file_path();
    match read_to_string(file_path) {
      Ok(config_file) => {
        let config = toml::from_str(&config_file)?;
        return Ok(config);
      }
      Err(_) => {
        return Ok(Config::new());
      }
    }
  }

  pub fn save(&self) -> Result<(), Box<dyn Error>> {
    let toml = toml::to_string(self)?;
    let realopen_path = Config::root_path();
    let file_path = Config::file_path();
    create_dir_all(Path::new(&realopen_path))?;
    let mut local_file = File::create(Path::new(&file_path))?;
    local_file.write_all(toml.as_bytes())?;
    Ok(())
  }

  pub fn file_path() -> String {
    format!("{}/{}", Config::root_path(), "config")
  }

  pub fn local_repository() -> String {
    format!("{}/{}", Config::root_path(), ".data")
  }

  pub fn root_path() -> String {
    format!("{}/{}", home_dir().unwrap().to_str().unwrap(), ".ogk")
  }
}
