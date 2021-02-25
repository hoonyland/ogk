pub mod models;

extern crate diesel;

use crate::client::{DntcFile, DtlVo};
use bcrypt::{hash, DEFAULT_COST};
use diesel::dsl::not;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::{insert_into, update};
use dotenv::dotenv;
use std::env;
use std::error::Error;

pub struct DbManager {
    pub conn: MysqlConnection,
}

impl DbManager {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = MysqlConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

        Self { conn: conn }
    }

    // pub fn insert_bill(&self, row: &models::Bill) -> QueryResult<usize> {
    //     insert_into(bills_dsl::bills)
    //         .values(row)
    //         .execute(&self.conn)
    // }

    // pub fn update_bill(&self, row: &models::Bill) -> QueryResult<usize> {
    //     update(bills_dsl::bills)
    //         .set((
    //             bills_dsl::group_id.eq(&row.group_id),
    //             bills_dsl::proc_registration_number.eq(&row.proc_registration_number),
    //             bills_dsl::result_description.eq(&row.result_description),
    //             bills_dsl::request_description.eq(&row.request_description),
    //             bills_dsl::status.eq(&row.status),
    //         ))
    //         .filter(bills_dsl::id.eq(&row.id))
    //         .execute(&self.conn)
    // }

    // pub fn insert_bills(&self, rows: &Vec<models::Bill>) -> QueryResult<usize> {
    //     insert_into(bills_dsl::bills)
    //         .values(rows)
    //         .execute(&self.conn)
    // }

    // pub fn insert_file(&self, row: &models::File) -> QueryResult<usize> {
    //     insert_into(files_dsl::files)
    //         .values(row)
    //         .execute(&self.conn)
    // }

    // // pub fn insert_user(&self, mut row: models::User) -> Result<(), bcrypt::BcryptError> {
    // pub fn insert_user(&self, mut row: models::User) -> Result<(), Box<dyn Error>> {
    //     let hashed = match hash(&row.password, DEFAULT_COST) {
    //         Ok(h) => {
    //             row.password = h;
    //             insert_into(users_dsl::users)
    //                 .values(row)
    //                 .execute(&self.conn);
    //             Ok(())
    //         }
    //         Err(e) => Err(e),
    //     };

    //     Ok(())
    // }

    // // TODO: Query 범용적으로 만들 필요
    // pub fn fetch_bills_to_update(&self, user: &models::User) -> Vec<models::Bill> {
    //     bills_dsl::bills
    //         .filter(not(bills_dsl::status.eq("통지완료")))
    //         .filter(not(bills_dsl::status.eq("공개완료")))
    //         .filter(bills_dsl::user_id.eq(&user.username))
    //         .load::<models::Bill>(&self.conn)
    //         .expect("Error loading bills")
    // }

    // pub fn fetch_bills_all(&self) -> Vec<models::Bill> {
    //     bills_dsl::bills
    //         .load::<models::Bill>(&self.conn)
    //         .expect("Error loading bills")
    // }

    // pub fn fetch_users_all(&self) -> Vec<models::User> {
    //     users_dsl::users
    //         .load::<models::User>(&self.conn)
    //         .expect("Error loading users")
    // }
}
