use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::substance_util::{ensure_substance_file, get_substance_class, substances_to_vec};
use crate::SUBSTANCES_FILE;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Substance {
    pub name: String,
    pub substance_class: SubstanceClass,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum SubstanceClass {
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

impl PartialEq for SubstanceClass {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
pub fn add_substance() -> Result<(), std::io::Error> {
    let mut substances_bytes_loaded_des: HashMap<Uuid, Substance> = ensure_substance_file();
    let name = inquire::prompt_text("What is the substances name?").unwrap();
    if !substances_bytes_loaded_des.values().any(|x| x.name == name) {
        let class_variants = SubstanceClass::iter().collect::<Vec<_>>();
        let substance_class =
            get_substance_class("What type of substance is this?", class_variants);
        let substance = Substance {
            name,
            substance_class,
        };
        substances_bytes_loaded_des.insert(Uuid::new_v4(), substance);
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
            substance.name, substance.substance_class, id
        );
    }

    Ok(())
}

pub fn remove_substance() -> Result<(), std::io::Error> {
    let sub_read = std::fs::read(SUBSTANCES_FILE.to_string()).unwrap();
    let mut sub_dec: HashMap<Uuid, Substance> = bincode::deserialize(&sub_read).unwrap();

    let substances = substances_to_vec();
    let substances_select =
        inquire::MultiSelect::new("Which substance do you want to remove?", substances)
            .prompt()
            .unwrap();
    dbg!(&substances_select);
    for name in substances_select {
        let confirm = inquire::prompt_confirmation(format!(
            "Are you sure you want to remove '{}'? [y/N]",
            name
        ))
        .unwrap();
        if confirm {
            // Clone to avoid immutable borrow
            let sub_dec_clone = sub_dec.clone();
            let uuid =
                sub_dec_clone
                    .iter()
                    .find_map(|(id, val)| if val.name == name { Some(id) } else { None });
            if uuid.is_some() {
                let _ = sub_dec
                    .remove(uuid.expect("Fatal error. Couldn't find substance UUID in HashMap."));
            }
        }
    }

    let sub_enc = bincode::serialize(&sub_dec).unwrap();
    match std::fs::write(SUBSTANCES_FILE.to_string(), sub_enc) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum SubstanceEditOptions {
    Name,
    Class,
}

pub fn edit_substance() -> Result<(), std::io::Error> {
    let sub_read = std::fs::read(SUBSTANCES_FILE.to_string()).unwrap();
    let mut sub_dec: HashMap<Uuid, Substance> = bincode::deserialize(&sub_read).unwrap();

    let substances = substances_to_vec();
    let substance_name = inquire::Select::new("Which substance do you want to edit?", substances)
        .prompt()
        .unwrap();
    dbg!(&substance_name);
    let sub_dec_clone = sub_dec.clone();
    let uuid_opt = sub_dec_clone.iter().find_map(|(id, val)| {
        if val.name == substance_name {
            Some(id)
        } else {
            None
        }
    });
    if uuid_opt.is_some() {
        let uuid = uuid_opt.clone().unwrap().to_owned();
        let _ = sub_dec
            .remove(uuid_opt.expect("Fatal error. Couldn't find substance UUID in HashMap."));
        let edit_select = inquire::Select::new(
            format!("[{}] What do you want to edit?", substance_name).as_str(),
            SubstanceEditOptions::iter().collect::<Vec<_>>(),
        )
        .prompt()
        .unwrap();
        match edit_select {
            SubstanceEditOptions::Name => {
                let name_updated = inquire::prompt_text("What should the new name be?").unwrap();
                let class = match sub_dec_clone
                    .get(uuid_opt.expect("Fatal error. Couldn't find substance UUID in HashMap."))
                {
                    Some(class) => class.substance_class,
                    None => {
                        panic!("Fatal error. Couldn't find substance UUID in HashMap.")
                    }
                };
                dbg!(&class);
                let substance = Substance {
                    name: name_updated,
                    substance_class: class,
                };
                sub_dec.insert(uuid, substance);
            }
            SubstanceEditOptions::Class => {
                let class_variants = SubstanceClass::iter().collect::<Vec<_>>();
                let substance_class = get_substance_class(
                    format!(
                        "[{}] What should the new substance class be?",
                        substance_name
                    )
                    .as_str(),
                    class_variants,
                );
                let substance = Substance {
                    name: substance_name,
                    substance_class,
                };
                sub_dec.insert(uuid, substance);
            }
        }
        let sub_enc = bincode::serialize(&sub_dec).unwrap();
        std::fs::write(SUBSTANCES_FILE.to_string(), sub_enc).unwrap();
    }
    Ok(())
}
