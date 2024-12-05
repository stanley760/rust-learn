use clap::Parser;
use rcli::process::parse_csv;

use rcli::operation::{Opts, Subcommand};

// cargo run -- csv -i assets/juventus.csv --format yaml
// cargo run -- csv -i assets/juventus.csv --format json
// cargo run -- csv -i assets/juventus.csv --format toml

fn main() -> anyhow::Result<()>{
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            parse_csv(&opts.input, output, opts.format)?;
        }
    }
    Ok(())
}
