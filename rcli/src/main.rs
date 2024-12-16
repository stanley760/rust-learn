use clap::Parser;
use rcli::Base64Opts;
use rcli::parse_csv;
use rcli::gen_pwd;
use rcli::{Opts, Subcommand};
use rcli::process_base64_decode;
use rcli::process_base64_encode;
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

            gen_pwd::parse_gen_pwd(&opts)?;
        }

        Subcommand::Base64(opts) => match opts{
            Base64Opts::Encode(opts) => {
                
                process_base64_encode(&opts.input, opts.format)?;
            },
            Base64Opts::Decode(opts) => {
                process_base64_decode(&opts.input, opts.format)?;
            }
        }

        Subcommand::Text(opts) => {
            println!("{:?}", opts);
        }
    }
    Ok(())
}
