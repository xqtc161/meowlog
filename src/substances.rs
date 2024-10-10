use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;
use strum::{EnumIter, IntoEnumIterator};
use uuid::Uuid;

use crate::config::SUBSTANCES_FILE;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Substance {
    name: String,
    class: SubstanceClass,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, strum::Display, strum::EnumIter)]
enum SubstanceClass {
    Stimulant,
    Depressant,
    Psychedelic,
    Dissociative,
    Cannabinoid,
    Entheogen,
    Deliriant,
    Empathogen,
    Neurotransmitter,
}

impl SubstanceClass {
    fn to_string(&self) -> String {
        match self {
            Self::Stimulant => "Stimulant".to_string(),
            Self::Depressant => "Depressant".to_string(),
            Self::Psychedelic => "Psychedelic".to_string(),
            Self::Dissociative => "Dissociative".to_string(),
            Self::Cannabinoid => "Cannabinoid".to_string(),
            Self::Entheogen => "Entheogen".to_string(),
            Self::Deliriant => "Deliriant".to_string(),
            Self::Empathogen => "Empathogen".to_string(),
            Self::Neurotransmitter => "Neurotransmitter".to_string(),
        }
    }
}
pub fn path_exists(path: String) -> bool {
    std::fs::metadata(path).is_ok()
}
pub fn add_substance() -> Result<(), std::io::Error> {
    let mut substances_bytes_loaded_des: HashMap<Uuid, Substance>;
    if path_exists(SUBSTANCES_FILE.to_string()) {
        let substances_bytes_loaded = std::fs::read(SUBSTANCES_FILE.to_string()).unwrap();
        substances_bytes_loaded_des = bincode::deserialize(&substances_bytes_loaded).unwrap();
    } else {
        std::fs::File::create(SUBSTANCES_FILE.to_string()).unwrap();
        substances_bytes_loaded_des = HashMap::new();
    }
    let name = inquire::prompt_text("What is the substances name?").unwrap();
    if !substances_bytes_loaded_des.values().any(|x| x.name == name) {
        let class_variants = SubstanceClass::iter().collect::<Vec<_>>();
        let class_select = inquire::Select::new("What type of substance is this?", class_variants)
            .prompt()
            .unwrap();
        let substance = Substance {
            name,
            class: class_select,
        };
        let subs_hash = substances_bytes_loaded_des.insert(Uuid::new_v4(), substance);
        let sub_enc = bincode::serialize(&substances_bytes_loaded_des).unwrap();
        match std::fs::write(SUBSTANCES_FILE.to_string(), sub_enc) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    } else {
        println!("Substance already exists!");
        Ok(())
    }
}

pub fn list_substances() -> Result<(), std::io::Error> {
    let sub_read = std::fs::read(SUBSTANCES_FILE.to_string()).unwrap();
    let sub_dec: HashMap<Uuid, Substance> = bincode::deserialize(&sub_read).unwrap();
    for (id, substance) in sub_dec.clone().into_iter() {
        println!(
            "Name:  {}\nClass: {:?}\nUUID:  {:?}\n",
            substance.name, substance.class, id
        );
    }

    Ok(())
}

pub fn substances_to_vec() -> Vec<String> {
    let sub_read_res = std::fs::read(SUBSTANCES_FILE.to_string());
    let sub_read = match sub_read_res {
        Ok(sub_contents) => sub_contents,
        Err(_) => {
            println!("Error! Substance file does not exist. Creating file...");
            let hash: HashMap<Uuid, Substance> = HashMap::new();
            let hash_ser = bincode::serialize(&hash).unwrap();
            std::fs::write(SUBSTANCES_FILE.to_string(), hash_ser).unwrap();
            let ret: Vec<u8> = vec![];
            ret
        }
    };
    let sub_dec: HashMap<Uuid, Substance> = bincode::deserialize(&sub_read).unwrap();
    let mut sub_vec: Vec<String> = vec![];
    for (id, substance) in sub_dec.clone().into_iter() {
        sub_vec.push(substance.name);
    }
    sub_vec
}

pub fn create_substances_file() -> Result<(), std::io::Error> {
    let hash: HashMap<Uuid, Substance> = HashMap::new();
    let hash_ser = bincode::serialize(&hash).unwrap();
    std::fs::write(SUBSTANCES_FILE.to_string(), hash_ser)
}
