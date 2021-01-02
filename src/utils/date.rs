use chrono::prelude::*;
use std::convert::From;

pub struct KstDateTime {
  pub datetime: DateTime<FixedOffset>,
}

impl KstDateTime {
  pub fn format(&self, format: Option<&str>) -> String {
    let _format = match format {
      Some(f) => f,
      None => "%Y-%m-%d",
    };
    self.datetime.format(_format).to_string()
  }
}

impl From<DateTime<Utc>> for KstDateTime {
  fn from(datetime: DateTime<Utc>) -> KstDateTime {
    KstDateTime {
      datetime: datetime.with_timezone(&FixedOffset::east(9 * 3600)), // KST +09:00
    }
  }
}

// pub fn add_month(datetime: DateTime<Utc>, month: i32) -> DateTime<Utc> {
//   let current_year = datetime.year();
//   let current_month = datetime.month() as i32;
//   let current_day = datetime.day();

//   let (return_year, return_month): (i32, i32) = match current_month + month {
//     1..=12 => (current_year, current_month + month),
//     _ => (current_year + 1, current_month + month - 12),
//   };

//   Utc
//     .ymd(return_year, return_month as u32, current_day)
//     .and_hms(0, 0, 0)
// }
