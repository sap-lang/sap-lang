use pest::iterators::Pair;

use crate::{ast::SapAST, error_diag::SapParserError};

use super::Rule;

pub fn parse_postfix(
    _lhs: Result<SapAST, SapParserError>,
    _pair: Pair<Rule>,
) -> Result<SapAST, SapParserError> {
    todo!()
}
