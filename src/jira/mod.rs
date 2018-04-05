use colored::*;
use config::Config;
use goji::{Credentials, Issue, Jira, SearchOptionsBuilder};

pub fn jira_client(config: Config) -> Jira {
  Jira::new(config.jira_host, Credentials::Basic(config.jira_user, config.jira_token)).expect("Setup of JIRA client failed")
}

pub fn list_open(jira: &Jira, project: &str) -> Vec<String> {
  let search_options = SearchOptionsBuilder::new().max_results(1000).build();

  let results = jira
    .search()
    .iter(format!("project = {} AND status != Done ORDER BY id DESC", project), &search_options)
    .expect("Call to JIRA did not work");

  results
    .into_iter()
    .map(|issue| format!("{} {}", issue.key, issue.summary().unwrap_or_default()))
    .collect()
}

pub fn list_all(jira: &Jira, project: &str) -> Vec<String> {
  let search_options = SearchOptionsBuilder::new().max_results(1000).build();

  let results = jira
    .search()
    .iter(format!("project = {} ORDER BY id DESC", project), &search_options)
    .expect("Call to JIRA did not work");

  results
    .into_iter()
    .map(|issue| format!("{} {}", issue.key, issue.summary().unwrap_or_default()))
    .collect()
}

pub fn summary(jira: &Jira, issue: &str) -> String {
  let issue: Issue = jira.issues().get(issue).expect("Retrieving of issue did not work");

  format!(
    "{}\n\n{}",
    &issue.summary().unwrap_or_default().bold().on_black().white(),
    issue.description().unwrap_or_default(),
  )
}
