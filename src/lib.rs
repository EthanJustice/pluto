#[macro_use]
// std

// crates
use pest::Parser;
use pest_derive::*;

// local

#[derive(Parser)]
#[grammar = "lib.pest"]
pub struct CharonParser;

#[cfg(test)]
mod tests {
    use std::fs::{read_dir, read_to_string};

    use super::*;
    // todo: fuzzing
    #[test]
    fn valid_pairs() {
        let pair = CharonParser::parse(Rule::pair, "example 22");
        assert!(pair.is_ok());
    }

    #[test]
    fn valid_number() {
        let num = CharonParser::parse(Rule::number, "2000000");
        assert!(num.is_ok());
    }

    #[test]
    fn valid_string() {
        let raw = String::from("\"This is a string\"");
        let string = CharonParser::parse(Rule::string, raw.as_str());
        assert!(string.is_ok());
    }

    #[test]
    fn test_examples() {
        use std::fs::{read_dir, read_to_string};
        use std::path::Path;
        read_dir("./examples").unwrap().for_each(|f| {
            let path = f.unwrap().path();
            let file = read_to_string(path).unwrap();
            file.split("\n").for_each(|line| {
                // skips blank lines
                if line.len() > 2 {
                    let rule = CharonParser::parse(Rule::pair, line);
                    //                println!("{:#?}", rule);
                    assert!(rule.is_ok());
                }
            })
        })
    }
}
