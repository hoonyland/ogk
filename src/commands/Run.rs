use crate::crawler;
use crate::dm::{models, DbManager};
use crate::utils::users;
use crate::fm::FileManager;
use chrono::prelude::*;
use chrono::{Duration, Local};
use clap::Clap;
use std::error::Error;
use std::path::Path;

#[derive(Clap)]
pub struct Opts {
  #[clap(short, long)]
  from: Option<String>,
}

pub fn run(opts: Opts) -> Result<(), Box<dyn Error>> {
  let dm = DbManager::new();

  let today_datetime: DateTime<Utc> = Utc::now();
  let today_timezone = today_datetime.with_timezone(&FixedOffset::east(9 * 3600)); // KST +09:00
  let today = today_timezone.format("%Y-%m-%d").to_string();
  let yesterday_timezone = today_timezone
    .checked_sub_signed(Duration::days(1))
    .unwrap();
  let yesterday = yesterday_timezone.format("%Y-%m-%d").to_string();
  let from_day = opts.from.unwrap_or(yesterday);

  let users_csv_path = Path::new("users.csv");
  let users = users::load_from_csv(users_csv_path).unwrap();

  for user in &users {
    let client = crawler::Client::new(user);
    let mut fm = FileManager::new(user);
    let _ = fm.clone_remote_repo();

    // 1. 해당 날짜 이전 건 중 공개완료가 아직 안된 건들 업데이트
    let bills_to_update = dm.fetch_bills_to_update(user);
    for bill in bills_to_update {
      match client.fetch_bill(&bill)? {
        None => println!(
          "{}에 대한 세부 페이지가 제공되지 않았습니다.",
          bill.registration_number
        ),
        Some(json_result) => {
          let bill_row = models::Bill::new(&user.username, &json_result.dtlVo);
          bill_row.update(&dm)?;
          models::Bill::download_file(json_result.dntcFileList, &client, &dm, &fm, &bill);
        }
      }
    }

    // 2. 쿼리날짜 기준으로 가져온 건들 추가
    let total_page = models::Bill::fetch_total_page_on_specific_date(&client, &from_day, &today)?;
    for page in 1..total_page {
      let bills = models::Bill::fetch_list_by_page(&client, page)?;

      for bill in bills.list {
        let mut bill_row = models::Bill::new(&user.username, &bill);
        bill_row.save(&dm)?;

        match client.fetch_bill(&bill_row)? {
          None => println!(
            "{}에 대한 세부 페이지가 제공되지 않았습니다.",
            bill_row.registration_number
          ),
          Some(json_result) => {
            let bill_row = models::Bill::new(&user.username, &json_result.dtlVo);
            bill_row.update(&dm)?;
            models::Bill::download_file(json_result.dntcFileList, &client, &dm, &fm, &bill_row);
          }
        }
      }
    }

    match fm.upload() {
      Ok(_) => {
        println!("Done!");
      }
      Err(error) => {
        panic!("{}", error);
      }
    }
  }

  Ok(())
}
