use std::fmt;
use std::fmt::Formatter;
use std::path::Path;
use std::str::FromStr;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Parser, Debug)]
pub enum Subcommand {
    #[command(name = "csv", about = "CSV subcommand")]
    Csv(CsvOpts),
    #[command(name = "genpwd", about = "Generate a random password")]
    Genpwd(GenPwdOpts),
}

#[derive(Parser, Debug, Clone, Copy)]
pub enum Format {
    Json,
    Yaml,
    Toml,
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: Format,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Parser)]
pub struct GenPwdOpts {
    #[arg(long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t= true)]
    pub numbers: bool,

    #[arg(long, default_value_t= true)]
    pub symbols: bool,
}

fn verify_input(input: &str) -> Result<String, &'static str> {
    if Path::new(input).exists()  {
        Ok(input.into())
    } else {
        Err("Input file does not exist")
    }
}

fn parse_format(format: &str) -> Result<Format, anyhow::Error> {
    format.parse::<Format>()
}

impl From<Format> for &'static str {
    fn from(value: Format) -> Self {
        match value {
            Format::Json => "json",
            Format::Yaml => "yaml",
            Format::Toml => "toml",
        }
    }
}

impl FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Format::Json),
            "yaml" => Ok(Format::Yaml),
            "toml" => Ok(Format::Toml),
            _ => anyhow::bail!("Invalid format"),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}