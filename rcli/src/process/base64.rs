use std::{fs::File, io:: Read};
use base64::{engine::general_purpose, Engine as _};
use crate::operation::base64::Base64Format;

pub fn process_base64_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    
    let buf = read_file(input)?;
    let buf = buf.trim();
    let b64 = match format {
        Base64Format::Standard => general_purpose::STANDARD.encode(&buf),
        Base64Format::UrlSafe => general_purpose::URL_SAFE_NO_PAD.encode(&buf),
    };
    
    println!("{:?}", b64);
    Ok(())
}

pub fn process_base64_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {

    let buf = read_file(input)?;
    let buf = buf.trim();
    let decoded = match format {
        Base64Format::Standard => general_purpose::STANDARD.decode(&buf)?,
        Base64Format::UrlSafe => general_purpose::URL_SAFE_NO_PAD.decode(&buf)?,
    };

    println!("{:?}", String::from_utf8(decoded)?);
    Ok(())
}

fn read_file(input: &str) -> Result<String, anyhow::Error> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    Ok(buf)
}