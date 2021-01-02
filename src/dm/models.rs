use crate::client;
use crate::dm;
use crate::dm::scheme::*;
// use crate::fm;
// use crate::utils::date::add_month;
// use chrono::prelude::*;
// use chrono::{Duration, Local};
use crypto::digest::Digest;
use crypto::sha1::Sha1;
// use diesel::prelude::*;
// use std::error::Error;

#[derive(Insertable, Debug, Queryable, serde::Serialize)]
#[table_name = "bills"]
pub struct Bill {
  pub id: String,
  pub group_id: String,
  pub proc_org_code: String,
  pub proc_org_name: String,
  pub proc_registration_number: String,
  pub public_date: Option<String>,
  pub registration_number: String,
  pub request_date: String,
  pub request_description: String,
  pub request_proc_registration_number: String,
  pub request_subject: String,
  pub result_description: Option<String>,
  pub status: String,
  pub user_id: String,
}

impl Bill {
  pub fn new(user_id: &str, bill: &client::DtlVo) -> Self {
    // create a Sha1 object to create group id
    let mut hasher = Sha1::new();
    hasher.input_str(format!("{}_{}", &bill.rqestSj, &bill.rqestCn).as_str());

    let mut result_description: Option<String> = None;
    if !bill.decsnCn.is_empty() {
      result_description = Some(bill.decsnCn.clone());
    } else if !bill.clsdrResnCn.is_empty() {
      result_description = Some(bill.clsdrResnCn.clone());
    }

    Bill {
      id: format!("{}_{}", &bill.rqestRceptNo, &bill.prcsInsttCd),
      group_id: hasher.result_str(),
      proc_org_code: bill.prcsInsttCd.clone(),
      proc_org_name: bill.prcsFullInsttNm.clone(),
      proc_registration_number: bill.procRegstrNo.clone(),
      public_date: None,
      registration_number: bill.rqestRceptNo.clone(),
      request_date: bill.rceptDt.replace(".", "-").clone(),
      request_description: bill.rqestCn.clone(),
      request_proc_registration_number: bill.rqestProcRegstrNo.clone(),
      request_subject: bill.rqestSj.clone(),
      result_description: result_description,
      status: bill.insttRqestProcStNm.clone(),
      user_id: user_id.to_owned(),
    }
  }

  // pub async fn fetch_list_by_page(
  //   client: &crawler::Client,
  //   page: u32,
  // ) -> impl futures::Future<Output = crawler::Bills, reqwest::Error> {
  //   let response = client.fetch_bills_all(&page.to_string()).await?;
  //   response.json::<crawler::Bills>()
  // }

  // pub fn fetch_list_by_page_on_specific_date(
  //   client: &crawler::Client,
  //   page: i32,
  //   from_date: &str,
  //   to_date: &str,
  // ) -> Result<crawler::Bills, Box<dyn Error>> {
  //   let bills_response = client.fetch_bills(from_date, to_date, &page.to_string())?;
  //   let bills = bills_response.json::<crawler::Bills>()?;
  //   Ok(bills)
  // }

  // pub fn fetch_total_page(client: &crawler::Client) -> Result<u32, Box<dyn Error>> {
  //   let bills_response = client.fetch_bills_all("1")?;
  //   let bills = bills_response.json::<crawler::Bills>()?;
  //   let total_items = bills.vo.totalPage;
  //   let mut total_page = (total_items / 10) as u32;
  //   if total_items % 10 != 0 {
  //     total_page = total_page + 1;
  //   }
  //   Ok(total_page)
  // }

  // pub fn fetch_total_page_on_specific_date(
  //   client: &crawler::Client,
  //   from_date: &str,
  //   to_date: &str,
  // ) -> Result<u32, Box<dyn Error>> {
  //   let bills_response = client.fetch_bills(from_date, to_date, "1")?;
  //   let bills = bills_response.json::<crawler::Bills>()?;
  //   let total_items = bills.vo.totalPage;
  //   let mut total_page = (total_items / 10) as u32;
  //   if total_items % 10 != 0 {
  //     total_page = total_page + 1;
  //   }
  //   Ok(total_page)
  // }

  // pub fn save(&self, dm: &dm::DbManager) -> Result<(), Box<dyn Error>> {
  //   match dm.insert_bill(self) {
  //     Ok(_) => (Ok(())),
  //     Err(err) => {
  //       println!(
  //         "통보완료된 \"{}\"건에 해당하는 정보를 DB에 저장하는 것을 실패하였습니다.",
  //         self.request_subject
  //       );

  //       println!("{}", err.description().contains("Duplicate entry"));

  //       if (err.description()).contains("Duplicate entry") != true {
  //         panic!(err);
  //       }

  //       Ok(())
  //     }
  //   }
  // }

  // pub fn download_file(
  //   _files: Option<Vec<crawler::DntcFile>>,
  //   client: &crawler::Client,
  //   dm: &dm::DbManager,
  //   fm: &fm::FileManager,
  //   bill: &Bill,
  // ) -> Result<(), Box<dyn Error>> {
  //   match _files {
  //     None => Ok(()),
  //     Some(files) => {
  //       for file in files {
  //         match client.download_file(&file) {
  //           Ok(downloaded) => {
  //             let _ = File::download(&fm, &dm, &bill, &file, &downloaded);
  //           }
  //           Err(error) => {
  //             println!("{:?}", error);
  //           }
  //         }
  //       }

  //       Ok(())
  //     }
  //   }
  // }

  // pub fn set_public_date(
  //   &mut self,
  //   user: &dm::models::User,
  // ) -> Result<&mut dm::models::Bill, Box<dyn Error>> {
  //   let public_datetime: Option<DateTime<Utc>> = match user.embago_month {
  //     Some(em) => {
  //       let request_datetime = Utc
  //         .datetime_from_str(
  //           &format!("{} 00:00:00", self.request_date),
  //           "%Y-%m-%d %H:%M:%S",
  //         )
  //         .unwrap();

  //       Some(add_month(request_datetime, em))
  //     }
  //     None => None,
  //   };

  //   self.public_date = match public_datetime {
  //     Some(pd) => Some(pd.format("%Y-%m-%d").to_string()),
  //     None => Some(String::new()),
  //   };

  //   Ok(self)
  // }

  // pub fn update(&self, dm: &dm::DbManager) -> Result<(), Box<dyn Error>> {
  //   match dm.update_bill(self) {
  //     Ok(_) => (Ok(())),
  //     Err(err) => {
  //       println!(
  //         "통보완료된 \"{}\"건에 해당하는 정보를 DB에 수정하는 것을 실패하였습니다.",
  //         self.request_subject
  //       );

  //       if !(err.description()).contains("Duplicate entry") {
  //         panic!(err);
  //       }

  //       Ok(())
  //     }
  //   }
  // }
}

#[derive(serde::Deserialize, Insertable, Debug, Queryable)]
#[table_name = "users"]
pub struct User {
  pub embago_month: Option<i32>,
  pub username: String,
  pub password: String,
}

impl User {
  pub fn new(username: &str, password: &str) -> Self {
    User {
      embago_month: None,
      username: username.to_owned(),
      password: password.to_owned(),
    }
  }

  // pub fn save(self, dm: &dm::DbManager) -> Result<(), Box<dyn Error>> {
  //   dm.insert_user(self)
  // }
}

#[derive(Insertable, Debug, Queryable)]
#[table_name = "files"]
pub struct File {
  pub id: Option<i32>,
  pub filename: String,
  pub bill_id: String,
}

impl File {
  pub fn new(filename: &str, bill_id: &str) -> Self {
    File {
      id: None,
      filename: filename.to_owned(),
      bill_id: bill_id.to_owned(),
    }
  }

  // pub fn download(
  //   fm: &fm::FileManager,
  //   dm: &dm::DbManager,
  //   bill: &Bill,
  //   file: &crawler::DntcFile,
  //   bytes: &bytes::Bytes,
  // ) -> Result<(), Box<dyn Error>> {
  //   fm.save(&bill, &file, &bytes, false);
  //   let file_row = dm::models::File::new(
  //     &fm::FileManager::make_filename(&bill, &file),
  //     &bill.registration_number,
  //   );

  //   match dm.insert_file(&file_row) {
  //     Ok(_) => Ok(()),
  //     Err(err) => {
  //       println!(
  //         "파일: \"{}\"을 디비에 저장하는 것을 실패하였습니다.",
  //         &file_row.filename
  //       );

  //       if !(err.description()).contains("Duplicate entry") {
  //         panic!(err);
  //       }

  //       Ok(())
  //     }
  //   }
  // }
}
