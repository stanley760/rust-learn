pub mod base64;
pub mod csv;
pub mod genpwd;
use std::path::Path;

use base64::Base64Opts;
use genpwd::GenPwdOpts;
use clap::Parser;
use self::csv::CsvOpts;


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

    #[command(subcommand)]
    Base64(Base64Opts),
}

fn verify_input(input: &str) -> Result<String, &'static str> {
    if input == "-" || Path::new(input).exists()  {
        Ok(input.into())
    } else {
        Err("Input file does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input() {
        assert_eq!(verify_input("-"), Ok("-".into()));
        // assert_eq!(verify_input("*"), Err("file not existed."));
        assert_eq!(verify_input("assets/juventus.csv"), Ok("assets/juventus.csv".into()));
        
    }
}