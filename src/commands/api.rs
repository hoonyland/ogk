use crate::crawler;
use crate::dm::{models, DbManager};
use crate::fm::FileManager;
use crate::utils::users;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use chrono::{Duration, Local};
use clap::Clap;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use futures::future::{ok, Future};
use std::env;
use std::error::Error;
use std::path::Path;

use crate::dm::scheme::bills::dsl as bills_dsl;

#[derive(Clap)]
pub struct Opts {}

#[derive(serde::Serialize)]
struct Test;

#[derive(serde::Serialize)]
struct RowsResponse<T> {
  rows: Vec<T>,
  total_count: u32,
}

#[derive(serde::Deserialize)]
struct RowsQuery {
  page: Option<u32>,
}

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[get("/")]
async fn get_bills() -> Result<HttpResponse, Error> {
  let conn = pool.get().expect("couldn't get db connection from pool");
  let bills = bills_dsl::bills.load::<models::Bill>(&conn)?;
  Ok(bills)
  // let bills = bills_dsl::bills.load::<models::Bill>(&conn);

  // let page: u32 = match query.page {
  //   Some(p) => p,
  //   None => 1,
  // };

  // let bills = dm.fetch_bills_all();

  // let response = RowsResponse {
  //   rows: bills,
  //   total_count: page,
  // };

  // ok(HttpResponse::Ok().json(response))
}

#[actix_web::main]
pub async fn run(opts: Opts) -> Result<(), Box<dyn Error>> {
  dotenv().ok();

  let connspec = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let conn = ConnectionManager::<MysqlConnection>::new(connspec);
  let pool = r2d2::Pool::builder().max_size(10).build(conn).unwrap();

  let PORT = match env::var("API_PORT") {
    Ok(p) => p,
    Err(_) => String::from("8080"),
  };

  println!("Running on {} ...", format!("127.0.0.1:{}", PORT));
  HttpServer::new(|| {
    App::new()
      .data(pool.clone())
      .service(web::scope("/bills").service(get_bills))
  })
  .bind(format!("127.0.0.1:{}", PORT))?
  .run()
  .await?;

  Ok(())
}
