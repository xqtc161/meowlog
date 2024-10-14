use bincode;
use serde_json;

pub fn parse() -> Result<(), std::io::Error> {
    let drugs = std::fs::read("drugs.json").unwrap();
    Ok(())
}
