use serde::{Deserialize, Serialize};
use std::collections::HashMap;
mod parser;

pub fn parse() {
    let file = include_str!("../../../drugs.json");
    let db: DrugDatabase = serde_json::from_str(file).unwrap();
    println!("{:?}", db);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DrugDatabase(HashMap<String, Drug>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Drug {
    pub aliases: Option<Vec<String>>,
    pub categories: Option<Vec<Category>>,
    #[serde(rename = "formatted_aftereffects")]
    pub formatted_aftereffects: Option<Duration>,
    #[serde(rename = "formatted_dose")]
    pub formatted_dose: Option<Dose>,
    #[serde(rename = "formatted_duration")]
    pub formatted_duration: Option<Duration>,
    #[serde(rename = "formatted_effects")]
    pub formatted_effects: Option<Vec<String>>,
    #[serde(rename = "formatted_onset")]
    pub formatted_onset: Option<Duration>,
    pub links: Option<Links>,
    pub name: String,
    #[serde(rename = "pretty_name")]
    pub pretty_name: String,
    pub properties: Properties,
    pub pweffects: Option<HashMap<String, String>>,
    #[serde(rename = "dose_note")]
    pub dose_note: Option<String>,
    pub sources: Option<Sources>,
    pub combos: Option<Combos>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    Depressant,
    HabitForming,
    Tentative,
    ResearchChemical,
    Psychedelic,
    Stimulant,
    Dissociative,
    Inactive,
    Empathogen,
    Common,
    Benzodiazepine,
    Opioid,
    Supplement,
    Nootropic,
    Barbiturate,
    Deliriant,
    Ssri,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dose {
    pub oral: Option<Dosage>,
    pub insufflated: Option<Dosage>,
    pub rectal: Option<Dosage>,
    pub vapourized: Option<Dosage>,
    pub intravenous: Option<Dosage>,
    pub smoked: Option<Dosage>,
    pub sublingual: Option<Dosage>,
    pub buccal: Option<Dosage>,
    pub intramuscular: Option<Dosage>,
    pub transdermal: Option<Dosage>,
    pub hbwr: Option<Dosage>,
    #[serde(rename = "Morning_Glory")]
    pub morning_glory: Option<Dosage>,
    pub dried: Option<Dosage>,
    pub fresh: Option<Dosage>,
    #[serde(rename = "Insufflated(Pure)")]
    pub insufflated_pure: Option<Dosage>,
    #[serde(rename = "Oral(Benzedrex)")]
    pub oral_benzedrex: Option<Dosage>,
    #[serde(rename = "Oral(Pure)")]
    pub oral_pure: Option<Dosage>,
    pub dry: Option<Dosage>,
    pub wet: Option<Dosage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dosage {
    pub common: Option<String>,
    pub light: Option<String>,
    pub strong: Option<String>,
    pub threshold: Option<String>,
    pub heavy: Option<String>,
    pub dangerous: Option<String>,
    pub fatal: Option<String>,
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Duration {
    #[serde(rename = "_unit")]
    pub unit: Option<Unit>,
    pub value: Option<String>,
    pub insufflated: Option<String>,
    pub oral: Option<String>,
    pub rectal: Option<String>,
    pub vapourized: Option<String>,
    pub smoked: Option<String>,
    #[serde(rename = "Oral_ER")]
    pub oral_er: Option<String>,
    #[serde(rename = "Oral_IR")]
    pub oral_ir: Option<String>,
    pub intramuscular: Option<String>,
    pub intravenous: Option<String>,
    pub metabolites: Option<String>,
    pub parent: Option<String>,
    #[serde(rename = "Oral_MAOI")]
    pub oral_maoi: Option<String>,
    pub buccal: Option<String>,
    pub transdermal: Option<String>,
    pub sublingual: Option<String>,
    #[serde(rename = "Insufflated_IR")]
    pub insufflated_ir: Option<String>,
    #[serde(rename = "Insufflated_XR")]
    pub insufflated_xr: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    pub experiences: String,
    pub pihkal: Option<String>,
    pub tihkal: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {
    #[serde(rename = "after-effects")]
    pub after_effects: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub avoid: Option<String>,
    pub categories: Option<Vec<Category>>,
    pub dose: Option<String>,
    pub duration: Option<String>,
    #[serde(rename = "half-life")]
    pub half_life: Option<String>,
    pub onset: Option<String>,
    pub summary: Option<String>,
    #[serde(rename = "test-kits")]
    pub test_kits: Option<String>,
    pub experiences: Option<String>,
    pub warning: Option<String>,
    pub marquis: Option<String>,
    pub effects: Option<String>,
    pub risks: Option<String>,
    pub comeup: Option<String>,
    pub note: Option<String>,
    pub detection: Option<String>,
    pub wiki: Option<String>,
    pub mdma: Option<String>,
    pub tolerance: Option<String>,
    pub bioavailability: Option<String>,
    #[serde(rename = "dose_to_diazepam")]
    pub dose_to_diazepam: Option<String>,
    #[serde(rename = "adverse-effects")]
    pub adverse_effects: Option<String>,
    pub chemistry: Option<String>,
    pub contraindications: Option<String>,
    pub legal: Option<String>,
    #[serde(rename = "overdose-symptoms")]
    pub overdose_symptoms: Option<String>,
    pub pharmacokinetics: Option<String>,
    pub pharmacology: Option<String>,
    pub obtain: Option<String>,
    pub pharmacodynamics: Option<String>,
    #[serde(rename = "side-effects")]
    pub side_effects: Option<String>,
    pub molecule: Option<String>,
    pub vaporization: Option<String>,
    pub calculator: Option<String>,
    pub chart: Option<String>,
    pub oral: Option<String>,
    #[serde(rename = "general-advice")]
    pub general_advice: Option<String>,
    pub potentiators: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Combos {
    #[serde(rename = "2c-t-x")]
    pub c2_t_x: Option<Combo>,
    #[serde(rename = "2c-x")]
    pub c2_x: Option<Combo>,
    #[serde(rename = "5-meo-xxt")]
    pub c5_meo_xxt: Option<Combo>,
    pub alcohol: Option<Combo>,
    pub amphetamines: Option<Combo>,
    pub amt: Option<Combo>,
    pub benzodiazepines: Option<Combo>,
    pub caffeine: Option<Combo>,
    pub cannabis: Option<Combo>,
    pub cocaine: Option<Combo>,
    pub dextromethorphan: Option<Combo>,
    pub diphenhydramine: Option<Combo>,
    pub dmt: Option<Combo>,
    pub dox: Option<Combo>,
    #[serde(rename = "ghb/gbl")]
    pub ghb_gbl: Option<Combo>,
    pub lithium: Option<Combo>,
    pub ketamine: Option<Combo>,
    pub lsd: Option<Combo>,
    pub maois: Option<Combo>,
    pub mdma: Option<Combo>,
    pub mephedrone: Option<Combo>,
    pub mescaline: Option<Combo>,
    pub mushrooms: Option<Combo>,
    pub mxe: Option<Combo>,
    pub nbomes: Option<Combo>,
    pub nitrous: Option<Combo>,
    pub opioids: Option<Combo>,
    pub pcp: Option<Combo>,
    pub ssris: Option<Combo>,
    pub tramadol: Option<Combo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Combo {
    pub sources: Option<Vec<SourceData>>,
    pub note: Option<String>,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceData {
    pub author: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    #[serde(rename = "Low Risk & Decrease")]
    LowRiskAndDecrease,
    Dangerous,
    #[serde(rename = "Low Risk & No Synergy")]
    LowRiskAndNoSynergy,
    Caution,
    Unsafe,
    #[serde(rename = "Low Risk & Synergy")]
    LowRiskAndSynergy,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Unit {
    Hours,
    Minutes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sources {
    #[serde(rename = "_general")]
    pub general: Option<Vec<String>>,
    pub dose: Option<Vec<String>>,
    pub duration: Option<Vec<String>>,
    pub bioavailability: Option<Vec<String>>,
    pub legality: Option<Vec<String>>,
    pub onset: Option<Vec<String>>,
}
// ssh NZrVk5tjc7aJVHbDmzwVbcNtZ@lon1.tmate.io
