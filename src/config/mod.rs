use errors::AppError;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;
use toml;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
  pub jira_host: String,
  pub jira_user: String,
  pub jira_token: String,
}

fn config_path() -> PathBuf {
  PathBuf::from(format!("{}/.fbjira.toml", env::home_dir().expect("Could not get homedir.").display()))
}

pub fn load() -> Result<Config, AppError> {
  let path: PathBuf = config_path();

  let mut file: File = File::open(path).expect("File must exist");
  let mut contents = String::new();

  file.read_to_string(&mut contents).expect("Unable to read file");

  toml::from_str(&contents).map_err(AppError::TOML)
}

pub fn write(config: &Config) -> Result<(), AppError> {
  let path: PathBuf = config_path();

  let config_as_toml: String = toml::to_string(&config).expect("Cannot marshal config to toml");

  if !path.exists() {
    let mut file = File::create(&path).expect("Creation of file failed.");
    file.write_all(config_as_toml.as_bytes()).map_err(AppError::IO)
  } else {
    let mut options = OpenOptions::new();
    options.write(true);

    let file = options.open(&path).expect("Cannot open file");

    let mut writer = BufWriter::new(&file);
    writer.write_all(config_as_toml.as_bytes()).map_err(AppError::IO)
  }
}

pub fn init() -> Result<(), AppError> {
  let path: PathBuf = config_path();

  if path.exists() {
    Err(AppError::RuntimeError("The config file already exists!".to_string()))
  } else {
    let config: Config = Config {
      jira_host: "<your-jira-host>".to_string(),
      jira_user: "<your-jira-user>".to_string(),
      jira_token: "<your-jira-token>".to_string(),
    };

    write(&config)
  }
}
