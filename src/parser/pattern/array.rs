use pest::iterators::Pair;

use crate::{
    ast::{SapAST, SapASTBody},
    error_diag::{SapDiagnosticSpan, SapParserError},
    parser::Rule,
};

use super::{Pattern, parse_eclipse_pattern, parse_pattern};

fn parse_array_pattern_elem(elem: Pair<Rule>) -> Result<SapAST, SapParserError> {
    let span = SapDiagnosticSpan::from_pest_span(&elem.as_span());
    match elem.as_rule() {
        Rule::pattern => parse_pattern(elem),
        Rule::eclipse_pattern => parse_eclipse_pattern(elem).map(|eclipse_pattern| SapAST {
            span,
            body: SapASTBody::Pattern(Pattern::Eclipse(eclipse_pattern)),
        }),
        _ => unreachable!("Invalid array pattern element rule: {:?}", elem),
    }
}

pub fn parse_array_pattern(array_pattern: Pair<Rule>) -> Result<SapAST, SapParserError> {
    let span = SapDiagnosticSpan::from_pest_span(&array_pattern.as_span());
    let mut elems = vec![];
    for elem in array_pattern.into_inner() {
        assert_eq!(elem.as_rule(), Rule::array_pattern_elem);
        let elem = elem
            .into_inner()
            .next()
            .expect("Array pattern element should have a child");
        elems.push(parse_array_pattern_elem(elem)?);
    }
    Ok(SapAST {
        span,
        body: SapASTBody::Pattern(Pattern::Array(elems)),
    })
}


#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::*;

    #[test]
    fn test_parse_array_pattern() {
        let inputs = ["[]", "[foo]", "[[b]]", "[a, ...b]", "[a, ...b, [...c], d]"];

        for input in inputs {
            let mut array_pattern =
                crate::parser::SapParser::parse(Rule::array_pattern, input).unwrap();
            let id = array_pattern.next().unwrap();
            let ast = parse_array_pattern(id).unwrap();
            println!("{:#?}", ast);
        }
    }
}
