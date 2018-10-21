use colored::*;
use config::Config;
use errors::AppError;
use goji::{Credentials, Jira, SearchOptionsBuilder};

pub fn jira_client(config: Config) -> Result<Jira, AppError> {
  match Jira::new(config.jira_domain, Credentials::Basic(config.jira_user, config.jira_token)) {
    Ok(jira) => Ok(jira),
    Err(_) => Err(AppError::RuntimeError("Cannot create jira client".to_string())),
  }
}

pub fn list_open(jira: &Jira, project: &str) -> Vec<String> {
  let search_options = SearchOptionsBuilder::new().max_results(1000).build();

  let results = jira
    .search()
    .iter(format!("project = {} AND status != Done ORDER BY id DESC", project), &search_options);

  match results {
    Ok(results) => results.map(|issue| format!("{} {}", issue.key, issue.summary().unwrap_or_default())).collect(),
    Err(_) => vec![],
  }
}

pub fn list_all(jira: &Jira, project: &str) -> Vec<String> {
  let search_options = SearchOptionsBuilder::new().max_results(1000).build();

  let results = jira.search().iter(format!("project = {} ORDER BY id DESC", project), &search_options);

  match results {
    Ok(results) => results.map(|issue| format!("{} {}", issue.key, issue.summary().unwrap_or_default())).collect(),
    Err(_) => vec![],
  }
}

pub fn summary(jira: &Jira, issue_name: &str) -> String {
  let issue = jira.issues().get(issue_name);

  match issue {
    Ok(issue) => format!(
      "{}\n\n{}",
      &issue.summary().unwrap_or_default().bold().on_black().white(),
      issue.description().unwrap_or_default(),
    ),
    Err(_) => format!("Cannot load summary for issue {}", issue_name),
  }
}
