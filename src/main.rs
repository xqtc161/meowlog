use core::panic;
use std::{collections::HashMap, path::Path};

use bincode::serialize;
use chrono::Utc;
use clap::{Parser, Subcommand};
use config::{INGESTIONS_FILE, LOCAL_PATH, SUBSTANCES_FILE};
use git2;
use inquire;
use serde::{self, Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};
use uuid::Uuid;

mod config;

mod ingestions;
mod substances;

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
    // let home = std::env::var("HOME").unwrap();
    // let local_path = format!("{}/.local/share/meowlog", &home);
    if !substances::path_exists(LOCAL_PATH.to_string()) {
        match std::fs::create_dir(LOCAL_PATH.to_string()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Could not create data directory with error: {:?}", e);
                std::process::exit(1);
            }
        }
    }
    if !substances::path_exists(SUBSTANCES_FILE.to_string()) {
        match substances::create_substances_file() {
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
    if !substances::path_exists(INGESTIONS_FILE.to_string()) {
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
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::AddIngestion) => ingestions::add_ingestion(),
        Some(Commands::EditIngestion) => {}
        Some(Commands::ListIngestions) => ingestions::list_ingestions().unwrap(),
        Some(Commands::RemoveIngestion) => {}
        Some(Commands::AddSubstance) => substances::add_substance().unwrap(),
        Some(Commands::EditSubstance) => {}
        Some(Commands::ListSubstances) => substances::list_substances().unwrap(),
        Some(Commands::RemoveSubstance) => {}
        None => {}
    }
}
