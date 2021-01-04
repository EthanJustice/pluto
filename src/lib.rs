#[macro_use]
// std

// crates
use pest::Parser;
use pest_derive::*;

// local

#[derive(Parser)]
#[grammar = "lib.pest"]
pub struct PlutoParser;

pub fn validate(input: &str) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    // todo: better line break splitting implementation
    input.split("\r\n").enumerate().for_each(|(idx, line)| {
        // skips blank lines
        if line.len() > 2 {
            if line.starts_with("/") {
                let rule = PlutoParser::parse(Rule::comment, line);
                if rule.is_ok() == false {
                    let mut error = String::from("Invalid comment on line ");
                    error.push_str(idx.to_string().as_str());
                    errors.push(error);
                }
            } else {
                if line.contains(" ") == false {
                    let rule = PlutoParser::parse(Rule::group, line);
                    if rule.is_ok() == false {
                        let mut error = String::from("Invalid group on line ");
                        error.push_str(idx.to_string().as_str());
                        errors.push(error);
                    }
                } else {
                    let rule = PlutoParser::parse(Rule::pair, line);
                    if rule.is_ok() == false {
                        let mut error = String::from("Invalid key/value pair on line ");
                        error.push_str(idx.to_string().as_str());
                        errors.push(error);
                    }
                }
            }
        }
    });
    if errors.len() == 0 {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{read_dir, read_to_string};

    use super::*;
    // todo: fuzzing
    #[test]
    fn valid_pairs() {
        let pair = PlutoParser::parse(Rule::pair, "example 22");
        assert!(pair.is_ok());
    }

    #[test]
    fn valid_number() {
        let num = PlutoParser::parse(Rule::number, "2000000");
        assert!(num.is_ok());
    }

    #[test]
    fn valid_string() {
        let raw = String::from("\"This is a string\"");
        let string = PlutoParser::parse(Rule::string, raw.as_str());
        assert!(string.is_ok());
    }

    #[test]
    fn valid_booleans() {
        let t = PlutoParser::parse(Rule::boolean, "true");
        let f = PlutoParser::parse(Rule::boolean, "false");
        assert!(t.is_ok());
        assert!(f.is_ok());

        let nt = PlutoParser::parse(Rule::boolean, "truee");
        assert!(nt.is_ok());
    }

    #[test]
    fn test_examples() {
        use std::fs::{read_dir, read_to_string};
        use std::path::Path;
        read_dir("./examples").unwrap().for_each(|f| {
            let path = f.unwrap().path();
            let file = read_to_string(path).unwrap();
            assert!(validate(file).is_ok());
        })
    }
}
