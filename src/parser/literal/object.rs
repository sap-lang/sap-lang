use pest::iterators::Pair;

use crate::{
    error_diag::{SapDiagnosticSpan, SapParserError, SapParserErrorCode},
    parser::{
        Rule,
        expr::parse_expr,
        literal::string::StringLiteral,
        pratt_parser,
        primary::{id::Id, parse_op_expr_child},
    },
};

use super::Literal;

pub fn parse_object(object_literal: Pair<Rule>) -> Result<Literal, SapParserError> {
    assert_eq!(object_literal.as_rule(), Rule::object_literal);
    let mut elems = vec![];
    for elem in object_literal.into_inner() {
        let span = SapDiagnosticSpan::from_pest_span(&elem.as_span());
        let mut kv = elem.into_inner();
        let k = kv.next().ok_or(SapParserError {
            span: span.clone(),
            code: SapParserErrorCode::InvalidKVPair,
            message: "Expected key-value pair".to_string(),
        })?;
        let v = kv.next();
        let key = parse_op_expr_child(k)?;
        let mut is_key_id = false;
        let key = match key.body {
            // id
            crate::ast::SapASTBody::Id(Id(id)) => {
                is_key_id = true;
                id
            }
            // literal string
            crate::ast::SapASTBody::Literal(literal) => match literal {
                Literal::String(s) => s.to_string(),
                _ => unreachable!("Expected string, found {:?}", literal),
            },
            _ => unreachable!("Expected id or string, found {:?}", key),
        };

        if let Some(v) = v {
            let value = parse_expr(v.into_inner(), pratt_parser())?;
            elems.push((key, value));
        } else if is_key_id {
            elems.push((key.clone(), crate::ast::SapAST {
                span: span.clone(),
                body: crate::ast::SapASTBody::Id(Id(key)),
            }));
        } else {
            elems.push((key.clone(), crate::ast::SapAST {
                span: span.clone(),
                body: crate::ast::SapASTBody::Literal(Literal::String(StringLiteral::SingleLine(
                    key,
                ))),
            }))
        }
    }
    Ok(super::Literal::Object(elems))
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::*;

    #[test]
    fn test_parse_object() {
        let inputs = [
            "{}",
            "{key: 1}",
            "{1: {\"key\": 2}}",
            "{a: 3, \"b\": 4}",
            "{a: 5, \\b -> b : 6, \"c\": {(): 7}, \"e\": 8}",
        ];

        for input in inputs {
            let mut object_literal =
                crate::parser::SapParser::parse(Rule::object_literal, input).unwrap();
            let id = object_literal.next().unwrap();
            let ast = parse_object(id).unwrap();
            println!("{:#?}", ast);
        }
    }
}
