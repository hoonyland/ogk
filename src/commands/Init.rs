use crate::client;
use crate::dm::models::{Bill, File, User};
use crate::dm::DbManager;
// use crate::fm::FileManager;
use crate::utils::users;
use clap::Clap;
use std::error::Error;
use std::path::Path;

#[derive(Clap, Debug)]
pub struct Opts {}

/**
 * 1. Upload users data to remote database storage
 * 2. Fetch all bills from initial date to yesterday
 */
pub async fn run(_: Opts) -> Result<(), Box<dyn Error>> {
  let client = client::Client::new();
  let dm = DbManager::new();

  let users_csv_path = Path::new("users.csv");
  let users = users::load_from_csv(users_csv_path).unwrap();

  // // 1. Upload users to remote database storage
  // for user in &users {
  //   let user_row = User::new(&user.username, &user.password);
  //   user_row.save(&dm)?;
  // }

  for user in &users {
    let _ = client.as_ref().auth(&user.username, &user.password).await;
    let f1 = client.as_ref().fetch_bills_all("1");
    let f2 = client.as_ref().fetch_bills_all("2");
    let f3 = client.as_ref().fetch_bills_all("3");

    let (r1, r2, r3) = futures::future::join3(f1, f2, f3).await;
    let bills3 = r3.unwrap().json::<client::Bills>().await.unwrap();
    let bills1 = r1.unwrap().json::<client::Bills>().await.unwrap();
    let bills2 = r2.unwrap().json::<client::Bills>().await.unwrap();
  }
  // users.into_iter().map(|user| async move {
  // });
  // for user in &users {
  //   // let mut fm = FileManager::new(user);
  //   // let _ = fm.clone_remote_repo();

  //   // let bills = client.fetch_bills_all("2").await;

  //   // let bills = Bill::fetch_list_by_page(&client, 1)?;
  //   // let total_page = Bill::fetch_total_page(&client)?;
  //   // println!("총 {} 페이지를 가져옵니다.", total_page);

  //   // for page in 1..total_page {
  //   //   let bills = Bill::fetch_list_by_page(&client, page)?;

  //   //   for bill in bills.list {
  //   //     let mut bill_row = Bill::new(&user.username, &bill);
  //   //     bill_row.save(&dm)?;

  //   //     match client.fetch_bill(&bill_row)? {
  //   //       None => println!(
  //   //         "{}에 대한 세부 페이지가 제공되지 않았습니다.",
  //   //         bill_row.registration_number
  //   //       ),
  //   //       Some(json_result) => {
  //   //         let bill_row = Bill::new(&user.username, &json_result.dtlVo);
  //   //         bill_row.update(&dm)?;
  //   //         Bill::download_file(json_result.dntcFileList, &client, &dm, &fm, &bill_row);
  //   //       }
  //   //     }
  //   //   }
  //   // }

  //   // match fm.upload() {
  //   //   Ok(_) => {
  //   //     println!("Done!");
  //   //   }
  //   //   Err(error) => {
  //   //     panic!("{}", error);
  //   //   }
  //   // }
  // }

  Ok(())
}
