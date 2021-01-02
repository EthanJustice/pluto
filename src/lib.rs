#[macro_use]
// std

// crates
use pest::Parser;
use pest_derive::*;

// local

#[derive(Parser)]
#[grammar = "lib.pest"]
pub struct ProtoParser;

#[cfg(test)]
mod tests {
    use std::fs::{read_dir, read_to_string};

    use super::*;
    // todo: fuzzing
    #[test]
    fn valid_pairs() {
        let pair = ProtoParser::parse(Rule::pair, "example 22");
        assert!(pair.is_ok());
    }

    #[test]
    fn valid_number() {
        let num = ProtoParser::parse(Rule::number, "2000000");
        assert!(num.is_ok());
    }

    #[test]
    fn valid_string() {
        let raw = String::from("\"This is a string\"");
        let string = ProtoParser::parse(Rule::string, raw.as_str());
        assert!(string.is_ok());
    }

    #[test]
    fn valid_booleans() {
        let t = ProtoParser::parse(Rule::boolean, "true");
        let f = ProtoParser::parse(Rule::boolean, "false");
        assert!(t.is_ok());
        assert!(f.is_ok());

        let nt = ProtoParser::parse(Rule::boolean, "truee");
        assert!(nt.is_ok());
    }

    #[test]
    fn test_examples() {
        use std::fs::{read_dir, read_to_string};
        use std::path::Path;
        read_dir("./examples").unwrap().for_each(|f| {
            let path = f.unwrap().path();
            let file = read_to_string(path).unwrap();
            file.split("\r\n").for_each(|line| {
                // skips blank lines
                if line.len() > 2 {
                    if line.starts_with("/") {
                        let rule = ProtoParser::parse(Rule::comment, line);
                        assert!(rule.is_ok());
                    } else {
                        let rule = ProtoParser::parse(Rule::pair, line);
                        assert!(rule.is_ok());
                    }
                }
            })
        })
    }
}
