use crate::ingestions_util::{
    ensure_ingestion_files, get_dose_unit, get_ingestion_confirmation, get_ingestion_method,
    get_substance, get_user_date, get_user_time,
};
use chrono::{NaiveDate, NaiveTime, Utc};
use inquire;
use serde::{self, Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::process::exit;
use uuid::Uuid;

use crate::config::INGESTIONS_FILE;
use crate::substances::Substance;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Ingestion {
    pub substance: Substance,
    pub dose: Dose,
    pub ingestion_method: IngestionMethod,
    pub time: NaiveTime,
    pub date: NaiveDate,
}

impl std::fmt::Display for Ingestion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}   {} {}{}",
            self.date,
            self.time.format("%H:%M"),
            self.substance.name,
            self.dose.value,
            self.dose.unit
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Dose {
    pub unit: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, strum::Display, strum::EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum DoseUnit {
    Ug,
    Mg,
    G,
    Ml,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, strum::Display, strum::EnumIter, PartialEq)]
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

    let current_datetime = Utc::now().naive_utc();
    let date: NaiveDate = get_user_date(current_datetime);
    let time: NaiveTime = get_user_time(current_datetime);
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
        date,
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
    let ing_des: HashMap<Uuid, Ingestion> = bincode::deserialize(&ing_read).unwrap();
    for (id, ingestion) in ing_des.clone().into_iter() {
        println!(
            "Substance:  {} ({})\nDose:       {} {}\nTime:       {}\nUUID:       {:?}\n",
            ingestion.substance.name,
            ingestion.ingestion_method,
            ingestion.dose.value,
            ingestion.dose.unit,
            ingestion.time,
            id
        );
    }

    Ok(())
}

pub fn edit_ingestion() -> Result<(), std::io::Error> {
    let ing_des = ensure_ingestion_files();
    if ing_des.is_empty() {
        eprintln!("No ingestions to edit!");
        exit(1);
    }

    let mut ingest_sel_vec_id: Vec<Uuid> = Vec::new();
    let mut ingest_sel_vec_ing: Vec<Ingestion> = Vec::new();

    for ingestion in ing_des.clone().into_iter() {
        ingest_sel_vec_id.push(ingestion.0);
        ingest_sel_vec_ing.push(ingestion.1);
    }

    let ingest_select =
        inquire::Select::new("Which ingestion do you want to edit?", ingest_sel_vec_ing)
            .prompt()
            .unwrap();
    let ing_id = ing_des
        .iter()
        .map(|(key, &ref val)| {
            if val.substance.name == ingest_select.substance.name
                && val.substance.substance_class == ingest_select.substance.substance_class
                && val.date == ingest_select.date
                && val.time == ingest_select.time
            {
                key.clone()
            } else {
                unreachable!()
            }
        })
        .collect::<Vec<Uuid>>();

    let edit_select = inquire::MultiSelect::new(
        "What do you want to edit?",
        vec!["Substance", "Dose", "Ingestion Method", "Time", "Date"],
    )
    .prompt()
    .unwrap();

    for edit in edit_select {
        match edit {
            "Substance" => {
                let substance = get_substance();
                let ingestion = Ingestion {
                    substance,
                    dose: ingest_select.dose.clone(),
                    ingestion_method: ingest_select.ingestion_method.clone(),
                    time: ingest_select.time,
                    date: ingest_select.date,
                };
                let confirm = get_ingestion_confirmation(ingestion.clone());
                if confirm {
                    let mut ing_des = ensure_ingestion_files();
                    ing_des.insert(ing_id[0], ingestion.clone());
                    let ingestion_ser = bincode::serialize(&ing_des).unwrap();
                    std::fs::write(INGESTIONS_FILE.to_string(), ingestion_ser).unwrap();
                } else {
                    edit_ingestion();
                }
            }
            "Dose" => {
                let dose_num: f64 = inquire::prompt_f64("Enter the amount consumed:").unwrap();
                let dose_unit: DoseUnit = get_dose_unit();
                let dose = Dose {
                    unit: dose_unit.to_string(),
                    value: dose_num,
                };
                let ingestion = Ingestion {
                    substance: ingest_select.substance.clone(),
                    dose,
                    ingestion_method: ingest_select.ingestion_method.clone(),
                    time: ingest_select.time,
                    date: ingest_select.date,
                };
                let confirm = get_ingestion_confirmation(ingestion.clone());
                if confirm {
                    let mut ing_des = ensure_ingestion_files();
                    ing_des.insert(ing_id[0], ingestion.clone());
                    let ingestion_ser = bincode::serialize(&ing_des).unwrap();
                    std::fs::write(INGESTIONS_FILE.to_string(), ingestion_ser).unwrap();
                } else {
                    edit_ingestion();
                }
            }
            "Ingestion Method" => {
                let ingestion_method = get_ingestion_method();
                let ingestion = Ingestion {
                    substance: ingest_select.substance.clone(),
                    dose: ingest_select.dose.clone(),
                    ingestion_method,
                    time: ingest_select.time,
                    date: ingest_select.date,
                };
                let confirm = get_ingestion_confirmation(ingestion.clone());
                if confirm {
                    let mut ing_des = ensure_ingestion_files();
                    ing_des.insert(ing_id[0], ingestion.clone());
                    let ingestion_ser = bincode::serialize(&ing_des).unwrap();
                    std::fs::write(INGESTIONS_FILE.to_string(), ingestion_ser).unwrap();
                } else {
                    edit_ingestion();
                }
            }
            "Time" => {
                let time: NaiveTime = get_user_time(Utc::now().naive_utc());
                let ingestion = Ingestion {
                    substance: ingest_select.substance.clone(),
                    dose: ingest_select.dose.clone(),
                    ingestion_method: ingest_select.ingestion_method.clone(),
                    time,
                    date: ingest_select.date,
                };
                let confirm = get_ingestion_confirmation(ingestion.clone());
                if confirm {
                    let mut ing_des = ensure_ingestion_files();
                    ing_des.insert(ing_id[0], ingestion.clone());
                    let ingestion_ser = bincode::serialize(&ing_des).unwrap();
                    std::fs::write(INGESTIONS_FILE.to_string(), ingestion_ser).unwrap();
                } else {
                    edit_ingestion();
                }
            }
            "Date" => {
                let date: NaiveDate = get_user_date(Utc::now().naive_utc());
                let ingestion = Ingestion {
                    substance: ingest_select.substance.clone(),
                    dose: ingest_select.dose.clone(),
                    ingestion_method: ingest_select.ingestion_method.clone(),
                    time: ingest_select.time,
                    date,
                };
                let confirm = get_ingestion_confirmation(ingestion.clone());
                if confirm {
                    let mut ing_des = ensure_ingestion_files();
                    ing_des.insert(ing_id[0], ingestion.clone());
                    let ingestion_ser = bincode::serialize(&ing_des).unwrap();
                    std::fs::write(INGESTIONS_FILE.to_string(), ingestion_ser).unwrap();
                } else {
                    edit_ingestion();
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn create_ingestions_file() -> Result<(), std::io::Error> {
    let hash: HashMap<Uuid, Ingestion> = HashMap::new();
    let hash_ser = bincode::serialize(&hash).unwrap();
    std::fs::write(INGESTIONS_FILE.to_string(), hash_ser)
}

