use crate::dm::models;
use std::error::Error;
use std::path::Path;

pub fn load_from_csv(path: &Path) -> Result<Vec<models::User>, Box<dyn Error>> {
  let mut rdr = csv::Reader::from_path(path).unwrap();
  let mut users = vec![];
  for result in rdr.deserialize() {
    let user: models::User = result?;
    users.push(user);
  }
  Ok(users)
}
