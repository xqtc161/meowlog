use core::panic;

use clap::{Parser, Subcommand};
use config::{INGESTIONS_FILE, LOCAL_PATH, SUBSTANCES_FILE};

mod config;
mod util;

mod ingestions;
mod ingestions_util;
mod substance_util;
mod substances;

// mod drug_parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds ingestion
    AddIngestion,

    /// Edits an ingestion
    EditIngestion,

    /// List ingestions
    ListIngestions,

    /// Remove ingestion
    RemoveIngestion,

    /// Adds substance
    AddSubstance,

    /// Edits an substance
    EditSubstance,

    /// List substances
    ListSubstances,

    /// Remove substance
    RemoveSubstance,
}

fn main() {
    ensure_files();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::AddIngestion) => ingestions::add_ingestion(),
        Some(Commands::EditIngestion) => {ingestions::edit_ingestion().unwrap()}
        Some(Commands::ListIngestions) => ingestions::list_ingestions().unwrap(),
        Some(Commands::RemoveIngestion) => {}
        Some(Commands::AddSubstance) => substances::add_substance().unwrap(),
        Some(Commands::EditSubstance) => substances::edit_substance().unwrap(),
        Some(Commands::ListSubstances) => substances::list_substances().unwrap(),
        Some(Commands::RemoveSubstance) => substances::remove_substance().unwrap(),
        None => {}
    }
}

fn ensure_files() {
    if !util::path_exists(LOCAL_PATH.to_string()) {
        match std::fs::create_dir(LOCAL_PATH.to_string()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Could not create data directory with error: {:?}", e);
                std::process::exit(1);
            }
        }
    }
    if !util::path_exists(SUBSTANCES_FILE.to_string()) {
        match substance_util::create_substances_file() {
            Ok(_) => {
                println!(
                    "Created substances file at {:?}",
                    SUBSTANCES_FILE.to_string()
                )
            }
            Err(_) => {
                eprintln!("Could not create substances file");
                panic!()
            }
        };
    }
    if !util::path_exists(INGESTIONS_FILE.to_string()) {
        match ingestions::create_ingestions_file() {
            Ok(_) => {
                println!(
                    "Created ingestions file at {:?}",
                    INGESTIONS_FILE.to_string()
                )
            }
            Err(_) => {
                eprintln!("Could not create substances file");
                panic!()
            }
        };
    }
}
