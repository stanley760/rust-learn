use clap::Parser;
use rcli::parse_csv;
use rcli::parse_gen_pwd;
use rcli::process_base64_decode;
use rcli::process_base64_encode;
use rcli::process_sign;
use rcli::Base64Opts;
use rcli::TextSignFormat;
use rcli::{Opts, Subcommand};
// cargo run -- csv -i assets/juventus.csv --format yaml
// cargo run -- csv -i assets/juventus.csv --format json
// cargo run -- csv -i assets/juventus.csv --format toml

fn main() -> anyhow::Result<()> {
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
            parse_gen_pwd(&opts)?;
        }

        Subcommand::Base64(opts) => match opts {
            Base64Opts::Encode(opts) => {
                process_base64_encode(&opts.input, opts.format)?;
            }
            Base64Opts::Decode(opts) => {
                process_base64_decode(&opts.input, opts.format)?;
            }
        },

        Subcommand::Text(opts) => match opts {
            rcli::TextOpts::Sign(opts) => match opts.format {
                TextSignFormat::Blake3 => {
                    process_sign(&opts.input, &opts.key, opts.format)?;
                }
                rcli::TextSignFormat::Ed25519 => {
                    println!("Ed25519");
                }
            },
            rcli::TextOpts::Verify(opts) => {}
        },
    }
    Ok(())
}
