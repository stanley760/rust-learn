use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;
use clap::{Args, Subcommand, Parser};
use colored::Colorize;
use mime::Mime;
use reqwest::{Client, header, Response, Url};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

#[derive(Parser)]
#[command(version="1.0.0", author="stanley", color=clap::ColorChoice::Always)]
struct Opts {
    #[command(subcommand)]
    methods: Methods,
}

#[derive(Subcommand)]
enum Methods {
    Get(Get),
    Post(Post),
}

#[derive(Args)]
struct Get {
    #[arg(value_parser=parse_url)]
    url: String,
}

#[derive(Args)]
struct Post {
    #[arg(value_parser=parse_url)]
    url: String,
    #[arg(value_parser=parse_pair)]
    body: Vec<Pair>,
}

#[derive(Debug, Clone)]
struct Pair {
    k: String,
    v: String,
    t: PairType,
}

#[derive(Debug, Clone)]
enum PairType {
    Header,
    Param,
}

impl FromStr for Pair {
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair_type;
        let split_char  = if s.contains(":") {
            pair_type = PairType::Header;
            ':'
        } else {
            pair_type = PairType::Param;
            '='
        };
        let mut split = s.split(split_char);
        let err = || anyhow!(format!("failed to parse pair: {}", s));
        Ok(Self {
            k: split.next().ok_or_else(err)?.into(),
            v: split.next().ok_or_else(err)?.into(),
            t: pair_type,
        })
    }
}

fn parse_url(s: &str) -> anyhow::Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

fn parse_pair(s: &str) -> anyhow::Result<Pair> {
    Ok(s.parse()?)
}

async fn get(cli: Client, args: &Get) -> anyhow::Result<()> {
    let resp = cli.get(&args.url).send().await?;
    println!("{}", resp.text().await?);
    Ok(())
}

async fn post(cli: Client, args: &Post) -> anyhow::Result<()> {
    let mut body = HashMap::new();
    let mut header_map = HeaderMap::new();

    for pair in args.body.iter() {
        match pair.t {
            PairType::Param => {body.insert(&pair.k, &pair.v);}
            PairType::Header => {
                if let Ok(name) = HeaderName::from_str(pair.k.as_str()) {
                    if let Ok(value) = HeaderValue::from_str(pair.v.as_str()) {
                        header_map.insert(name, value);
                    } else {
                        println!("invalid header value: {}", pair.v);
                    }
                } else {
                    println!("invalid header key: {}", pair.k);
                }
            }
        }
    }
    let resp = cli.post(&args.url)
                    .headers(header_map)
                    .json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

async fn print_resp(resp: Response) -> anyhow::Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).yellow();
    println!("{}\n", status);
}

fn print_headers(resp: &Response) {
    for (key, val) in resp.headers() {
        println!("{}: {:?}", key.to_string().green(), val);
    }
}

fn print_body(mime: Option<Mime>, resp: &String) {
    match mime {
        Some(v) => {
            if v == mime::APPLICATION_JSON {
                println!("{}", jsonxf::pretty_print(resp).unwrap().cyan())
            }
        },
        _ => println!("{}", resp),
    }
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers().get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

// windows[powershell]:
// cargo build --quiet; ..\target\debug\httpie post a=1 b=2

// linux/macos[bash]:
// cargo build --quiet && ../target/debug/httpie post a=1 b=2

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    let client = Client::new();
    let res = match opts.methods {
        Methods::Get(args) => get(client, &args).await?,
        Methods::Post(args) => post(client, &args).await?,
    };
    Ok(res)
}
