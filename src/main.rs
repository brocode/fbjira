#[macro_use]
extern crate clap;

extern crate goji;

#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate colored;

extern crate dirs;

use clap::{App, AppSettings, Arg, SubCommand};
use config::Config;
use goji::Jira;

fn main() {
  let matches = App::new(crate_name!())
    .version(crate_version!())
    .about(crate_description!())
    .author(crate_authors!())
    .global_setting(AppSettings::ColoredHelp)
    .setting(AppSettings::SubcommandRequired)
    .subcommand(
      SubCommand::with_name("config")
        .setting(AppSettings::SubcommandRequired)
        .about("All about managing your fbjira config")
        .subcommand(SubCommand::with_name("init").about("Creates an empty config file in your home directory")),
    )
    .subcommand(
      SubCommand::with_name("issue")
        .setting(AppSettings::SubcommandRequired)
        .about("Work with issues (List open issues, get summary for a specific issue...)")
        .subcommand(
          SubCommand::with_name("list-open")
            .about("List all open issues")
            .arg(Arg::with_name("PROJECT").multiple(true).required(true)),
        )
        .subcommand(
          SubCommand::with_name("list-all")
            .about("List all issues !!! Takes a really long time !!!")
            .arg(Arg::with_name("PROJECT").multiple(true).required(true)),
        )
        .subcommand(
          SubCommand::with_name("summary")
            .about("Get a summary for a given issue key")
            .arg(Arg::with_name("ISSUE").required(true)),
        ),
    )
    .get_matches();

  if let Some(matches) = matches.subcommand_matches("config") {
    if matches.subcommand_matches("init").is_some() {
      match config::init() {
        Err(e) => println!("{}", e),
        Ok(_) => println!("Config file successfully created"),
      }
    }
  }

  if let Some(matches) = matches.subcommand_matches("issue") {
    let config: Config = config::load().expect("Could not load config.");
    let jira: Jira = jira::jira_client(config).unwrap();

    if let Some(matches) = matches.subcommand_matches("list-open") {
      let issues_with_summary: Vec<String> = matches
        .values_of("PROJECT")
        .into_iter()
        .flat_map(|project| project)
        .flat_map(|project| jira::list_open(&jira, &project))
        .collect();

      for issue_with_summary in issues_with_summary {
        println!("{}", issue_with_summary)
      }
    }

    if let Some(matches) = matches.subcommand_matches("list-all") {
      let issues_with_summary: Vec<String> = matches
        .values_of("PROJECT")
        .into_iter()
        .flat_map(|project| project)
        .flat_map(|project| jira::list_all(&jira, &project))
        .collect();

      for issue_with_summary in issues_with_summary {
        println!("{}", issue_with_summary)
      }
    }

    if let Some(matches) = matches.subcommand_matches("summary") {
      let issue = matches.value_of("ISSUE").unwrap().to_string();

      println!("{}", jira::summary(&jira, &issue))
    }
  }
}

mod config;
mod errors;
mod jira;
