use pest::iterators::Pair;

use crate::{
    ast::{SapAST, SapASTBody},
    error_diag::{SapDiagnosticSpan, SapParserError, SapParserErrorCode},
    parser::{PRATT_PARSER, expr::parse_expr, pattern::parse_pattern, pratt_parser},
};

use super::{Rule, id::parse_id};

fn parse_lambda_expr_child(lambda_expr: Pair<Rule>) -> Result<SapAST, SapParserError> {
    let span = SapDiagnosticSpan::from_pest_span(&lambda_expr.as_span());
    match lambda_expr.as_rule() {
        Rule::tr_lambda => {
            let mut patterns = vec![];
            let mut implicit_params: Option<Vec<SapAST>> = None;
            let mut guard = None;
            let mut body = Box::new(SapAST {
                span: span.clone(),
                body: SapASTBody::Error(SapParserError {
                    span: span.clone(),
                    code: SapParserErrorCode::InvalidLambdaExpr,
                    message: "Lambda expression has no body".to_string(),
                }),
            });
            for a in lambda_expr.into_inner() {
                match a.as_rule() {
                    Rule::pattern => {
                        patterns.push(parse_pattern(a)?);
                    }
                    Rule::implicit_params => match &mut implicit_params {
                        Some(res) => {
                            for i in a.into_inner() {
                                res.push(parse_id(i)?);
                            }
                        }
                        None => {
                            implicit_params = {
                                let mut res = vec![];
                                for i in a.into_inner() {
                                    res.push(parse_id(i)?);
                                }
                                Some(res)
                            }
                        }
                    },
                    Rule::guard => {
                        guard = Some(Box::new(parse_expr(
                            a.into_inner()
                                .next()
                                .expect("Guard should have a child")
                                .into_inner(),
                            pratt_parser(),
                        )?));
                    }
                    Rule::expr => {
                        body = Box::new(parse_expr(a.into_inner(), pratt_parser())?);
                    }

                    _ => unreachable!(),
                }
            }

            Ok(SapAST {
                span,
                body: crate::parser::SapASTBody::LambdaExpr(LambdaExpr {
                    patterns,
                    implicit_params,
                    guard,
                    body,
                }),
            })
        }
        Rule::no_param_lambda_expr => {
            let mut block = vec![];
            let exprs = lambda_expr.into_inner();
            for e in exprs {
                assert_eq!(e.as_rule(), Rule::expr);
                block.push(parse_expr(
                    e.into_inner(),
                    PRATT_PARSER.get().expect("Pratt parser not initialized"),
                )?);
            }
            let body = Box::new(SapAST {
                span: span.clone(),
                body: SapASTBody::Block(block),
            });
            Ok(SapAST {
                span,
                body: crate::parser::SapASTBody::LambdaExpr(LambdaExpr {
                    patterns: vec![],
                    implicit_params: None,
                    guard: None,
                    body,
                }),
            })
        }

        _ => unreachable!("Invalid lambda rule: {:?}", lambda_expr),
    }
}

pub fn parse_lambda_expr(lambda_expr: Pair<Rule>) -> Result<SapAST, SapParserError> {
    let lambda_expr = lambda_expr
        .into_inner()
        .next()
        .expect("Lambda expression should have a child");
    parse_lambda_expr_child(lambda_expr)
}

#[derive(Debug, PartialEq, Clone)]
pub struct LambdaExpr {
    pub patterns: Vec<SapAST>,
    pub implicit_params: Option<Vec<SapAST>>,
    pub guard: Option<Box<SapAST>>,
    pub body: Box<SapAST>,
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::*;

    #[test]
    fn test_parse_tr_lambda_expr() {
        let inputs = [
            r#"\a->a"#,
            r#"\a->\b->c"#,
            r#"\a b -> a"#,
            r#"\a b ? c -> a"#,
            r#"\a b ? c d -> a"#,
            r#"\a b ? c : d -> c"#,
        ];

        for input in inputs {
            let mut lambda = crate::parser::SapParser::parse(Rule::lambda_expr, input).unwrap();
            let id = lambda.next().unwrap();
            let ast = parse_lambda_expr(id).unwrap();
            println!("{:#?}", ast);
            // assert_eq!(ast, *expected);
        }
    }
}
