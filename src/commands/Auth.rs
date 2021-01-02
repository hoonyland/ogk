use crate::client;
use crate::utils::auth::{AuthConfig, AuthUser};
use clap::Clap;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;

const MAIN_AUTHOR: &str = "Hoony <hoonyland@protonmail>";

#[derive(Clap, Debug)]
struct LoginOpts {
  #[clap(short, long)]
  username: String,
  #[clap(short, long)]
  password: String,
}

#[derive(Clap, Debug)]
struct GenKeyOpts {}

#[derive(Clap, Debug)]
enum SubCommand {
  /// authenticate to open.go.kr and store credential information
  #[clap(version = "0.1", author = MAIN_AUTHOR)]
  Login(LoginOpts),
  /// generate jwt token to authenticate without password
  #[clap(version = "0.1", author = MAIN_AUTHOR)]
  GenKey(GenKeyOpts),
}

#[derive(Clap, Debug)]
pub struct Opts {
  #[clap(subcommand)]
  subcmd: SubCommand,
}

async fn login(opts: &LoginOpts) -> Result<(), Box<dyn std::error::Error>> {
  let username = &opts.username;
  let password = &opts.password;

  let client = client::Client::new();
  let _ = client.as_ref().auth(username, password).await?;

  let config = AuthConfig::new(username, password);
  let _ = config.save()?;

  Ok(())
}

fn gen_key() -> Result<(), Box<dyn Error>> {
  unimplemented!()
}

pub async fn run(opts: Opts) -> Result<(), Box<dyn std::error::Error>> {
  match opts.subcmd {
    SubCommand::Login(sub_opts) => {
      let _ = login(&sub_opts).await;
    }
    SubCommand::GenKey(_) => {
      gen_key();
    }
  }

  Ok(())
}
