use clap::Parser;
use super::{csv::CsvOpts, genpwd::GenPwdOpts};

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



