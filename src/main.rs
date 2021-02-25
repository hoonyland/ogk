#[macro_use]
extern crate diesel;
extern crate base64;
extern crate clap;
extern crate dotenv;
extern crate toml;

mod client;
mod commands;
mod dm;
mod fm;
mod schema;
mod utils;
mod vars;

// use chrono::prelude::*;
// use chrono::{Duration, Local};
use clap::Clap;
use commands::{Auth, Config, Fetch, Init};
// use diesel::prelude::*;
// use dm::{models, DbManager};
// use fm::FileManager;
use std::error::Error;
// use std::path::Path;
// use utils::date::add_month;
// use utils::users::load_from_csv;

const MAIN_AUTHOR: &str = "Hoony <hoonyland@protonmail>";

#[derive(Clap, Debug)]
enum SubCommand {
    /// features related to authenticate to open.go.kr
    #[clap(version = "0.1", author = MAIN_AUTHOR)]
    Auth(Auth::Opts),
    #[clap(version = "0.1", author = MAIN_AUTHOR)]
    #[clap(version = "0.1", author = MAIN_AUTHOR)]
    Fetch(Fetch::Opts),
    Init(Init::Opts),
    Config(Config::Opts),
}

#[derive(Clap)]
#[clap(version = "0.1", author = MAIN_AUTHOR)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Auth(opts) => {
            let _ = Auth::run(opts).await;
        }
        SubCommand::Config(opts) => {
            let _ = Config::run(opts).await;
        }
        SubCommand::Fetch(opts) => {
            let _ = Fetch::run(opts).await;
        }
        SubCommand::Init(opts) => {
            let _ = Init::run(opts).await;
        } // SubCommand::Run(opts) => {
          //     run::run(opts);
          // }
    }

    Ok(())

    // let matches = App::new("rlp")
    //   .version("0.1")
    //   .author("Hoony Chang <hoony@dotface.kr>")
    //   .about("realopen cli")
    //   .arg(
    //     Arg::with_name("install")
    //       .short('i')
    //       .long("install")
    //       .help("Install realopen for users on users.csv"),
    //   )
    //   .arg(
    //     Arg::with_name("run")
    //       .short('r')
    //       .long("run")
    //       .help("Run daily crawler for users on users.csv"),
    //   )
    //   .arg(
    //     Arg::with_name("from")
    //       .long("from")
    //       .help("Specific date to search with from-date")
    //       .takes_value(true),
    //   )
    //   .subcommand(
    //     App::new("install")
    //       .about("add user & crawl all past bills by the user")
    //       .version("1.0"),
    //   )
    //   // .subcommand(
    //   //     App::new("embago")
    //   //         .about("controls testing features")
    //   //         .version("1.3")
    //   //         .author("Hoony <hoony@dotface.kr>")
    //   //         .arg(Arg::new("month").short('m').about("embago month"))
    //   //         .arg(Arg::new("user").short('u').about("username")),
    //   // )
    //   .get_matches();

    // let db_manager = DbManager::new();

    // let today_datetime: DateTime<Utc> = Utc::now();
    // let fixed_today = today_datetime.with_timezone(&FixedOffset::east(9 * 3600)); // KST +09:00
    // let today = fixed_today.format("%Y-%m-%d").to_string();
    // let fixed_yesterday = fixed_today.checked_sub_signed(Duration::days(1)).unwrap();
    // let yesterday = fixed_yesterday.format("%Y-%m-%d").to_string();
    // let from_date = matches.value_of("from").unwrap_or(&yesterday);

    // match matches.subcommand_name() {
    //     // Some("embago") => {}
    //     Some("install") => install::main(),
    //     Some(_) => panic!("지원하지 않는 명령어입니다."),
    //     None => panic!("명령어를 입력해주세요."),
    // }

    // let users = db_manager.fetch_users_all();

    // if matches.is_present("install") {
    //   for user in &users {
    //     let client = crawler::Client::new(user);
    //     // let mut file_manager = FileManager::new(user);
    //     // let _ = file_manager.clone_remote_repo();

    //     // let bills = models::Bill::fetch_list_by_page(&client, 1)?;
    //     // let total_page = models::Bill::fetch_total_page(&client)?;
    //     // println!("총 {} 페이지를 가져옵니다.", total_page);

    //     let files_result = client.fetch_bill("6153617", "9294619", "9294619")?;
    //     let files_result_txt = files_result.text()?;
    //     let mut start_index = 0;
    //     let mut end_index = 0;
    //     let start_index_result = files_result_txt.find("var result     	= {");
    //     match start_index_result {
    //       None => {}
    //       Some(index) => start_index = index,
    //     }
    //     let end_index_result = files_result_txt.find("}};");
    //     match end_index_result {
    //       None => {}
    //       Some(index) => end_index = index,
    //     }
    //     let files_result_json = &files_result_txt[(start_index + 18)..(end_index + 2)];
    //     let bill_with_files: crawler::BillWithFiles =
    //       serde_json::from_str(files_result_json).unwrap();

    //     println!("{:?}", &bill_with_files.dtlVo);
    //     // let bill_row = models::Bill::new(&user.username, &bill_with_files.dtlVo);
    //   }

    //   //     for page in 1..total_page {
    //   //         let bills = models::Bill::fetch_list_by_page(&client, page)?;

    //   //         for bill in bills.list {
    //   //             let mut bill_row = models::Bill::new(&user.username, &bill);
    //   //             bill_row.set_public_date(&user);
    //   //             bill_row.save(&db_manager)?;

    //   //             let files_result_response = client.fetch_bill(
    //   //                 &bill.rqestRceptNo,
    //   //                 &bill.rqestProcRegstrNo,
    //   //                 &bill.procRegstrNo,
    //   //             )?;
    //   //             let files_result_txt = files_result_response.text()?;
    //   //             // TODO: start_index & end_index 로 파일 여부에 따른 핸들링 리팩토링 필요
    //   //             let mut start_index = 0;
    //   //             let mut end_index = 0;
    //   //             let start_index_result = files_result_txt.find("var result     	= {");
    //   //             match start_index_result {
    //   //                 None => println!("{} has no files", &bill.rqestRceptNo),
    //   //                 Some(index) => start_index = index,
    //   //             }
    //   //             let end_index_result = files_result_txt.find("}};");
    //   //             match end_index_result {
    //   //                 None => println!("{} has no files", &bill.rqestRceptNo),
    //   //                 Some(index) => end_index = index,
    //   //             }
    //   //             let _ = file_manager.mkdir_by_bill(&bill);
    //   //             if start_index > 0 && end_index > 0 {
    //   //                 let files_result_json =
    //   //                     &files_result_txt[(start_index + 18)..(end_index + 2)];
    //   //                 let bill_with_files: crawler::BillWithFiles =
    //   //                     serde_json::from_str(files_result_json).unwrap();

    //   //                 let bill_row = models::Bill::new(&user.username, &bill_with_files.dtlVo);
    //   //                 bill_row.update(&db_manager)?;

    //   //                 match bill_with_files.dntcFileList {
    //   //                     Some(file_list) => {
    //   //                         for file in file_list {
    //   //                             match client.download_file(&file) {
    //   //                                 Ok(downloaded) => {
    //   //                                     let _ = models::File::download(
    //   //                                         &file_manager,
    //   //                                         &db_manager,
    //   //                                         &bill,
    //   //                                         &file,
    //   //                                         &downloaded,
    //   //                                     );
    //   //                                 }
    //   //                                 Err(error) => {
    //   //                                     println!("{:?}", error);
    //   //                                 }
    //   //                             }
    //   //                         }
    //   //                     }
    //   //                     None => {}
    //   //                 }
    //   //             }
    //   //             println!("\"({})\"건을 모두 확인하였습니다.", &bill.rqestSj);
    //   //         }

    //   //         match file_manager.upload() {
    //   //             Ok(_) => {
    //   //                 println!("Done!");
    //   //             }
    //   //             Err(error) => {
    //   //                 panic!("{}", error);
    //   //             }
    //   //         }
    //   //     }
    //   // }
    // }

    // /**
    //  * Daily RUN
    //  */
    // if matches.is_present("run") {
    //   for user in &users {
    //     let bills_update = db_manager.fetch_bills_to_update(user);
    //     let client = crawler::Client::new(user);
    //     let mut file_manager = FileManager::new(user);

    //     let _ = file_manager.clone_remote_repo();
    //     for bill in bills_update {
    //       let files_result_response = client.fetch_bill(
    //         &bill.registration_number,
    //         &bill.request_proc_registration_number,
    //         &bill.proc_registration_number,
    //       )?;

    //       let files_result_txt = files_result_response.text()?;
    //       // TODO: start_index & end_index 로 파일 여부에 따른 핸들링 리팩토링 필요
    //       let mut start_index = 0;
    //       let mut end_index = 0;
    //       let start_index_result = files_result_txt.find("var result     	= {");
    //       match start_index_result {
    //         None => println!("{} has no files", &bill.registration_number),
    //         Some(index) => start_index = index,
    //       }
    //       let end_index_result = files_result_txt.find("}};");
    //       match end_index_result {
    //         None => println!("{} has no files", &bill.registration_number),
    //         Some(index) => end_index = index,
    //       }
    //       let _ = file_manager.mkdir_by_bill_row(&bill);
    //       if start_index > 0 && end_index > 0 {
    //         let files_result_json = &files_result_txt[(start_index + 18)..(end_index + 2)];
    //         let bill_with_files: crawler::BillWithFiles =
    //           serde_json::from_str(files_result_json).unwrap();

    //         let mut bill_row = models::Bill::new(&user.username, &bill_with_files.dtlVo);
    //         bill_row.set_public_date(&user);
    //         let _ = bill_row.save(&db_manager);
    //         let _ = bill_row.update(&db_manager);

    //         match bill_with_files.atchFileList {
    //           Some(file_list) => {
    //             for file in file_list {
    //               match client.download_file(&file) {
    //                 Ok(downloaded) => {
    //                   let _ = models::File::download(
    //                     &file_manager,
    //                     &db_manager,
    //                     &bill_with_files.dtlVo,
    //                     &file,
    //                     &downloaded,
    //                   );
    //                 }
    //                 Err(error) => {
    //                   println!("{:?}", error);
    //                 }
    //               }
    //             }
    //           }
    //           None => {}
    //         }
    //       }
    //       println!("\"({})\"건을 모두 확인하였습니다.", &bill.request_subject);
    //     }

    //     println!("{}", &user.username);
    //     println!("{} ~ {} 청구건을 가져옵니다.", from_date, &today);

    //     let total_page = models::Bill::fetch_total_page_on_specific_date(&client, from_date, &today)?;
    //     println!("총 {} 페이지를 가져옵니다.", total_page);

    //     for page in 1..(total_page + 1) {
    //       let bills =
    //         models::Bill::fetch_list_by_page_on_specific_date(&client, page, from_date, &today)?;

    //       for bill in bills.list {
    //         let mut bill_row = models::Bill::new(&user.username, &bill);
    //         bill_row.set_public_date(&user);
    //         bill_row.save(&db_manager)?;

    //         let files_result_response = client.fetch_bill(
    //           &bill.rqestRceptNo,
    //           &bill.rqestProcRegstrNo,
    //           &bill.procRegstrNo,
    //         )?;
    //         let files_result_txt = files_result_response.text()?;
    //         // TODO: start_index & end_index 로 파일 여부에 따른 핸들링 리팩토링 필요
    //         let mut start_index = 0;
    //         let mut end_index = 0;
    //         let start_index_result = files_result_txt.find("var result     	= {");
    //         match start_index_result {
    //           None => println!("{} has no files", &bill.rqestRceptNo),
    //           Some(index) => start_index = index,
    //         }
    //         let end_index_result = files_result_txt.find("}};");
    //         match end_index_result {
    //           None => println!("{} has no files", &bill.rqestRceptNo),
    //           Some(index) => end_index = index,
    //         }
    //         let _ = file_manager.mkdir_by_bill(&bill);
    //         if start_index > 0 && end_index > 0 {
    //           let files_result_json = &files_result_txt[(start_index + 18)..(end_index + 2)];
    //           let bill_with_files: crawler::BillWithFiles =
    //             serde_json::from_str(files_result_json).unwrap();

    //           let bill_row = models::Bill::new(&user.username, &bill_with_files.dtlVo);
    //           bill_row.update(&db_manager)?;

    //           match bill_with_files.atchFileList {
    //             Some(file_list) => {
    //               for file in file_list {
    //                 match client.download_file(&file) {
    //                   Ok(downloaded) => {
    //                     let _ = models::File::download(
    //                       &file_manager,
    //                       &db_manager,
    //                       &bill,
    //                       &file,
    //                       &downloaded,
    //                     );
    //                   }
    //                   Err(error) => {
    //                     println!("{:?}", error);
    //                   }
    //                 }
    //               }
    //             }
    //             None => {}
    //           }
    //         }
    //         println!("\"({})\"건을 모두 확인하였습니다.", &bill.rqestSj);
    //       }
    //     }

    //     match file_manager.upload() {
    //       Ok(_) => {
    //         println!("Done!");
    //       }
    //       Err(error) => {
    //         panic!("{}", error);
    //       }
    //     }
    //   }
    // }

    // Ok(())
}
