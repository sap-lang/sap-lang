use serde::Serialize;

use crate::parser::Rule;

// parse_int is a helper, so this can directly take number rule
fn parse_int(pair: pest::iterators::Pair<Rule>) -> Number {
    match pair.as_rule() {
        Rule::bin_int => {
            let num = pair.as_str();
            let num = &num[2..];
            let num = i64::from_str_radix(num, 2).unwrap();
            Number::Int(num)
        }
        Rule::oct_int => {
            let num = pair.as_str();
            let num = &num[2..];
            let num = i64::from_str_radix(num, 8).unwrap();
            Number::Int(num)
        }
        Rule::dec_int => {
            let num = pair.as_str();
            let num = num.parse::<i64>().unwrap();
            Number::Int(num)
        }
        Rule::hex_int => {
            let num = pair.as_str();
            let num = &num[2..];
            let num = i64::from_str_radix(num, 16).unwrap();
            Number::Int(num)
        }
        _ => unreachable!("Not an int"),
    }
}

fn parse_float(pair: pest::iterators::Pair<Rule>) -> Number {
    let pair = pair.into_inner().next().expect("Float should have a child");
    match pair.as_rule() {
        Rule::float1 => {
            let mut float1s = pair.into_inner();
            let int = float1s
                .next()
                .map(parse_int)
                .expect("Float1 should have an int");
            let exp = float1s
                .next()
                .expect("Float1 should have an exp")
                .into_inner()
                .next()
                .map(parse_int)
                .expect("Float1 exp should have a child");

            let num = int.as_int() as f64 * 10.0_f64.powi(exp.as_int() as i32);
            Number::Float(num)
        }
        Rule::float2 => {
            let mut float2s = pair.into_inner();
            let int = float2s
                .next()
                .map(parse_int)
                .expect("Float2 should have an int")
                .as_int();
            let frac = float2s
                .next()
                .map(parse_int)
                .expect("Float2 should have an int")
                .as_int();
            let exp = float2s
                .next()
                .map(|exp| {
                    exp.into_inner()
                        .next()
                        .expect("Float2 exp should have a child")
                })
                .map(|exp| parse_int(exp))
                .unwrap_or(Number::Int(0))
                .as_int();

            let num = (int as f64 + frac as f64 * 10.0_f64.powi(-(frac.to_string().len() as i32)))
                * 10.0_f64.powi(exp as i32);
            Number::Float(num)
        }
        Rule::float3 => {
            let mut float3s = pair.into_inner();
            let frac = float3s
                .next()
                .map(parse_int)
                .expect("Float1 should have an int");
            let exp = float3s
                .next()
                .map(|exp| {
                    exp.into_inner()
                        .next()
                        .expect("Float2 exp should have a child")
                })
                .map(|exp| parse_int(exp))
                .unwrap_or(Number::Int(0));

            let num = frac.as_int() as f64 / 10.0_f64.powi(frac.as_int().to_string().len() as i32)
                * 10.0_f64.powi(exp.as_int() as i32);
            Number::Float(num)
        }
        _ => unreachable!(),
    }
}

pub fn parse_number(pair: pest::iterators::Pair<Rule>) -> Number {
    let pair = pair
        .into_inner()
        .next()
        .expect("Number should have a child");
    match pair.as_rule() {
        Rule::bin_int | Rule::oct_int | Rule::dec_int | Rule::hex_int => parse_int(pair),
        Rule::float => parse_float(pair),

        Rule::bigint => unimplemented!("BigInt"),
        _ => unreachable!(),
    }
}
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Number {
    Int(i64),
    Float(f64),
    // BigInt(BigInt),
}

impl Number {
    fn as_int(&self) -> i64 {
        match self {
            Number::Int(num) => *num,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_parse_int() {
        let pairs = crate::parser::SapParser::parse(Rule::dec_int, "123").unwrap();
        let number = parse_int(pairs.into_iter().next().unwrap());
        assert_eq!(number, Number::Int(123));

        let pairs = crate::parser::SapParser::parse(Rule::bin_int, "0b111").unwrap();
        let number = parse_int(pairs.into_iter().next().unwrap());
        assert_eq!(number, Number::Int(7));

        let pairs = crate::parser::SapParser::parse(Rule::oct_int, "0o77").unwrap();
        let number = parse_int(pairs.into_iter().next().unwrap());
        assert_eq!(number, Number::Int(63));

        let pairs = crate::parser::SapParser::parse(Rule::hex_int, "0x7F").unwrap();
        let number = parse_int(pairs.into_iter().next().unwrap());
        assert_eq!(number, Number::Int(127));
    }

    #[test]
    fn test_parse_float() {
        let pairs = crate::parser::SapParser::parse(Rule::float, "1.23e3").unwrap();
        let number = parse_float(pairs.into_iter().next().unwrap());
        assert_eq!(number, Number::Float(1230.0));

        let pairs = crate::parser::SapParser::parse(Rule::float, "1.23").unwrap();
        let number = parse_float(pairs.into_iter().next().unwrap());
        assert_eq!(number, Number::Float(1.23));

        let pairs = crate::parser::SapParser::parse(Rule::float, ".23e2").unwrap();
        let number = parse_float(pairs.into_iter().next().unwrap());
        assert_eq!(number, Number::Float(23.0));
    }

    #[test]
    fn test_parse_number() {
        let pairs = crate::parser::SapParser::parse(Rule::number, "123").unwrap();
        let number = parse_number(pairs.into_iter().next().unwrap());
        assert_eq!(number, Number::Int(123));

        let pairs = crate::parser::SapParser::parse(Rule::number, "1.23e3").unwrap();
        let number = parse_number(pairs.into_iter().next().unwrap());
        assert_eq!(number, Number::Float(1230.0));
    }
}
