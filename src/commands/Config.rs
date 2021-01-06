use crate::utils::config::Config;
use crate::vars::MAIN_AUTHOR;
use clap::Clap;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
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
  println!("{}", toml::to_string(&config)?);
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
