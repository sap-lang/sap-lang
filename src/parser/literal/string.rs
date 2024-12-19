use crate::parser::Rule;
use pest::iterators::{Pair, Pairs};
use regex::{Captures, Regex};

fn handle_special_escape(str: String) -> String {
    // hex 8 digit
    let regex_pattern4 = Regex::new(r"\\U([0-9a-fA-F]{8})").unwrap();
    if regex_pattern4.is_match(&str) {
        return regex_pattern4
            .replace_all(&str, |cap: &Captures| {
                let hex = &cap[1];
                let hex_int = u32::from_str_radix(hex, 16).unwrap();
                let char = char::from_u32(hex_int).unwrap();
                format!("{}", char)
            })
            .to_string();
    }

    // hex 4 digit
    let regex_pattern3 = Regex::new(r"\\u([0-9a-fA-F]{4})").unwrap();
    if regex_pattern3.is_match(&str) {
        return regex_pattern3
            .replace_all(&str, |cap: &Captures| {
                let hex = &cap[1];
                let hex_int = u32::from_str_radix(hex, 16).unwrap();
                let char = char::from_u32(hex_int).unwrap();
                format!("{}", char)
            })
            .to_string();
    }

    // hex 2 digit
    let regex_pattern2 = Regex::new(r"\\x([0-9a-fA-F]{2})").unwrap();
    if regex_pattern2.is_match(&str) {
        return regex_pattern2
            .replace_all(&str, |cap: &Captures| {
                let hex = &cap[1];
                let hex_int = u32::from_str_radix(hex, 16).unwrap();
                let char = char::from_u32(hex_int).unwrap();
                format!("{}", char)
            })
            .to_string();
    }

    // ascii_oct_digit
    let regex_pattern1 = Regex::new(r"\\([0-7]{1,3})").unwrap();
    if regex_pattern1.is_match(&str) {
        regex_pattern1
            .replace_all(&str, |cap: &Captures| {
                let oct = &cap[1];
                let oct_int = u32::from_str_radix(oct, 8).unwrap();
                let char = char::from_u32(oct_int).unwrap();
                format!("{}", char)
            })
            .to_string()
    } else {
        str
    }
}

fn handle_c_escape(str: &str, multiline: bool) -> String {
    match str {
        r"\0" => "\0".to_string(),
        r"\a" => "\x07".to_string(),
        r"\b" => "\x08".to_string(),
        r"\e" => "\x1b".to_string(),
        r"\f" => "\x0c".to_string(),
        r"\n" => "\x0a".to_string(),
        r"\r" => "\x0d".to_string(),
        r"\t" => "\x09".to_string(),
        r"\v" => "\x0b".to_string(),
        r"\?" => "\x3f".to_string(),
        r"\\" => "\\".to_string(),
        r#"\""# => {
            if multiline {
                r#"\""#.to_string()
            } else {
                r#"""#.to_string()
            }
        }
        r"\`" => {
            if multiline {
                r#"`"#.to_string()
            } else {
                r#"\`"#.to_string()
            }
        }
        _ => str.to_string(),
    }
}

fn handle_escape(str: &str, multiline: bool) -> String {
    handle_special_escape(handle_c_escape(str, multiline))
}

// FIXME: bug, " + " parse to "+ "
// FIXME: bug, ` + ` to unreachable
fn parse_string_inner(str: Pairs<Rule>, multiline: bool) -> String {
    str.map(|s| match s.as_rule() {
        Rule::normal_string_fragment => s.as_str().to_string(),
        Rule::escaped_string_fragment => handle_escape(s.as_str(), multiline),
        _ => unreachable!(),
    })
    .collect::<Vec<String>>()
    .join("")
}

pub fn parse_string(str: Pair<Rule>) -> StringLiteral {
    match str.as_rule() {
        Rule::raw_string_inner => StringLiteral::Raw(str.as_str().to_string()),
        Rule::normal_string_inner => {
            StringLiteral::SingleLine(parse_string_inner(str.into_inner(), false))
        }

        Rule::multiline_string_inner => {
            StringLiteral::MultiLine(parse_string_inner(str.into_inner(), true))
        }

        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum StringLiteral {
    SingleLine(String),
    MultiLine(String),
    Raw(String),
}

impl ToString for StringLiteral {
    fn to_string(&self) -> String {
        match self {
            StringLiteral::SingleLine(s) => s.clone(),
            StringLiteral::MultiLine(s) => s.clone(),
            StringLiteral::Raw(s) => s.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_escape() {
        let escapes = [
            r"\ue000",
            r"\ue000",
            r"\a",
            r"\b",
            r"\e",
            r"\f",
            r#"\\"#,
            r"\n",
            r#"\""#,
            r"\x65",
            r"\U0010ffff",
        ];
        for e in escapes {
            let str = handle_escape(e, false);
            println!("{}", str);
        }
    }
}
