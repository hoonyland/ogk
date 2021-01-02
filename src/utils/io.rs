use std::io::{self, Write};
use std::error::Error;

pub fn stdout(output: &[u8]) -> Result<(), io::Error> {
  let stdout = io::stdout();
  let mut handle = stdout.lock();
  handle.write_all(output)?;
  Ok(())
}
