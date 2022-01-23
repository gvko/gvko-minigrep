use std::{env, error::Error, fs};

pub struct Input {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Input {
    pub fn new(mut args: env::Args) -> Result<Input, &'static str> {
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename string"),
        };
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        return Ok(Input {
            query,
            filename,
            case_sensitive,
        });
    }
}

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input.filename)?;
    let mut results = Vec::new();

    if input.case_sensitive {
        results = search(&input.query, &contents);
    } else {
        results = search_case_insensitive(&input.query, &contents);
    }

    for line in results {
        println!("{}", line);
    }

    return Ok(());
}

/// Searches for a given query string in the contents of a given text document
///
/// # Examples
/// ```
/// let query = "duct";
/// let contents = "\
///Rust:
///safe, fast, productive.
///Pick three.
///Duct tape.";
///
/// assert_eq!(vec!["safe, fast, productive."], gvko_minigrep::search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lower_case = &query.to_lowercase();

    return contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query_lower_case))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
