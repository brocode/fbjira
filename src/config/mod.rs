use errors::AppError;
use std::env;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
  pub jira_host: String,
  pub jira_user: String,
  pub jira_token: String,
}

pub fn load() -> Result<Config, AppError> {
  let path: String = format!("{}/.fbjira.toml", env::home_dir().expect("Could not get homedir.").display());

  let mut file: File = File::open(path).expect("File must exist");
  let mut contents = String::new();

  file.read_to_string(&mut contents).expect("Unable to read file");

  toml::from_str(&contents).map_err(AppError::TOML)
}
