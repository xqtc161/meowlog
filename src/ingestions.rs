use chrono::{NaiveDateTime, Utc};
use color_eyre::Section;
use inquire;
use serde::{self, Deserialize, Serialize};
use std::{collections::HashMap, process::exit};
use strum::{EnumIter, IntoEnumIterator};
use uuid::Uuid;

use crate::{config::INGESTIONS_FILE, substances::Substance};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Ingestion {
    substance: String,
    dose: Dose,
    ingest_method: IngestionMethod,
    time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Dose {
    unit: String,
    value: f64,
}

#[derive(Serialize, Deserialize, Debug, strum::Display, strum::EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum DoseUnit {
    Ug,
    Mg,
    G,
    Ml,
}

#[derive(Serialize, Deserialize, Debug, Clone, strum::Display, strum::EnumIter)]
pub enum IngestionMethod {
    Oral,
    Sublingual,
    Buccal,
    Insuffulated,
    Rectal,
    Transdermal,
    Subcutaneous,
    Intramuscular,
    Intravenous,
    Smoked,
    Inhaled,
}

pub fn add_ingestion() {
    let mut ingesstions_bytes_loaded_des: HashMap<Uuid, Ingestion>;
    if crate::substances::path_exists(INGESTIONS_FILE.to_string()) {
        let substances_bytes_loaded = std::fs::read(INGESTIONS_FILE.to_string()).unwrap();
        ingesstions_bytes_loaded_des = bincode::deserialize(&substances_bytes_loaded).unwrap();
    } else {
        std::fs::File::create(INGESTIONS_FILE.to_string()).unwrap();
        ingesstions_bytes_loaded_des = HashMap::new();
        let ingesstions_bytes_loaded_ser =
            bincode::serialize(&ingesstions_bytes_loaded_des).unwrap();
        std::fs::write(INGESTIONS_FILE.to_string(), ingesstions_bytes_loaded_ser).unwrap();
    }
    let substances = crate::substances::substances_to_vec();
    if substances.is_empty() {
        eprintln!("Add a substance before you log an ingestions");
        exit(1)
    }
    let substance = inquire::Select::new("What did yout ingest?", substances)
        .prompt()
        .unwrap();
    let ingestion_method_select = inquire::Select::new(
        "How did you ingest?",
        IngestionMethod::iter().collect::<Vec<_>>(),
    )
    .prompt()
    .unwrap();

    dbg!(&substance);
    dbg!(&ingestion_method_select);
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
    let dose_num: f64 = inquire::prompt_f64("Enter the amount consumed:").unwrap();
    let dose_unit = inquire::Select::new(
        "What unit should be used?",
        DoseUnit::iter().collect::<Vec<_>>(),
    )
    .prompt()
    .unwrap();

    let dose = Dose {
        unit: dose_unit.to_string(),
        value: dose_num,
    };
    let ingestion = Ingestion {
        substance,
        dose,
        ingest_method: ingestion_method_select,
        time: date_time,
    };
    println!(
        "Substance:  {} ({})\nDose:       {}{}\nTime:       {}\n",
        ingestion.substance,
        ingestion.ingest_method,
        ingestion.dose.value,
        ingestion.dose.unit,
        ingestion.time,
    );
    let confirm =
        inquire::prompt_confirmation("Does the ingestion above look alright? [y/N]").unwrap();
    dbg!(&confirm);
    if confirm {
        ingesstions_bytes_loaded_des.insert(Uuid::new_v4(), ingestion.clone());
        let ingestion_ser = bincode::serialize(&ingesstions_bytes_loaded_des).unwrap();
        std::fs::write(INGESTIONS_FILE.to_string(), ingestion_ser);
    } else {
        add_ingestion();
    }
}

pub fn list_ingestions() -> Result<(), std::io::Error> {
    let ing_read = std::fs::read(INGESTIONS_FILE.to_string()).unwrap();
    let ing_dec: HashMap<Uuid, Ingestion> = bincode::deserialize(&ing_read).unwrap();
    for (id, ingestion) in ing_dec.clone().into_iter() {
        println!(
            "Substance:  {} ({})\nDose:       {}{}\nTime:       {}\nUUID:       {:?}\n",
            ingestion.substance,
            ingestion.ingest_method,
            ingestion.dose.value,
            ingestion.dose.unit,
            ingestion.time,
            id
        );
    }

    Ok(())
}

pub fn create_ingestions_file() -> Result<(), std::io::Error> {
    let hash: HashMap<Uuid, Ingestion> = HashMap::new();
    let hash_ser = bincode::serialize(&hash).unwrap();
    std::fs::write(INGESTIONS_FILE.to_string(), hash_ser)
}
