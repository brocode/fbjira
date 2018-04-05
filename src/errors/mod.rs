use goji;
use std::error::Error;
use std::fmt;
use std::io;
use toml;

#[derive(Debug)]
pub enum AppError {
  IO(io::Error),
  TOML(toml::de::Error),
  JIRA(goji::Error),
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      AppError::IO(ref err) => write!(f, "IO error: {}", err),
      AppError::TOML(ref err) => write!(f, "TOML error: {}", err),
      AppError::JIRA(ref err) => write!(f, "JIRA error: {}", err),
    }
  }
}

impl Error for AppError {
  fn description(&self) -> &str {
    match *self {
      AppError::IO(ref err) => err.description(),
      AppError::TOML(ref err) => err.description(),
      AppError::JIRA(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      AppError::IO(ref err) => Some(err),
      AppError::TOML(ref err) => Some(err),
      AppError::JIRA(ref err) => Some(err),
    }
  }
}

impl From<io::Error> for AppError {
  fn from(err: io::Error) -> AppError {
    AppError::IO(err)
  }
}

impl From<toml::de::Error> for AppError {
  fn from(err: toml::de::Error) -> AppError {
    AppError::TOML(err)
  }
}

impl From<goji::Error> for AppError {
  fn from(err: goji::Error) -> AppError {
    AppError::JIRA(err)
  }
}