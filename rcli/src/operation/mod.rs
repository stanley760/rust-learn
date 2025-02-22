mod base64;
mod csv;
mod genpwd;
mod text;

use clap::Parser;
use std::path::Path;
use self::csv::CsvOpts;

pub use base64::Base64Opts;
pub use genpwd::GenPwdOpts;
pub use text::TextOpts;
pub use text::TextSignFormat;
pub use base64::Base64Format;
pub use csv::Format;

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

    #[command(subcommand)]
    Text(TextOpts),
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
    use crate::operation::base64::Base64Format;

    use crate::process:: process_base64_encode;

    use super::*;

    #[test]
    fn test_verify_input() {
        assert_eq!(verify_input("-"), Ok("-".into()));
        // assert_eq!(verify_input("*"), Err("file not existed."));
        assert_eq!(verify_input("assets/juventus.csv"), Ok("assets/juventus.csv".into()));
        
    }

    #[test]
    fn test_process_encode() {
        let input = "cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_base64_encode(input, format).is_ok());
    }

}