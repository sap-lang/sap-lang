use pest::iterators::Pair;

use crate::{ast::{SapAST, SapASTBody}, error_diag::{SapDiagnosticSpan, SapParserError, SapParserErrorCode}};

use super::Rule;

pub fn parse_prefix(
    pair: Pair<Rule>,
    rhs: Result<SapAST, SapParserError>,
) -> Result<SapAST, SapParserError> {
    let span = SapDiagnosticSpan::from_pest_span(&pair.as_span());
    // check if right is pattern
    let rhs = match rhs {
        Ok(rhs) => rhs,
        Err(e) => SapAST::error(e),
    };

    let rhs = if let SapASTBody::Pattern(_) = rhs.body {
        SapAST::error(SapParserError {
            span: rhs.span.clone(),
            code: SapParserErrorCode::PatternShouldNotBeOperand,
            message: "Pattern should not be used as operand".to_string(),
        })
    } else {
        rhs
    };

    match pair.as_rule() {
        Rule::prefix_typeof => Ok(SapAST{
            span: span.clone(),
            body: SapASTBody::Typeof(Box::new(rhs)),
        }),
        Rule::prefix_not => Ok(SapAST{
            span: span.clone(),
            body: SapASTBody::Not(Box::new(rhs)),
        }),
        Rule::prefix_neg => Ok(SapAST{
            span: span.clone(),
            body: SapASTBody::Neg(Box::new(rhs)),
        }),
        Rule::prefix_bit_not => Ok(SapAST{
            span: span.clone(),
            body: SapASTBody::BitNot(Box::new(rhs)),
        }),
        Rule::prefix_yield => Ok(SapAST{
            span: span.clone(),
            body: SapASTBody::Yield(Box::new(rhs)),
        }),
        _ => unimplemented!("Unhandled prefix rule: {:?}", pair),
    }
}
