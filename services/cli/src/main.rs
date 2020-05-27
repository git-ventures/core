pub mod analyse;
pub mod init;
pub mod subcommands;

use crate::analyse::Analyse;
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
    let cli = App::new(TITLE)
        .version(crate_version!())
        // .author(crate_authors!())
        .about(ABOUT)
        .subcommand(
            SubCommand::with_name(Command::Init.to_string().as_str())
                .about(CommandDescriptions::from(Command::Init).as_str()),
        )
        .subcommand(
            SubCommand::with_name(Command::Analyse.to_string().as_str())
                .arg(
                    Arg::with_name("commit")
                        .short("c")
                        .long("commit")
                        .takes_value(true)
                        .help("Analyse a specific commit"),
                )
                .arg(
                    Arg::with_name("branch")
                        .short("b")
                        .long("branch")
                        .takes_value(true)
                        .help("Analyse a specific branch"),
                )
                .about(CommandDescriptions::from(Command::Analyse).as_str()),
        )
        .get_matches();

    // Initialize Git Ventures Project
    // Sets up project details
    if let Some(_args) = cli.subcommand_matches(Command::Init.to_string().as_str()) {
        let init = Init::new().description()?.securities_filing_type()?;

        println!("{:?}", init);
    }

    if let Some(args) = cli.subcommand_matches(Command::Analyse.to_string().as_str()) {
        let mut analyse = Analyse::new()?;
        if let Some(commit) = args.value_of("commit") {
            // Analyse a specific commit hash or 'HEAD'
            println!("Commit {:?}", commit);
            analyse.commit(Some(commit))?;
        } else if let Some(branch) = args.value_of("branch") {
            // Analyse a specific commit branch
            analyse.branch(branch)?;
        } else {
            // Find the head of the commit history;
            analyse.commit(None)?;
        }
    }

    Ok(())
}
