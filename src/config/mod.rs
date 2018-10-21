use dirs;
use errors::AppError;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;
use toml;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
  pub jira_domain: String,
  pub jira_user: String,
  pub jira_token: String,
}

fn config_path() -> Result<PathBuf, AppError> {
  match dirs::home_dir() {
    Some(home_dir) => Ok(PathBuf::from(format!("{}/.fbjira.toml", home_dir.display()))),
    None => Err(AppError::RuntimeError("Cannot read home directory".to_string())),
  }
}

pub fn load() -> Result<Config, AppError> {
  let path: PathBuf = config_path()?;

  let mut file: File = File::open(path)?;
  let mut contents = String::new();

  file.read_to_string(&mut contents)?;

  toml::from_str(&contents).map_err(AppError::TOML)
}

pub fn write(config: &Config) -> Result<(), AppError> {
  let path: PathBuf = config_path()?;

  let config_as_toml: String = toml::to_string(&config)?;

  if !path.exists() {
    let mut file = File::create(&path)?;
    file.write_all(config_as_toml.as_bytes()).map_err(AppError::IO)
  } else {
    let mut options = OpenOptions::new();
    options.write(true);

    let file = options.open(&path)?;

    let mut writer = BufWriter::new(&file);
    writer.write_all(config_as_toml.as_bytes()).map_err(AppError::IO)
  }
}

pub fn init() -> Result<(), AppError> {
  let path: PathBuf = config_path()?;

  if path.exists() {
    Err(AppError::RuntimeError("The config file already exists!".to_string()))
  } else {
    let config: Config = Config {
      jira_domain: "<your-jira-domain>".to_string(),
      jira_user: "<your-jira-user>".to_string(),
      jira_token: "<your-jira-token>".to_string(),
    };

    write(&config)
  }
}
