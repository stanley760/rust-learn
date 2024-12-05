use crate::operation::opts::Format;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

pub fn parse_csv(input: &str, output : String, format: Format) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    
    let headers = reader.headers()?.clone();
    
    let result = reader.records().map(| res| {
        let record = res.unwrap();
        headers.iter().zip(record.iter()).collect::<Value>()
    }).collect::<Vec<Value>>();
    let content = match format {
        Format::Json => serde_json::to_string_pretty(&result)?,
        Format::Yaml => serde_yaml::to_string(&result)?,
        Format::Toml => {
            todo!("转换类型错误需要重构");
            toml::to_string(&result)?
        },
    };

    fs::write(output, content)?;
    Ok(())
}