mod commands;
mod data_source;

use clap::{Parser, Subcommand};
use log::debug;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all organisations
    ListOrganisations,
}

fn main() {
    pretty_env_logger::try_init_custom_env("ACTINU_LOG").expect("Could not instantiate logger!");

    debug!("Starting Actinu...");

    let args = Cli::parse();

    debug!("Found arguments: {:?}", &args);

    match &args.command {
        Commands::ListOrganisations => commands::list_organisations(),
    }
}
