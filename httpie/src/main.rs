use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version="1.0.0", author="stanley")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Parser, Debug)]
struct Get {
    url: String,
}

#[derive(Parser, Debug)]
struct Post {
    url: String,
    body: Vec<String>,
}

// windows[powershell]:
// cargo build --quiet; ..\target\debug\httpie post a=1 b=2

// linux/macos[bash]:
// cargo build --quiet && ..\target\debug\httpie post a=1 b=2
fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
