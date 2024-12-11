use std::{fs::File, io::Read};
use base64::{engine::general_purpose, Engine as _};
use crate::operation::base64::Base64Format;

pub fn process_base64_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    
    let buf = read_file(input)?;

    let b64 = match format {
        Base64Format::Standard => general_purpose::STANDARD.encode(buf),
        Base64Format::UrlSafe => general_purpose::URL_SAFE.encode(buf),
    };
    
    println!("{:?}", b64);
    Ok(())
}

pub fn process_base64_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {

    let buf = read_file(input)?;

    let decoded = match format {
        Base64Format::Standard => general_purpose::STANDARD.decode(buf)?,
        Base64Format::UrlSafe => general_purpose::URL_SAFE.decode(buf)?,
    };

    println!("{:?}", String::from_utf8(decoded)?);
    Ok(())
}

fn read_file(input: &str) -> Result<Vec<u8>, anyhow::Error> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}