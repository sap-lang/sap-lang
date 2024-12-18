use number::Number;
use string::StringLiteral;

use crate::{
    ast::SapAST,
    error_diag::SapParserError,
    parser::Rule,
};

pub mod array;
pub mod number;
pub mod object;
pub mod string;

fn parse_boolean(pair: pest::iterators::Pair<Rule>) -> Literal {
    match pair.as_rule() {
        Rule::boolean_true => Literal::Boolean(true),
        Rule::boolean_false => Literal::Boolean(false),
        _ => unreachable!(),
    }
}

fn parse_null(pair: pest::iterators::Pair<Rule>) -> Literal {
    match pair.as_rule() {
        Rule::null => Literal::Null,
        _ => unreachable!(),
    }
}

fn parse_undefined(pair: pest::iterators::Pair<Rule>) -> Literal {
    match pair.as_rule() {
        Rule::undefined => Literal::Undefined,
        _ => unreachable!(),
    }
}

fn parse_void(pair: pest::iterators::Pair<Rule>) -> Literal {
    match pair.as_rule() {
        Rule::void => Literal::Void,
        _ => unreachable!(),
    }
}

fn parse_slot(pair: pest::iterators::Pair<Rule>) -> Literal {
    match pair.as_rule() {
        Rule::slot => Literal::Slot,
        _ => unreachable!(),
    }
}

fn parse_literal_child(pair: pest::iterators::Pair<Rule>) -> Result<Literal, SapParserError> {
    Ok(match pair.as_rule() {
        Rule::boolean => parse_boolean(pair),
        Rule::null => parse_null(pair),
        Rule::undefined => parse_undefined(pair),
        Rule::void => parse_void(pair),
        Rule::slot => parse_slot(pair),
        Rule::number => Literal::Number(number::parse_number(pair)),
        Rule::normal_string_inner | Rule::multiline_string_inner | Rule::raw_string_inner => {
            Literal::String(string::parse_string(pair))
        }
        Rule::array_literal => array::parse_array(pair)?,
        Rule::object_literal => object::parse_object(pair)?,
        _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
    })
}

pub fn parse_literal(pair: pest::iterators::Pair<Rule>) -> Result<Literal, SapParserError> {
    assert_eq!(pair.as_rule(), Rule::literal);
    parse_literal_child(
        pair.into_inner()
            .next()
            .expect("Literal should have a child"),
    )
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Null,
    Undefined,
    Void,
    Slot,
    Boolean(bool),
    Number(Number),
    String(StringLiteral),
    Array(Vec<SapAST>),
    Object(Vec<(SapAST, SapAST)>),
}
