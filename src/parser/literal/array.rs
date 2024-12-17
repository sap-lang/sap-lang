use pest::iterators::Pair;

use crate::{
    ast::{SapAST, SapASTBody},
    error_diag::{SapDiagnosticSpan, SapParserError},
    parser::{Rule, expr::parse_expr, pratt_parser},
};

use super::Literal;

pub fn parse_array(array_literal: Pair<Rule>) -> Result<Literal, SapParserError> {
    assert_eq!(array_literal.as_rule(), Rule::array_literal);
    let mut elems = vec![];
    for elem in array_literal.into_inner() {
        elems.push(parse_expr(elem.into_inner(), pratt_parser())?);
    }
    Ok(super::Literal::Array(elems))
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::*;

    #[test]
    fn test_parse_array() {
        let inputs = ["[]", "[1]", "[[2]]", "[3, 4]", "[5, 6, [7], 8]"];

        for input in inputs {
            let mut array_literal =
                crate::parser::SapParser::parse(Rule::array_literal, input).unwrap();
            let id = array_literal.next().unwrap();
            let ast = parse_array(id).unwrap();
            println!("{:#?}", ast);
        }
    }
}
