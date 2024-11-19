use serde::{Deserialize, Serialize};
use csv::Reader;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String, 
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn parse_csv(input: &str, output : &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let result: Vec<Player> = reader.deserialize().collect::<Result<Vec<Player>, _>>()?;


    let json = serde_json::to_string_pretty(&result)?;
    fs::write(output, json)?;
    Ok(())
}