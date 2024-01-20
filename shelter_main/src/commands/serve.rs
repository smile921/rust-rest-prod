use clap::{ArgMatches, Command};
use crate::settings::Settings;

pub fn configure() -> Command {
    Command::new("serve").about("satrt the server!")
}

pub fn handle(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("serve") {
        println!("Server is up!");
    }
    Ok(())
}