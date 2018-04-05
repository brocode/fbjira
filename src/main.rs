#[macro_use]
extern crate clap;

extern crate goji;

#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate colored;

use clap::{App, AppSettings, Arg, SubCommand};
use colored::*;
use config::Config;
use goji::{Credentials, Issue, Jira, SearchOptionsBuilder};

fn main() {
  let config: Config = config::load().expect("Could not load config.");

  let matches = App::new(crate_name!())
    .version(crate_version!())
    .about(crate_description!())
    .author(crate_authors!())
    .global_setting(AppSettings::ColoredHelp)
    .setting(AppSettings::SubcommandRequired)
    .subcommand(
      SubCommand::with_name("issue")
        .setting(AppSettings::SubcommandRequired)
        .about("Work with issues (List open issues, get summary for a specific issue...)")
        .subcommand(
          SubCommand::with_name("list-open")
            .about("List all open issues")
            .arg(Arg::with_name("PROJECT").required(true)),
        )
        .subcommand(
          SubCommand::with_name("summary")
            .about("Get a summary for a given issue key")
            .arg(Arg::with_name("ISSUE").required(true)),
        ),
    )
    .get_matches();

  let jira: Jira = Jira::new(config.jira_host, Credentials::Basic(config.jira_user, config.jira_token)).expect("Setup of JIRA client failed");

  if let Some(matches) = matches.subcommand_matches("issue") {
    if let Some(matches) = matches.subcommand_matches("list-open") {
      let search_options = SearchOptionsBuilder::new().max_results(1000).build();

      let results = jira
        .search()
        .iter(
          format!("project = {} AND status != Done ORDER BY key DESC", matches.value_of("PROJECT").unwrap()),
          &search_options,
        )
        .expect("Call to JIRA did not work");

      let issues: Vec<(String, Option<String>)> = results.into_iter().map(|issue| (issue.key.to_owned(), issue.summary())).collect();

      for (key, summary) in issues {
        println!("{} {}", key, summary.unwrap_or_default());
      }
    }

    if let Some(matches) = matches.subcommand_matches("summary") {
      let issue: Issue = jira.issues().get(matches.value_of("ISSUE").unwrap()).expect("Retrieving of issue did not work");

      println!(
        "{}\n\n{}",
        &issue.summary().unwrap_or_default().bold().on_black().white(),
        issue.description().unwrap_or_default(),
      );
    }
  }
}

mod config;
mod errors;
