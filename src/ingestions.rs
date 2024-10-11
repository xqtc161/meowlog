use crate::ingestions_util::{
    ensure_ingestion_files, get_dose_unit, get_ingestion_confirmation, get_ingestion_method,
    get_substance, get_user_datetime,
};
use chrono::NaiveDateTime;
use inquire;
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::config::INGESTIONS_FILE;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ingestion {
    pub substance: String,
    pub dose: Dose,
    pub ingestion_method: IngestionMethod,
    pub time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dose {
    pub unit: String,
    pub value: f64,
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
    let mut ingesstions_bytes_loaded_des: HashMap<Uuid, Ingestion> = ensure_ingestion_files();

    let substance = get_substance();

    let ingestion_method = get_ingestion_method();

    let time: NaiveDateTime = get_user_datetime();
    let dose_num: f64 = inquire::prompt_f64("Enter the amount consumed:").unwrap();
    let dose_unit: DoseUnit = get_dose_unit();

    let dose = Dose {
        unit: dose_unit.to_string(),
        value: dose_num,
    };

    let ingestion = Ingestion {
        substance,
        dose,
        ingestion_method,
        time,
    };

    let confirm = get_ingestion_confirmation(ingestion.clone());
    if confirm {
        ingesstions_bytes_loaded_des.insert(Uuid::new_v4(), ingestion.clone());
        let ingestion_ser = bincode::serialize(&ingesstions_bytes_loaded_des).unwrap();
        std::fs::write(INGESTIONS_FILE.to_string(), ingestion_ser).unwrap();
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
            ingestion.ingestion_method,
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
