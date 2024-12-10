use clap::Parser;

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