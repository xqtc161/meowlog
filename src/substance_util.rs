use std::collections::HashMap;
use uuid::Uuid;

use crate::substances::SubstanceClass;
use crate::util::path_exists;
use crate::{config::SUBSTANCES_FILE, substances::Substance};

pub fn ensure_substance_file() -> HashMap<Uuid, Substance> {
    let substances_bytes_loaded_des: HashMap<Uuid, Substance>;
    if path_exists(SUBSTANCES_FILE.to_string()) {
        let substances_bytes_loaded = std::fs::read(SUBSTANCES_FILE.to_string()).unwrap();
        substances_bytes_loaded_des = bincode::deserialize(&substances_bytes_loaded).unwrap();
    } else {
        std::fs::File::create(SUBSTANCES_FILE.to_string()).unwrap();
        substances_bytes_loaded_des = HashMap::new();
    }
    substances_bytes_loaded_des
}

pub fn get_substance_class(msg: &str, variants: Vec<SubstanceClass>) -> SubstanceClass {
    let class = inquire::Select::new(msg, variants).prompt().unwrap();
    class
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
    for (_id, substance) in sub_dec.clone().into_iter() {
        sub_vec.push(substance.name);
    }
    sub_vec
}

pub fn create_substances_file() -> Result<(), std::io::Error> {
    let hash: HashMap<Uuid, Substance> = HashMap::new();
    let hash_ser = bincode::serialize(&hash).unwrap();
    std::fs::write(SUBSTANCES_FILE.to_string(), hash_ser)
}
