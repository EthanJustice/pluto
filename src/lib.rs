#[macro_use]
// std

// crates
use pest::Parser;
use pest_derive::*;

// local

#[derive(Parser)]
#[grammar = "lib.pest"]
pub struct PlutoParser;

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
            file.split("\r\n").for_each(|line| {
                // skips blank lines
                if line.len() > 2 {
                    if line.starts_with("/") {
                        let rule = PlutoParser::parse(Rule::comment, line);
                        assert!(rule.is_ok());
                    } else {
                        if line.contains(" ") == false {
                            let rule = PlutoParser::parse(Rule::group, line);
                        //                            assert!(rule.is_ok());
                        } else {
                            let rule = PlutoParser::parse(Rule::pair, line);
                            assert!(rule.is_ok());
                        }
                    }
                }
            })
        })
    }
}
