use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Too Few argumenets, Atleast 3 were expected");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        if ignore_case {
            eprintln!("IGNORE_CASE is set by environment variable");
        }

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;

    let result = if config.ignore_case {
        search_case_insensitive(&content, &config.query)
    } else {
        search_case_sensitive(&content, &config.query)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(content: &'a str, query: &str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = vec![];
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(content: &'a str, query: &str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = vec![];
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    fn give_content() -> &'static str {
        "Hello World\nHello Mr Bruno\nHello Tim\nworld is such a lovely place\nthank you god for giving me such a lovely life"
    }

    #[test]
    fn search_case_sensitive_test_simple1() {
        let content = give_content();
        assert_eq!(
            search_case_sensitive(content, "Hello"),
            vec!["Hello World", "Hello Mr Bruno", "Hello Tim"]
        );
    }

    #[test]
    fn search_case_sensitive_test_simple2() {
        let content = give_content();
        assert_eq!(
            search_case_sensitive(content, "lovely"),
            vec![
                "world is such a lovely place",
                "thank you god for giving me such a lovely life"
            ]
        );
    }

    #[test]
    fn search_case_sensitive_test_not_in_text_in_as_it_is() {
        let content = give_content();
        assert_eq!(search_case_sensitive(content, "hello"), vec![] as Vec<&str>);
    }

    #[test]
    fn search_case_sensitive_test_empty_query() {
        let content = give_content();
        assert_eq!(
            search_case_sensitive(content, ""),
            content.lines().collect::<Vec<&str>>()
        );
    }

    #[test]
    fn search_case_sensitive_empty_content() {
        assert_eq!(search_case_sensitive("", "random"), Vec::<&str>::new());
    }

    #[test]
    fn search_case_insensitive_simple1() {
        let content = give_content();
        assert_eq!(
            search_case_insensitive(content, "WORLD"),
            vec!["Hello World", "world is such a lovely place"]
        );
    }
}
