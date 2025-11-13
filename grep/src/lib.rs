use std::env;
use std::error::Error;
use std::fs;

pub fn read_file(config: Config) -> Result<(), Box<dyn Error>> {
    let result = fs::read_to_string(config.filename)?;

    let condition = if config.case_sensitive {
        search(&config.query, &result)
    } else {
        search_insensitive(&config.query, &result)
    };
    for line in condition {
        println!("{}", line);
    }
    //println!("the file content is:\n {}", result);
    Ok(())
}

#[allow(dead_code)]
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let q_to_lower = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&q_to_lower))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "product";
        let contents = "\
Rust:
safe, fast, productive.
Product three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct there.";

        println!("{:?}", search_insensitive(query, contents));
    }
}
