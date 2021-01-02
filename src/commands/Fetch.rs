use crate::client;
use crate::dm::models::{Bill, File, User};
use crate::dm::DbManager;
use crate::utils::{date, io, users};
use chrono::prelude::*;
use clap::Clap;
use std::error::Error;
use std::path::Path;

#[derive(Clap, Debug)]
pub struct Opts {
  /// bill id to fetch. only fetch a bill with this id.
  #[clap(long = "bill-id")]
  bill_id: Option<String>,
  /// <optional> from-date to search with <%Y-%m-%d> format. ignored if you set bill id.
  #[clap(short, long)]
  from_date: Option<String>,
  /// <optional> to-date with <%Y-%m-%d> format. ignored if you set bill id.
  #[clap(short, long)]
  to_date: Option<String>,
  /// <optional> ignored if you set bill id.
  #[clap(short, long, default_value = "1")]
  page: String,
}

async fn fetch_bill(bill_id: &str) -> Result<(), Box<dyn Error>> {
  let client = client::Client::new();
  client.auth_from_storage().await?;
  println!("success");
  Ok(())
}

async fn fetch_bills(
  page: &str,
  from_date: &str,
  _to_date: Option<&str>,
) -> Result<(), Box<dyn Error>> {
  let client = client::Client::new();
  client.auth_from_storage().await?;

  let to_date = match _to_date {
    Some(td) => td.to_owned(),
    None => date::KstDateTime::from(Utc::now()).format(Some("%Y-%m-%d")),
  };

  let response = client
    .as_ref()
    .fetch_bills(page, from_date, &to_date)
    .await?;

  let formatted_output = format!("{:?}", response);
  io::stdout(formatted_output.as_bytes())?;

  Ok(())
}

pub async fn run(opts: Opts) -> Result<(), Box<dyn Error>> {
  // let dm = DbManager::new();

  if let Some(bill_id) = opts.bill_id {
    fetch_bill(&bill_id).await;
    return Ok(());
  }

  let from_date = match opts.from_date {
    Some(_from_date) => _from_date,
    None => date::KstDateTime::from(Utc::now()).format(Some("%Y-%m-%d")),
  };

  fetch_bills(&opts.page, &from_date, None).await;
  Ok(())
}
