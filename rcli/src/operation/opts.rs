use std::path::Path;

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
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input(input: &str) -> Result<String, &'static str> {
    if Path::new(input).exists()  {
        Ok(input.into())
    } else {
        Err("Input file does not exist")
    }
}