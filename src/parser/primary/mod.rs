use pest::iterators::Pair;

use crate::{
    ast::SapAST,
    error_diag::{SapDiagnosticSpan, SapParserError},
};

use super::{Rule, expr::parse_expr, literal::parse_literal, pattern::parse_pattern, pratt_parser};

pub mod id;
pub mod lambda_expr;

fn parse_op_expr_child(pair: Pair<Rule>) -> Result<SapAST, SapParserError> {
    let span = SapDiagnosticSpan::from_pest_span(&pair.as_span());
    match pair.as_rule() {
        Rule::id => id::parse_id(pair),
        Rule::lambda_expr => lambda_expr::parse_lambda_expr(pair),
        Rule::literal => Ok(SapAST {
            span,
            body: crate::ast::SapASTBody::Literal(parse_literal(pair)?),
        }),
        Rule::expr => parse_expr(pair.into_inner(), pratt_parser()),
        Rule::block => {
            let mut vec = vec![];
            for expr in pair.into_inner() {
                vec.push(parse_expr(expr.into_inner(), pratt_parser())?);
            }
            Ok(SapAST {
                span,
                body: crate::ast::SapASTBody::Block(vec),
            })
        }
        _ => unimplemented!("Unhandled primary rule: {:?}", pair),
    }
}

fn parse_op_expr(pair: Pair<Rule>) -> Result<SapAST, SapParserError> {
    let span = SapDiagnosticSpan::from_pest_span(&pair.as_span());
    assert_eq!(pair.as_rule(), Rule::op_expr);
    let mut pairs = pair.into_inner();

    let first = pairs.next().expect("Primary should have a child");
    let mut res = parse_op_expr_child(first)?;

    for chain in pairs {
        let chain_op = chain
            .into_inner()
            .next()
            .expect("ChainOp should have a child");
        match chain_op.as_rule() {
            Rule::slice => {
                let inner = chain_op.into_inner();
                let mut body = (Box::new(res), None, None, None);
                for i in inner {
                    match i.as_node_tag().expect("Slice should have a tag") {
                        "from" => {
                            let expr = parse_expr(i.into_inner(), pratt_parser())?;
                            body.1 = Some(Box::new(expr));
                        }
                        "to" => {
                            let expr = parse_expr(i.into_inner(), pratt_parser())?;
                            body.2 = Some(Box::new(expr));
                        }
                        "step" => {
                            let expr = parse_expr(i.into_inner(), pratt_parser())?;
                            body.3 = Some(Box::new(expr));
                        }
                        _ => unreachable!("Invalid slice rule: {:?}", i),
                    }
                }
                res = SapAST {
                    span: span.clone(),
                    body: crate::ast::SapASTBody::Slice(body.0, body.1, body.2, body.3),
                };
            }
            Rule::index => {
                let inner = chain_op
                    .into_inner()
                    .next()
                    .expect("Index should have a child");
                let expr = parse_expr(inner.into_inner(), pratt_parser())?;
                res = SapAST {
                    span: span.clone(),
                    body: crate::ast::SapASTBody::Index(Box::new(res), Box::new(expr)),
                };
            }
            Rule::access => {
                let inner = chain_op
                    .into_inner()
                    .next()
                    .expect("Access should have a child");
                let id = id::parse_id(inner)?;
                res = SapAST {
                    span: span.clone(),
                    body: crate::ast::SapASTBody::Access(Box::new(res), Box::new(id)),
                };
            }
            _ => unreachable!("Unhandled chain op rule: {:?}", chain_op),
        }
    }
    Ok(res)
}

fn parse_app_expr(pair: Pair<Rule>) -> Result<SapAST, SapParserError> {
    assert_eq!(pair.as_rule(), Rule::app);
    let span = SapDiagnosticSpan::from_pest_span(&pair.as_span());
    let mut all = pair.into_inner();
    let f = all.next().expect("App should have a function");
    let f = parse_op_expr(f)?;
    let mut params = vec![];
    for arg in all {
        let op_expr = arg
            .into_inner()
            .next()
            .expect("App should have an argument");
        let op_expr = parse_op_expr(op_expr)?;
        params.push(op_expr);
    }

    Ok(SapAST {
        span,
        body: crate::ast::SapASTBody::App(Box::new(f), params),
    })
}

pub fn parse_primary(pair: Pair<Rule>) -> Result<SapAST, SapParserError> {
    assert_eq!(pair.as_rule(), Rule::primary);
    let pair = pair
        .into_inner()
        .next()
        .expect("Primary should have a child");
    match pair.as_rule() {
        Rule::app => parse_app_expr(pair),
        Rule::op_expr => parse_op_expr(pair),
        Rule::pattern => parse_pattern(pair),
        _ => unimplemented!("Unhandled primary rule: {:?}", pair),
    }
}
