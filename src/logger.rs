use crate::util::get_file;
use lazy_static::lazy_static;
use std::{collections::HashSet, io::Write, sync::Mutex};

lazy_static! {
  static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new().unwrap());
}

pub struct Logger {
  file: std::fs::File,
  logged_messages: HashSet<String>,
}

impl Logger {
  fn new() -> Result<Self, std::io::Error> {
    let file = get_file("log.txt", "");
    Ok(Logger {
      file,
      logged_messages: HashSet::new(),
    })
  }

  pub fn log(message: &str) {
    let mut logger = LOGGER.lock().unwrap();

    if logger.logged_messages.contains(message) {
      return; // Don't log the same message again
    }

    let formatted_message = format!("{}\n", message);
    logger
      .file
      .write_all(formatted_message.as_bytes())
      .expect("Failed to write to log");
    logger.file.flush().expect("Failed to flush log");

    logger.logged_messages.insert(message.to_string());
  }
}