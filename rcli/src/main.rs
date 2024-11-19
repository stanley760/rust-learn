use clap::Parser;
use rcli::process::parse_csv;

use rcli::operation::{Opts, Subcommand};


fn main() -> anyhow::Result<()>{
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            parse_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}
