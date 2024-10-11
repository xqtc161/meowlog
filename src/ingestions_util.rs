use crate::config::INGESTIONS_FILE;
use crate::ingestions::{DoseUnit, Ingestion, IngestionMethod};
use crate::util::path_exists;
use chrono::Utc;
use inquire;
use std::{collections::HashMap, process::exit};
use strum::IntoEnumIterator;
use uuid::Uuid;

pub fn ensure_ingestion_files() -> HashMap<Uuid, Ingestion> {
    let ingesstions_bytes_loaded_des: HashMap<Uuid, Ingestion>;
    if path_exists(INGESTIONS_FILE.to_string()) {
        let substances_bytes_loaded = std::fs::read(INGESTIONS_FILE.to_string()).unwrap();
        ingesstions_bytes_loaded_des = bincode::deserialize(&substances_bytes_loaded).unwrap();
    } else {
        std::fs::File::create(INGESTIONS_FILE.to_string()).unwrap();
        ingesstions_bytes_loaded_des = HashMap::new();
        let ingesstions_bytes_loaded_ser =
            bincode::serialize(&ingesstions_bytes_loaded_des).unwrap();
        std::fs::write(INGESTIONS_FILE.to_string(), ingesstions_bytes_loaded_ser).unwrap();
    }
    ingesstions_bytes_loaded_des
}

pub fn get_user_datetime() -> chrono::NaiveDateTime {
    let current_time = Utc::now().naive_utc();
    let date_time: chrono::NaiveDateTime = inquire::CustomType::<chrono::NaiveDateTime>::new(
        "Enter the date and time (YYYY-MM-DD HH:MM):",
    )
    .with_placeholder("YYYY-MM-DD HH:MM")
    .with_default(current_time)
    .with_parser(&|input| {
        chrono::NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M").map_err(|_| ())
    })
    .with_error_message("Please enter a valid date and time in the format YYYY-MM-DD HH:MM.")
    .with_help_message("Use the format YYYY-MM-DD HH:MM")
    .prompt()
    .unwrap();
    date_time
}

pub fn get_dose_unit() -> DoseUnit {
    let dose_unit = inquire::Select::new(
        "What unit should be used?",
        DoseUnit::iter().collect::<Vec<_>>(),
    )
    .prompt()
    .unwrap();
    dose_unit
}

pub fn get_substance() -> String {
    let substances = crate::substance_util::substances_to_vec();
    if substances.is_empty() {
        eprintln!("Add a substance before you log an ingestions");
        exit(1)
    }
    let substance = inquire::Select::new("What did yout ingest?", substances)
        .prompt()
        .unwrap();
    substance
}

pub fn get_ingestion_method() -> IngestionMethod {
    let ingestion_method = inquire::Select::new(
        "How did you ingest?",
        IngestionMethod::iter().collect::<Vec<_>>(),
    )
    .prompt()
    .unwrap();
    ingestion_method
}

pub fn get_ingestion_confirmation(ingestion: Ingestion) -> bool {
    println!(
        "Substance:  {} ({})\nDose:       {}{}\nTime:       {}\n",
        ingestion.substance,
        ingestion.ingestion_method,
        ingestion.dose.value,
        ingestion.dose.unit,
        ingestion.time,
    );
    let confirm =
        inquire::prompt_confirmation("Does the ingestion above look alright? [y/N]").unwrap();
    confirm
}
