use crate::ingestions::{DoseUnit, Ingestion, IngestionMethod};
use crate::substances::Substance;
use crate::util::path_exists;
use crate::INGESTIONS_FILE;
use chrono::NaiveDateTime;
use inquire;
use std::{collections::HashMap, process::exit};
use strum::IntoEnumIterator;
use uuid::Uuid;

pub fn ensure_ingestion_files() -> HashMap<Uuid, Ingestion> {
    let ingestions_bytes_loaded_des: HashMap<Uuid, Ingestion>;
    if path_exists(INGESTIONS_FILE.to_string()) {
        let substances_bytes_loaded =
            std::fs::read(INGESTIONS_FILE.to_string()).expect("Could not read ingestions file");
        ingestions_bytes_loaded_des = bincode::deserialize(&substances_bytes_loaded).expect("Could not deserialize ingestions file. If you are tech-savvy try fixing it with a hex editor.");
    } else {
        std::fs::File::create(INGESTIONS_FILE.to_string()).unwrap();
        ingestions_bytes_loaded_des = HashMap::new();
        let ingesstions_bytes_loaded_ser =
            bincode::serialize(&ingestions_bytes_loaded_des).unwrap();
        std::fs::write(INGESTIONS_FILE.to_string(), ingesstions_bytes_loaded_ser).unwrap();
    }
    ingestions_bytes_loaded_des
}

pub fn get_user_date(current: NaiveDateTime) -> chrono::NaiveDate {
    let current_date = current.date();
    let date: chrono::NaiveDate =
        inquire::CustomType::<chrono::NaiveDate>::new("Enter the date (YYYY-MM-DD):")
            .with_placeholder("YYYY-MM-DD")
            .with_default(current_date)
            .with_parser(&|input| {
                chrono::NaiveDate::parse_from_str(input, "%Y-%m-%d").map_err(|_| ())
            })
            .with_error_message("Please enter a valid date and time in the format YYYY-MM-DD")
            .with_help_message("Use the format YYYY-MM-DD")
            .prompt()
            .unwrap();
    date
}

pub fn get_user_time(current: NaiveDateTime) -> chrono::NaiveTime {
    let current_time = current.time();
    let time: chrono::NaiveTime =
        inquire::CustomType::<chrono::NaiveTime>::new("Enter the time (HH:MM):")
            .with_placeholder("HH:MM")
            .with_default(current_time)
            .with_parser(&|input| chrono::NaiveTime::parse_from_str(input, "%H:%M").map_err(|_| ()))
            .with_error_message("Please enter a valid time in the format HH:MM.")
            .with_help_message("Use the format HH:MM")
            .prompt()
            .unwrap();
    time
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

pub fn get_substance() -> Substance {
    let substances = crate::substance_util::substances_to_vec();
    if substances.is_empty() {
        eprintln!("Add a substance before you log an ingestions");
        exit(1)
    }
    let substance_select = inquire::Select::new("What did yout ingest?", substances)
        .prompt()
        .unwrap();

    let substance_file: HashMap<Uuid, Substance> = crate::substance_util::ensure_substance_file();
    let substances: Vec<Substance> = substance_file
        .into_iter()
        .filter_map(|(_, s)| {
            if s.name == substance_select {
                Some(s)
            } else {
                None
            }
        })
        .collect();

    if substances.len() != 1 {
        eprintln!("Substance not found or multiple substances with the same name.");
        exit(1);
    }

    let substance = substances.into_iter().next().unwrap();
    dbg!(&substance);

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
        ingestion.substance.name,
        ingestion.ingestion_method,
        ingestion.dose.value,
        ingestion.dose.unit,
        ingestion.time,
    );
    let confirm =
        inquire::prompt_confirmation("Does the ingestion above look alright? [y/N]").unwrap();
    confirm
}
