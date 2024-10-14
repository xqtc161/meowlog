use core::panic;

use clap::{Command, Parser, Subcommand};
use clap_complete::aot::{generate, Generator, Shell};
use lazy_static::lazy_static;
use std::io;

lazy_static! {
    pub static ref HOME: String = std::env::var("HOME").unwrap();
    pub static ref LOCAL_PATH: String = format!("{}/.local/share/meowlog", HOME.to_string());
    pub static ref SUBSTANCES_FILE: String =
        format!("{}/substances.bin", LOCAL_PATH.to_string()).to_string();
    pub static ref INGESTIONS_FILE: String =
        format!("{}/ingestions.bin", LOCAL_PATH.to_string()).to_string();
}
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

    /// Generate shell completions
    GenerateCompletions { shell: String },
}

use clap::CommandFactory;
use std::str::FromStr;
fn build_cli_command() -> Command {
    Cli::command()
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
    ensure_files();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::AddIngestion) => ingestions::add_ingestion(),
        Some(Commands::EditIngestion) => ingestions::edit_ingestion().unwrap(),
        Some(Commands::ListIngestions) => ingestions::list_ingestions().unwrap(),
        Some(Commands::RemoveIngestion) => {}
        Some(Commands::AddSubstance) => substances::add_substance().unwrap(),
        Some(Commands::EditSubstance) => substances::edit_substance().unwrap(),
        Some(Commands::ListSubstances) => substances::list_substances().unwrap(),
        Some(Commands::RemoveSubstance) => substances::remove_substance().unwrap(),
        Some(Commands::GenerateCompletions { shell }) => {
            let mut cmd = Cli::command();
            eprintln!("Generating completion file for {shell}...");
            if matches!(shell.as_str(), "nu" | "nushell") {
                print_completions(clap_complete_nushell::Nushell, &mut cmd);
            } else if let Ok(shell) = Shell::from_str(shell.as_str()) {
                print_completions(shell, &mut cmd);
            } else {
                return eprintln!("Shell not recognized!");
            };
        }

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
