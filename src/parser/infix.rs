use pest::iterators::Pair;

use crate::{
    ast::{SapAST, SapASTBody},
    error_diag::{SapDiagnosticSpan, SapParserError, SapParserErrorCode},
};

use super::Rule;

pub fn parse_infix(
    lhs: Result<SapAST, SapParserError>,
    pair: Pair<Rule>,
    rhs: Result<SapAST, SapParserError>,
) -> Result<SapAST, SapParserError> {
    let lhs = match lhs {
        Ok(lhs) => lhs,
        Err(e) => SapAST::error(e),
    };

    let rhs = match rhs {
        Ok(rhs) => rhs,
        Err(e) => SapAST::error(e),
    };

    let span = SapDiagnosticSpan {
        start_line: lhs.span.start_line,
        start_col: lhs.span.start_col,
        start_offset: lhs.span.start_offset,
        end_line: rhs.span.end_line,
        end_col: rhs.span.end_col,
        end_offset: rhs.span.end_offset,
        source: pair.as_str().to_string(),
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

    let lhs = if pair.as_rule() == Rule::infix_assign || pair.as_rule() == Rule::infix_match_equals
    {
        match lhs.body {
            SapASTBody::Id(id) => SapAST {
                span: lhs.span.clone(),
                body: SapASTBody::Pattern(crate::parser::pattern::Pattern::Id(id)),
            },
            SapASTBody::Literal(literal) => SapAST {
                span: lhs.span.clone(),
                body: SapASTBody::Pattern(crate::parser::pattern::Pattern::Literal(literal)),
            },
            SapASTBody::Pattern(_) => lhs,
            _ => SapAST::error(SapParserError {
                span: lhs.span.clone(),
                code: SapParserErrorCode::AssignExprLHSNotAssignable,
                message: "Left side of assignment is not assignable".to_string(),
            }),
        }
    } else if let SapASTBody::Pattern(_) = lhs.body {
        SapAST::error(SapParserError {
            span: lhs.span.clone(),
            code: SapParserErrorCode::PatternShouldNotBeOperand,
            message: "Pattern should not be used as operand".to_string(),
        })
    } else {
        lhs
    };

    // rhs is not pattern
    match pair.as_rule() {
        // ----- check if left is not pattern -----
        Rule::infix_add => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::Add(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_sub => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::Sub(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_mul => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::Mul(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_div => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::Div(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_mod => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::Mod(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_eq => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::Eq(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_neq => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::Neq(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_extends => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::Extends(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_le => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::Le(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_ge => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::Ge(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_lt => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::Lt(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_gt => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::Gt(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_and => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::And(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_or => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::Or(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_bit_or => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::BitOr(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_bit_and => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::BitAnd(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_bit_xor => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::BitXor(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_bit_shift_l => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::BitShiftL(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_bit_shift_r => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::BitShiftR(Box::new(lhs), Box::new(rhs)),
        }),
        Rule::infix_function => {
            let id = pair.into_inner().next().unwrap();
            let id = crate::parser::primary::id::parse_id(id)?;
            Ok(SapAST {
                span: span.clone(),
                body: SapASTBody::App(Box::new(id), vec![lhs, rhs]),
            })
        }
        Rule::infix_assign_get_cont => {
            let cont_id = pair.into_inner().next().unwrap();
            let cont_id = crate::parser::primary::id::parse_id(cont_id)?;
            Ok(SapAST {
                span: span.clone(),
                body: SapASTBody::AssignGetCont(Box::new(lhs), Box::new(cont_id), Box::new(rhs)),
            })
        }
        Rule::infix_pipe => {
            if let SapASTBody::App(f, mut params) = rhs.body {
                params.push(lhs);
                Ok(SapAST {
                    span: span.clone(),
                    body: SapASTBody::App(f, params),
                })
            } else {
                Ok(SapAST {
                    span: span.clone(),
                    body: SapASTBody::App(Box::new(rhs), vec![lhs]),
                })
            }
        }
        // ----- end check if left is not pattern -----
        Rule::infix_assign => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::Assign(Box::new(lhs), Box::new(rhs)),
        }),

        Rule::infix_match_equals => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::MatchEquals(Box::new(lhs), Box::new(rhs)),
        }),

        Rule::infix_assign_slot => Ok(SapAST {
            span: span.clone(),
            body: SapASTBody::AssignSlot(Box::new(lhs), Box::new(rhs)),
        }),

        _ => unimplemented!("Unhandled infix rule: {:?}", pair),
    }
}
