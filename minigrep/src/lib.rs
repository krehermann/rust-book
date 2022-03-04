use std::env;
use std::error::Error;
use std::fs;
use std::io;

fn read_file(p: &str) -> io::Result<String> {
    let contents = fs::read_to_string(p);
    // println!("Read text:\n{}",contents);
    return contents;
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_file(config.filename)?;
    /*
    match contents {
        Ok(data) =>{  println!("contents:\n{}", data); Ok(())}
        Err(err) => { Err(Box::new(err))}
    }
    */
    let results = if config.case_sensitive {
        search(config.query, &contents)
    } else {
        search_case_insensitive(config.query, &contents)
    };
    println!("{:?}", results);
    Ok(())
}
pub struct Config<'a> {
    query: &'a str,
    filename: &'a str,
    case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() != 2 {
            return Err(stringify!(
                "expected 2 args, got {} args: {:?} ",
                args.len(),
                args
            ));
        }
        Ok(Config {
            query: &args[0],
            filename: &args[1],
            case_sensitive: env::var("MINIGREP_CASE_INSENSITIVE").is_err(),
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /* v1
    let mut lines = contents
        .split_terminator("\n")
        .filter(|line| line.contains(query)).c;
    let mut out = Vec::new();
    while true {
        let v = lines.next();
        if v.is_none() {
            break;
        }
        out.push(v.unwrap());
    }
    out
    */

    /*v2
    let out: Vec<&str> = contents
        .split_terminator("\n")
        .filter(|line| line.contains(query))
        .collect();
    out
    */

    /* v3
    contents
        .split_terminator("\n")
        .filter(|line| line.contains(query))
        .collect()
    */

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
#[cfg(test)]
mod tests {
    use crate::{search, search_case_insensitive, Config};

    #[test]
    fn test_config() {
        let args = vec![String::from("one"), String::from("two")];
        let c = Config::new(&args).unwrap();
        assert_eq!(c.query, args[0]);
        assert_eq!(c.filename, args[1]);
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insentive() {
        let query = "Lower Case";
        let contents = "\
this is lower case
THIS IS NOT LOWER CASE";
        assert_eq!(
            vec!["this is lower case", "THIS IS NOT LOWER CASE"],
            search_case_insensitive(query, contents)
        );
    }
}
