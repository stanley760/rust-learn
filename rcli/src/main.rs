use clap::Parser;
use rcli::operation::base64::Base64Opts;
use rcli::process::parse_csv;
use rcli::process::gen_pwd;
use rcli::operation::{Opts, Subcommand};
use rcli::process::process_base64_decode;
use rcli::process::process_base64_encode;
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
        Subcommand::Genpwd(opts) => {

            println!("{:?}", gen_pwd::parse_gen_pwd(&opts)?);
        }

        Subcommand::Base64(opts) => match opts{
            Base64Opts::Encode(opts) => {
                
                println!("{:?}", process_base64_encode(&opts.input, opts.format));
            },
            Base64Opts::Decode(opts) => {
                println!("{:?}", process_base64_decode(&opts.input, opts.format));
            }
        }
    }
    Ok(())
}
