pub mod analyze;
pub mod contributor;
pub mod init;
pub mod stats;
pub mod subcommands;

use crate::analyze::Analyze;
use crate::init::Init;
use crate::subcommands::*;
use anyhow::Error;
use clap::{crate_authors, crate_version, App, Arg, SubCommand};

// TODO: Pull from Cargo.toml
const TITLE: &str = "Git Ventures CLI";
const ABOUT: &str = "
Your next start-up should be as easy as `git init`.

Git Ventures is a set of software tools to help you 
turn your git repository into your next investable 
start-up.
";

fn main() -> Result<(), Error> {
    // Initialize Logging
    env_logger::init();

    let cli = App::new(TITLE)
        .version(crate_version!())
        // .author(crate_authors!())
        .about(ABOUT)
        .subcommand(
            SubCommand::with_name(Command::Init.to_string().as_str())
                .about(CommandDescriptions::from(Command::Init).as_str()),
        )
        .subcommand(
            SubCommand::with_name(Command::Analyze.to_string().as_str())
                .arg(
                    Arg::with_name("commit")
                        .short("c")
                        .long("commit")
                        .takes_value(true)
                        .help("Analyze a specific commit"),
                )
                .about(CommandDescriptions::from(Command::Analyze).as_str()),
        )
        .get_matches();

    // Initialize Git Ventures Project
    // Sets up project details
    if let Some(_args) = cli.subcommand_matches(Command::Init.to_string().as_str()) {
        let init = Init::new().description()?.securities_filing_type()?;

        println!("{:?}", init);
    }

    if let Some(args) = cli.subcommand_matches(Command::Analyze.to_string().as_str()) {
        let mut analyze = Analyze::new()?;
        match args.value_of("commit") {
            Some(oid) => analyze.commit(Some(oid))?,
            None => analyze.commit(None)?,
        }
    }

    Ok(())
}
