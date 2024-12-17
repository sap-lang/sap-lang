use pest::{iterators::Pairs, pratt_parser::PrattParser};

use crate::{
    ast::SapAST,
    error_diag::SapParserError,
    parser::{infix, postfix, prefix, primary},
};

use super::Rule;

pub fn parse_expr(pairs: Pairs<Rule>, pratt: &PrattParser<Rule>) -> Result<SapAST, SapParserError> {
    pratt
        .map_primary(primary::parse_primary)
        .map_prefix(prefix::parse_prefix)
        .map_postfix(postfix::parse_postfix)
        .map_infix(infix::parse_infix)
        .parse(pairs)
}
