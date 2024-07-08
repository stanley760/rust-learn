use std::error::Error;
use std::fs;

pub fn read_file(config: Config) -> Result<(), Box<dyn Error>> {
    let result = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &result) {
        println!("{}", line);
    }
    //println!("the file content is:\n {}", result);
    Ok(())
}

#[allow(dead_code)]
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str)| -> Vec<&'a str> {
    // todo
    let q_toLower = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(q_toLower))
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

        println!("{:?}", search(query, contents));
    }
}