use std::{fs::File, io:: Read};
use anyhow::Result;

pub fn read_file_string(input: &str) -> Result<String, anyhow::Error> {
    let mut reader: Box<dyn Read> = read_file(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn read_file(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}