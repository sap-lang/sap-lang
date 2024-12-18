use pest::iterators::Pair;

use crate::{
    ast::{SapAST, SapASTBody},
    error_diag::{SapDiagnosticSpan, SapParserError, SapParserErrorCode},
    parser::{
        Rule,
        literal::string::parse_string,
        pattern::{ObjectInner, parse_eclipse_pattern, parse_pattern},
        primary::id::parse_id,
    },
};

use super::Pattern;
pub fn parse_object_pattern(object_literal: Pair<Rule>) -> Result<Pattern, SapParserError> {
    assert_eq!(object_literal.as_rule(), Rule::object_pattern);
    let mut elems = vec![];
    for elem in object_literal.into_inner() {
        let span = SapDiagnosticSpan::from_pest_span(&elem.as_span());
        let mut kv = elem.into_inner();

        let k = kv.next().ok_or(SapParserError {
            span: span.clone(),
            code: SapParserErrorCode::InvalidKVPair,
            message: "Expected key-value pair".to_string(),
        })?;

        if let Rule::eclipse_pattern = k.as_rule() {
            elems.push(ObjectInner::Eclipse(parse_eclipse_pattern(k)?));
            continue;
        }

        let v = kv.next().ok_or(SapParserError {
            span,
            code: SapParserErrorCode::InvalidKVPair,
            message: "Expected key-value pair".to_string(),
        })?;

        let k_span = SapDiagnosticSpan::from_pest_span(&k.as_span());
        let key = match k.as_rule() {
            Rule::id => parse_id(k).map(|id| SapAST {
                span: k_span.clone(),
                body: SapASTBody::Pattern(super::Pattern::Id(id.get_id())),
            })?,
            Rule::normal_string_inner | Rule::raw_string_inner | Rule::multiline_string_inner => {
                let inner_string = parse_string(k).to_string();
                SapAST {
                    span: k_span.clone(),
                    body: SapASTBody::Pattern(super::Pattern::Id(crate::parser::primary::id::Id(
                        inner_string,
                    ))),
                }
            }
            _ => unreachable!("Invalid key rule: {:?}", k),
        };

        let value = parse_pattern(v)?;
        elems.push(ObjectInner::KV(key, value));
    }
    Ok(super::Pattern::Object(elems))
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::*;

    #[test]
    fn test_parse_object() {
        let inputs = ["{}", "{key: 1}", "{a: 3, \"b\": 4}", "{...p}"];

        for input in inputs {
            let mut object_literal =
                crate::parser::SapParser::parse(Rule::object_pattern, input).unwrap();
            let id = object_literal.next().unwrap();
            let ast = parse_object_pattern(id).unwrap();
            println!("{:#?}", ast);
        }
    }
}
