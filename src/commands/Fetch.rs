use crate::client;
use crate::dm::models::{Bill, File, User};
use crate::dm::DbManager;
use crate::fm::FileManager;
use crate::utils::{config, date};
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
  #[clap(short, long)]
  download: bool,
}

async fn fetch_bill(bill_id: &str, with_download: bool) -> Result<(), Box<dyn Error>> {
  let client = client::Client::new();
  client.auth_from_storage().await?;

  let _response = client
    .as_ref()
    .fetch_a_bill(bill_id, bill_id, bill_id)
    .await?;

  if let Some(ref response) = _response {
    if with_download == true {
      let config = config::Config::load_or_new()?;
      let local_repository = config::Config::local_repository();
      match config.file_repository {
        Some(remote_repository) => {
          let fm = FileManager::new(&local_repository, &remote_repository);
          if let Some(ref file_list) = response.atchFileList {
            for file in &*file_list {
              let downloaded = client.download_file(&file.fileUploadNo).await?;
              let f = fm.save(&downloaded, &*response, &file.uploadFileOrginlNm)?;
            }
          };
          fm.upload();
        }
        None => {
          eprintln!("청구파일을 다운로드 하려면 원격저장소 주소를 먼저 설정해주세요.")
        }
      }
    }

    let pretty_response = serde_json::to_string_pretty(&response)?;
    println!("{}", pretty_response);
  }

  Ok(())
}

async fn fetch_bills(
  page: &str,
  _from_date: Option<String>,
  _to_date: Option<&str>,
  with_download: bool,
) -> Result<(), Box<dyn Error>> {
  let client = client::Client::new();
  client.auth_from_storage().await?;

  let from_date = match _from_date {
    Some(date) => date.to_owned(),
    None => date::KstDateTime::from(Utc::now()).format(Some("%Y-%m-%d")),
  };

  let to_date = match _to_date {
    Some(td) => td.to_owned(),
    None => date::KstDateTime::from(Utc::now()).format(Some("%Y-%m-%d")),
  };

  let response = client
    .as_ref()
    .fetch_bills(page, &from_date, &to_date)
    .await?;

  if with_download == true {
    let config = config::Config::load_or_new()?;
    let local_repository = config::Config::local_repository();
    match config.file_repository {
      Some(remote_repository) => {
        std::io::stdout().lock();

        let fm = FileManager::new(&local_repository, &remote_repository);
        let mut file_count: i32 = 0;

        for bill in &response.list {
          let _response_bill = client
            .as_ref()
            .fetch_a_bill(
              &bill.rqestProcRegstrNo,
              &bill.rqestProcRegstrNo,
              &bill.rqestProcRegstrNo,
            )
            .await?;

          if let Some(ref response_bill) = _response_bill {
            if let Some(ref file_list) = response_bill.atchFileList {
              for file in &*file_list {
                file_count = file_count + 1;
                let downloaded = client.download_file(&file.fileUploadNo).await?;
                let f = fm.save(&downloaded, &*response_bill, &file.uploadFileOrginlNm)?;
              }
            };
          }
        }

        fm.upload();
      }
      None => {
        eprintln!("청구파일을 다운로드 하려면 원격저장소 주소를 먼저 설정해주세요.")
      }
    }
  }

  let pretty_response = serde_json::to_string_pretty(&response)?;
  println!("{}", pretty_response);

  Ok(())
}

pub async fn run(opts: Opts) -> Result<(), Box<dyn Error>> {
  if let Some(bill_id) = opts.bill_id {
    fetch_bill(&bill_id, opts.download).await;
    return Ok(());
  }

  fetch_bills(&opts.page, opts.from_date, None, opts.download).await;
  Ok(())
}
