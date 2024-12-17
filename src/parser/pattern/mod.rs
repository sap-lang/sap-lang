pub mod array;
pub mod object;

use array::parse_array_pattern;
use pest::iterators::Pair;

use crate::{
    ast::{SapAST, SapASTBody},
    error_diag::{SapDiagnosticSpan, SapParserError, SapParserErrorCode},
};

use super::{literal::{parse_literal, Literal}, primary::id::{parse_id, Id}, Rule};

pub fn parse_pattern(pattern: Pair<Rule>) -> Result<SapAST, SapParserError> {
    let pattern = pattern.into_inner().next().expect("Pattern should have a child");
    parse_pattern_child(pattern)
}

fn parse_pattern_child(pattern: Pair<Rule>) -> Result<SapAST, SapParserError> {
    let span = SapDiagnosticSpan::from_pest_span(&pattern.as_span());
    match pattern.as_rule() {
        Rule::id => {
            parse_id(pattern).map(|id| {
                SapAST {
                    span: id.span.clone(),
                    body: SapASTBody::Pattern(Pattern::Id(id.get_id())),
                }
            })
        }
        Rule::array_pattern => {
            parse_array_pattern(pattern)
        }
        // Rule::object_pattern => {
        //     // 解析 object_pattern 的逻辑
        //     // ...
        // }
        Rule::literal => {
            parse_literal(pattern).map(|literal| {
                SapAST {
                    span,
                    body: SapASTBody::Pattern(Pattern::Literal(literal)),
                }
            })
        }
        _ => unreachable!("Invalid pattern rule: {:?}", pattern),
    }
}

pub fn parse_eclipse_pattern(pattern: Pair<Rule>) -> Result<EclipsePattern, SapParserError> {
    let id = pattern.into_inner().next().expect("Eclipse pattern should have an id");
    parse_id(id).map(|id| {
        EclipsePattern{id: id.get_id()}
    })
}

#[derive(Debug, PartialEq)]
pub struct EclipsePattern {
    pub id: Id,
}

#[derive(Debug, PartialEq)]
pub enum ObjectInner {
    KV(SapAST, SapAST),
    Eclipse(EclipsePattern),
}

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Id(Id),
    Eclipse(EclipsePattern),
    Array(Vec<SapAST>),
    Object(Vec<ObjectInner>),
    Literal(Literal),
}
