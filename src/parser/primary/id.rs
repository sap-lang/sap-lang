use pest::iterators::Pair;
use serde::Serialize;

use crate::{
    ast::{SapAST, SapASTBody},
    error_diag::{SapDiagnosticSpan, SapParserError},
};

use super::Rule;

fn parse_id_child(id: Pair<Rule>) -> Result<SapAST, SapParserError> {
    match id.as_rule() {
        Rule::normal_id => Ok(SapAST {
            span: SapDiagnosticSpan::from_pest_span(&id.as_span()),
            body: SapASTBody::Id(Id(id.as_str().to_string())),
        }),

        Rule::magic_fn_id => Ok(SapAST {
            span: SapDiagnosticSpan::from_pest_span(&id.as_span()),
            body: SapASTBody::Id(Id(id.as_str().to_string())),
        }),

        Rule::macro_id => Ok(SapAST {
            span: SapDiagnosticSpan::from_pest_span(&id.as_span()),
            body: SapASTBody::Macro(Id(id.as_str().to_string())),
        }),
        _ => unreachable!("Invalid id rule: {:?}", id),
    }
}

pub fn parse_id(id: Pair<Rule>) -> Result<SapAST, SapParserError> {
    let id = id.into_inner().next().expect("Id should have a child");
    parse_id_child(id)
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Id(pub String);

#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::*;

    #[test]
    fn test_parse_normal_id() {
        let inputs = [
            "foo",
            "foo   ",
            "foo\t",
            "_bar",
            "你好召唤师",
            "Γειά",
            "こんにちは",
            "สวัสดีชาวโลก",
            "안녕하세요",
            "مرحبا",
            "🖖",
        ];

        for input in inputs {
            let mut id = crate::parser::SapParser::parse(Rule::id, input).unwrap();
            let id = id.next().unwrap();
            let ast = parse_id(id).unwrap();
            match ast.body {
                SapASTBody::Id(Id(s)) => assert_eq!(s, input.trim()),
                _ => panic!("Expected Id, found {:?}", ast.body),
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_id() {
        let mut id = crate::parser::SapParser::parse(Rule::id, "_").unwrap();
        let id = id.next().unwrap().into_inner().next().unwrap();
        let ast = parse_id_child(id).unwrap();
        println!("{:#?}", ast);
    }
}
