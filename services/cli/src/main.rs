pub mod init;
pub mod subcommands;

use crate::init::Init;
use crate::subcommands::*;
use anyhow::Error;
use clap::{crate_authors, crate_version, App, SubCommand};

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
        .author(crate_authors!())
        .about(ABOUT)
        .subcommand(
            SubCommand::with_name(Command::Init.to_string().as_str())
                .about(CommandDescriptions::from(Command::Init).as_str()),
        )
        .get_matches();

    if let Some(_args) = cli.subcommand_matches(Command::Init.to_string().as_str()) {
        let init = Init::new().description()?.securities_filing_type()?;

        println!("{:?}", init);
    }

    Ok(())
}
