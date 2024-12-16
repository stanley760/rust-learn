use std::{fmt::{self, Formatter}, str::FromStr};

use clap::Parser;

use super::verify_input;

#[derive(Debug, Parser)]
pub enum TextOpts {
    #[command(about = "generate a signature")]
    Sign(TextSignOpts),

    #[command(about = "Verify a signature")]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    
    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser= verify_input)]
    pub key: String,

    #[arg(long, default_value= "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    
    #[arg(short, long, value_parser = verify_input, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser= verify_input)]
    pub key: String,

    #[arg(short, long)]
    pub sig: String,

    #[arg(long, default_value= "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Clone, Debug, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse::<TextSignFormat>()
}

impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => anyhow::bail!("Invalid format"),
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}