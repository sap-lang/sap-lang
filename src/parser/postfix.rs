use pest::iterators::Pair;

use crate::{ast::SapAST, error_diag::SapParserError};

use super::Rule;

pub fn parse_postfix(
    lhs: Result<SapAST, SapParserError>,
    pair: Pair<Rule>,
) -> Result<SapAST, SapParserError> {
    // check if left is pattern
    let lhs = match lhs {
        Ok(lhs) => lhs,
        Err(e) => SapAST::error(e),
    };

    match pair.as_rule() {
        Rule::postfix_bang => todo!(),
        _ => unimplemented!("Unhandled postfix rule: {:?}", pair),
    }
    todo!()
}
