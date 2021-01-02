use crate::utils::io::stdout;
use crate::vars::MAIN_AUTHOR;
use clap::Clap;
use serde::{Deserialize, Serialize};
use std::env::home_dir;
use std::error::Error;
use std::fs::{create_dir_all, read_to_string, File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub async fn run(opts: Opts) -> Result<(), Box<dyn Error>> {
  match opts.subcmd {
    SubCommand::Database(sub_opts) => {
      let _ = database(&sub_opts).await;
    }
    SubCommand::List(sub_opts) => {
      let _ = list(&sub_opts).await;
    }
    SubCommand::Repository(sub_opts) => {
      let _ = repository(&sub_opts).await;
    }
  }

  Ok(())
}

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

  pub fn root_path() -> String {
    format!("{}/{}", home_dir().unwrap().to_str().unwrap(), ".ogk")
  }
}

async fn database(opts: &DatabaseOpts) -> Result<(), Box<dyn Error>> {
  let mut config = Config::load_or_new()?;
  if let Some(host) = &opts.host {
    config.db_host = Some(host.to_owned());
  };

  if let Some(username) = &opts.username {
    config.db_username = Some(username.to_owned());
  };

  if let Some(password) = &opts.password {
    config.db_password = Some(password.to_owned());
  };

  config.save()?;

  Ok(())
}

async fn list(_opts: &ListOpts) -> Result<(), Box<dyn Error>> {
  let mut config = Config::load_or_new()?;
  stdout(toml::to_string(&config)?.as_bytes());
  Ok(())
}

async fn repository(opts: &RepositoryOpts) -> Result<(), Box<dyn Error>> {
  let mut config = Config::load_or_new()?;
  if let Some(file_repository) = &opts.file {
    config.file_repository = Some(file_repository.to_owned());
    config.save()?;
  };

  Ok(())
}

#[derive(Clap, Debug)]
pub struct Opts {
  #[clap(subcommand)]
  subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
  #[clap(version = "0.1", author = MAIN_AUTHOR)]
  Database(DatabaseOpts),
  #[clap(version = "0.1", author = MAIN_AUTHOR)]
  List(ListOpts),
  #[clap(version = "0.1", author = MAIN_AUTHOR)]
  Repository(RepositoryOpts),
}

#[derive(Clap, Debug)]
struct DatabaseOpts {
  #[clap(short, long)]
  host: Option<String>,
  #[clap(short, long)]
  password: Option<String>,
  #[clap(short, long)]
  username: Option<String>,
}

#[derive(Clap, Debug)]
struct ListOpts {}

#[derive(Clap, Debug)]
struct RepositoryOpts {
  #[clap(short, long)]
  file: Option<String>,
}
