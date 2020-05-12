pub mod subcommands;

use crate::subcommands::*;
use clap::{App, SubCommand, crate_authors, crate_version};

// TODO: Pull from Cargo.toml
const TITLE: &str = "Git Ventures CLI";
const ABOUT: &str = "
Your next start-up should be as easy as `git init`.

Git Ventures is a set of software tools to help you 
turn your git repository into your next investable 
start-up.
";

fn main() {
    let cli = App::new(TITLE)
        .version(crate_version!())
        .author(crate_authors!())
        .about(ABOUT)
        .subcommand(
            SubCommand::with_name(Command::Init.to_string().as_str())
                .about(CommandDescriptions::from(Command::Init).as_str()),
        )
        .get_matches();

    if let Some(args) = cli.subcommand_matches(Command::Init.to_string().as_str()) {
        println!("Args: {:?}", args);
    }
}
