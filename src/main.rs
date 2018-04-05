#[macro_use]
extern crate clap;

extern crate goji;

#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate colored;

use clap::{App, AppSettings, Arg, SubCommand};
use config::Config;
use goji::Jira;

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
          SubCommand::with_name("list-all")
            .about("List all issues !!! Takes a really long time !!!")
            .arg(Arg::with_name("PROJECT").required(true)),
        )
        .subcommand(
          SubCommand::with_name("summary")
            .about("Get a summary for a given issue key")
            .arg(Arg::with_name("ISSUE").required(true)),
        ),
    )
    .get_matches();

  let jira: Jira = jira::jira_client(config);

  if let Some(matches) = matches.subcommand_matches("issue") {
    if let Some(matches) = matches.subcommand_matches("list-open") {
      let project = matches.value_of("PROJECT").unwrap().to_string();

      for issue_with_summary in jira::list_open(&jira, &project) {
        println!("{}", issue_with_summary)
      }
    }

    if let Some(matches) = matches.subcommand_matches("list-all") {
      let project = matches.value_of("PROJECT").unwrap().to_string();

      for issue_with_summary in jira::list_all(&jira, &project) {
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
