//! from this pass, pattern matching is transformed into if-else statements
//! and raw pattern is transformed into AST nodes
use std::collections::{BTreeMap, BTreeSet};

use crate::{
    def_pass_with_metainfo,
    error_reporting::{
        CompilePrompt, CompilePromptCode, CompilePromptLabel, RuntimeErrors, push_prompt,
    },
    simple_literal::SimpleLiteral,
};

use super::{
    flatten::{Flatten, FlattenLambdaExpr, Inner as FlattenInner},
    uuid,
};
use crate::simple_pattern::SimplePattern as Pattern;
#[derive(Debug, Clone)]
pub struct TrivializePatternLambdaExpr {
    // guard and patterns become if-else statements befor body
    pub params: Vec<String>,
    pub implicit_params: Option<Vec<String>>,
    pub body: Vec<TrivializePattern>,
}

impl TrivializePatternLambdaExpr {
    pub fn from_flatten(
        FlattenLambdaExpr {
            patterns,
            implicit_params,
            guard,
            body,
        }: FlattenLambdaExpr,
        prompt_label: &mut Vec<CompilePromptLabel>,
    ) -> TrivializePatternLambdaExpr {
        let mut exprs = vec![];
        let mut ids = vec![];
        for p in patterns {
            let id = uuid();
            ids.push(id.clone());
            let eid = TrivializePattern::Id(id.clone(), body.info);
            let np = pattern_to_ifs(p, eid, RuntimeErrors::FunctionGuardOrMatchGuardFailed);
            exprs.extend(np);
        }
        if let Some(guard) = guard {
            let mut t = TrivializePattern::from_flatten(*guard, prompt_label);
            let tt = t.pop().unwrap();
            exprs.extend(t);
            exprs.push(assert(
                Box::new(tt),
                RuntimeErrors::FunctionGuardOrMatchGuardFailed,
            ));
        }
        let t = TrivializePattern::from_flatten(*body, prompt_label);
        exprs.extend(t);

        TrivializePatternLambdaExpr {
            params: ids,
            implicit_params,
            body: exprs,
        }
    }
}

pub fn assert(expr: Box<TrivializePattern>, error: RuntimeErrors) -> TrivializePattern {
    let info = expr.info;
    // if expr return void else return error
    TrivializePattern::Assert(
        expr,
        Box::new(TrivializePattern::RuntimeError(error, info)),
        info,
    )
}

def_pass_with_metainfo! {
    TrivializePattern {
        LambdaExpr(lambda_expr: TrivializePatternLambdaExpr),
        IfThenElse(cond: Box<TrivializePattern>, true_expr: Vec<TrivializePattern>, false_expr: Vec<TrivializePattern>),

        RuntimeError(error: RuntimeErrors),

        Assert(cond: Box<TrivializePattern>, false_expr: Box<TrivializePattern>),

        Array(elems: Vec<String>),
        Object(body: Vec<(String, String)>),

        Literal(literal: SimpleLiteral),
        Id(id: String),
        Yield(yielding: Box<TrivializePattern>),
        Access(id: String, expr: Box<TrivializePattern>),
        Extend(base: Box<TrivializePattern>, parent: Box<TrivializePattern>),
        FindAndCallWithThis(calling: Box<TrivializePattern>, id: Box<TrivializePattern>),
        Apply(call: Box<TrivializePattern>, args: Vec<TrivializePattern>),

        Assign(left: String, right: Box<TrivializePattern>),

        // a = 1
        Set(left: String, right: Box<TrivializePattern>),
        // a[0] := 1
        SetUpdate(left: Box<TrivializePattern>, right: Box<TrivializePattern>),
        // a.a := 1
        SetInner(left: Box<TrivializePattern>, right: Box<TrivializePattern>),

        // a ::= \a -> a
        AssignSlot(leftslot: String, right: Box<TrivializePattern>),
        // only access
        AssignInnerSlot(leftslot: Box<TrivializePattern>, right: Box<TrivializePattern>),
    }
}

impl TrivializePattern {
    pub fn from_flatten(
        flatten: Flatten,
        prompt_label: &mut Vec<CompilePromptLabel>,
    ) -> Vec<TrivializePattern> {
        let mut prevs = vec![];
        let Flatten { inner, info } = flatten;
        match inner {
            FlattenInner::Block(flattens) => {
                for flatten in flattens {
                    prevs.extend(Self::from_flatten(flatten, prompt_label));
                }
            }
            FlattenInner::LambdaExpr(flatten_lambda_expr) => {
                let t = TrivializePattern::LambdaExpr(
                    TrivializePatternLambdaExpr::from_flatten(flatten_lambda_expr, prompt_label),
                    info,
                );
                prevs.push(t);
            }
            FlattenInner::IfThenElse(flatten, flatten1, flatten2) => {
                if let FlattenInner::MatchEquals(x, y) = flatten.inner {
                    flatten_pattern_if(
                        *x,
                        *y,
                        TrivializePattern::from_flatten(*flatten1, prompt_label)
                            .pop()
                            .unwrap(),
                        TrivializePattern::from_flatten(*flatten2, prompt_label)
                            .pop()
                            .unwrap(),
                        info,
                        prompt_label,
                        &mut prevs,
                    );
                } else {
                    let mut t = TrivializePattern::from_flatten(*flatten, prompt_label);
                    let tt = t.pop().unwrap();
                    prevs.extend(t);
                    let te = TrivializePattern::from_flatten(*flatten1, prompt_label);
                    let fe = TrivializePattern::from_flatten(*flatten2, prompt_label);
                    TrivializePattern::IfThenElse(Box::new(tt), te, fe, info);
                }
            }
            FlattenInner::Array(flattens) => {
                let mut elems = vec![];
                for flatten in flattens {
                    let mut t = Self::from_flatten(flatten, prompt_label);
                    let tt = t.pop().unwrap();
                    prevs.extend(t);
                    let id = uuid();
                    let tt = TrivializePattern::Assign(id.clone(), Box::new(tt), info);
                    prevs.push(tt);
                    elems.push(id);
                }
                let t = TrivializePattern::Array(elems, info);
                prevs.push(t);
            }
            FlattenInner::Object(items) => {
                let mut elems = vec![];
                for (key, flatten) in items {
                    let mut t = Self::from_flatten(flatten, prompt_label);
                    let tt = t.pop().unwrap();
                    prevs.extend(t);
                    let id = uuid();
                    let tt = TrivializePattern::Assign(id.clone(), Box::new(tt), info);
                    prevs.push(tt);
                    elems.push((key, id));
                }
                let t = TrivializePattern::Object(elems, info);
                prevs.push(t);
            }
            FlattenInner::Pattern(_) => {
                prompt_label.push(CompilePromptLabel::new(
                    CompilePromptCode::PatternShouldBeUsed,
                    info,
                ));
                prevs.push(TrivializePattern::RuntimeError(
                    RuntimeErrors::CustomError("unsupport".to_string()),
                    info,
                ));
            }
            FlattenInner::Literal(simple_literal) => {
                let t = TrivializePattern::Literal(simple_literal, info);
                prevs.push(t);
            }
            FlattenInner::Id(x) => {
                let t = TrivializePattern::Id(x, info);
                prevs.push(t);
            }
            FlattenInner::MacroId(_) => {
                prompt_label.push(CompilePromptLabel::new(
                    CompilePromptCode::Sap0DoesNotSupportMacro,
                    info,
                ));
                prevs.push(TrivializePattern::RuntimeError(
                    RuntimeErrors::CustomError("unsupport".to_string()),
                    info,
                ));
            }
            FlattenInner::Yield(flatten) => {
                let mut t = Self::from_flatten(*flatten, prompt_label);
                let tt = t.pop().unwrap();
                prevs.extend(t);
                let t = TrivializePattern::Yield(Box::new(tt), info);
                prevs.push(t);
            }
            FlattenInner::Access(x, flatten) => {
                let mut t = Self::from_flatten(*flatten, prompt_label);
                let tt = t.pop().unwrap();
                prevs.extend(t);
                let t = TrivializePattern::Access(x, Box::new(tt), info);
                prevs.push(t);
            }
            FlattenInner::Extend(flatten, flatten1) => {
                let mut t = Self::from_flatten(*flatten, prompt_label);
                let tt1 = t.pop().unwrap();
                prevs.extend(t);
                let mut t = Self::from_flatten(*flatten1, prompt_label);
                let tt2 = t.pop().unwrap();
                prevs.extend(t);
                let t = TrivializePattern::Extend(Box::new(tt1), Box::new(tt2), info);
                prevs.push(t);
            }
            FlattenInner::FindAndCallWithThis(flatten, flatten1) => {
                let mut t = Self::from_flatten(*flatten, prompt_label);
                let tt1 = t.pop().unwrap();
                prevs.extend(t);
                let mut t = Self::from_flatten(*flatten1, prompt_label);
                let tt2 = t.pop().unwrap();
                prevs.extend(t);
                let t = TrivializePattern::FindAndCallWithThis(Box::new(tt1), Box::new(tt2), info);
                prevs.push(t);
            }
            FlattenInner::Apply(flatten, flattens) => {
                let mut t = Self::from_flatten(*flatten, prompt_label);
                let tt = t.pop().unwrap();
                prevs.extend(t);
                let mut args = vec![];
                for flatten in flattens {
                    let mut t = Self::from_flatten(flatten, prompt_label);
                    let tt = t.pop().unwrap();
                    prevs.extend(t);
                    args.push(tt);
                }
                let t = TrivializePattern::Apply(Box::new(tt), args, info);
                prevs.push(t);
            }

            FlattenInner::MatchEquals(flatten, flatten1) => {
                flatten_pattern_if(
                    *flatten,
                    *flatten1,
                    TrivializePattern::Literal(SimpleLiteral::Bool(true), info),
                    TrivializePattern::Literal(SimpleLiteral::Bool(false), info),
                    info,
                    prompt_label,
                    &mut prevs,
                );
            }

            FlattenInner::Assign(flatten, flatten1) => {
                // assign pattern
                // assign id
                match flatten.inner {
                    FlattenInner::Pattern(p) => {
                        let mut t = Self::from_flatten(*flatten1, prompt_label);
                        let tt = t.pop().unwrap();

                        let pt = pattern_to_ifs(p, tt, RuntimeErrors::DestructorPatternMatchFailed);
                        prevs.extend(pt);
                    }
                    FlattenInner::Id(id) => {
                        let mut t = Self::from_flatten(*flatten1, prompt_label);
                        let tt = t.pop().unwrap();
                        prevs.extend(t);
                        let t = TrivializePattern::Assign(id, Box::new(tt), info);
                        prevs.push(t);
                    }
                    _ => {
                        prompt_label.push(CompilePromptLabel::new(
                            CompilePromptCode::ExpectedValidLHS,
                            info,
                        ));
                        prevs.push(TrivializePattern::RuntimeError(
                            RuntimeErrors::CustomError("unsupport".to_string()),
                            info,
                        ));
                    }
                }
            }
            FlattenInner::Set(flatten, flatten1) => match flatten.inner {
                FlattenInner::Access(x, y) => todo!(),
                FlattenInner::Apply(id, y) => {
                    if let FlattenInner::Id(str) = id.inner
                        && str == "([])"
                    {
                        todo!()
                    } else {
                        prompt_label.push(CompilePromptLabel::new(
                            CompilePromptCode::SetShouldOnlyBeUsedForAccess,
                            info,
                        ));
                        prevs.push(TrivializePattern::RuntimeError(
                            RuntimeErrors::CustomError("unsupport".to_string()),
                            info,
                        ));
                    }
                }
                FlattenInner::Id(id) => {
                    let mut t = Self::from_flatten(*flatten1, prompt_label);
                    let tt = t.pop().unwrap();
                    prevs.extend(t);
                    let t = TrivializePattern::Set(id, Box::new(tt), info);
                    prevs.push(t);
                }
                _ => {
                    prompt_label.push(CompilePromptLabel::new(
                        CompilePromptCode::ExpectedValidLHS,
                        info,
                    ));
                    prevs.push(TrivializePattern::RuntimeError(
                        RuntimeErrors::CustomError("unsupport".to_string()),
                        info,
                    ));
                }
            },
            FlattenInner::AssignSlot(flatten, flatten1) => match flatten.inner {
                FlattenInner::Id(id) => {
                    let mut t = Self::from_flatten(*flatten1, prompt_label);
                    let tt = t.pop().unwrap();
                    prevs.extend(t);
                    let t = TrivializePattern::AssignSlot(id, Box::new(tt), info);
                    prevs.push(t);
                }
                FlattenInner::Access(x, y) => todo!(),
                _ => {
                    prompt_label.push(CompilePromptLabel::new(
                        CompilePromptCode::ExpectedValidLHS,
                        info,
                    ));
                    prevs.push(TrivializePattern::RuntimeError(
                        RuntimeErrors::CustomError("unsupport".to_string()),
                        info,
                    ));
                }
            },
        }
        prevs
    }
}

fn flatten_pattern_if(
    flatten: Flatten,
    flatten1: Flatten,
    true_expr: TrivializePattern,
    false_expr: TrivializePattern,
    info: CommonMetaInfo,
    prompt_label: &mut Vec<CompilePromptLabel>,
    prevs: &mut Vec<TrivializePattern>,
) {
    if let FlattenInner::Pattern(p) = flatten.inner {
        let mut t = TrivializePattern::from_flatten(flatten1, prompt_label);
        let tt = t.pop().unwrap();

        let pt = pattern_to_ifs(p, tt, RuntimeErrors::DestructorPatternMatchFailed);
        let pt = assert_to_flatten_if(pt, true_expr, false_expr, info);
        prevs.extend(pt);
    } else {
        prompt_label.push(CompilePromptLabel::new(
            CompilePromptCode::ExpectedPatternFindExpression,
            info,
        ));
        prevs.push(TrivializePattern::RuntimeError(
            RuntimeErrors::CustomError("unsupport".to_string()),
            info,
        ));
    }
}

fn assert_to_flatten_if(
    mut p: Vec<TrivializePattern>,
    true_expr: TrivializePattern,
    false_expr: TrivializePattern,
    info: CommonMetaInfo,
) -> Vec<TrivializePattern> {
    let mut prevs = vec![];
    for i in 0..p.len() {
        let pp = &p[i];
        if let Inner::Assert(cond, _) = pp.inner.clone() {
            p.push(true_expr.clone());
            let mut pp = prevs;
            prevs = vec![];
            pp.extend(p[i + 1..].to_vec());
            let pp = assert_to_flatten_if(pp, true_expr.clone(), false_expr.clone(), info);
            let t = TrivializePattern::IfThenElse(cond.clone(), pp, vec![false_expr.clone()], info);
            prevs.push(t);
        } else {
            prevs.push(p[i].clone());
        }
    }
    prevs
}

fn pattern_to_ifs(
    p: Pattern,
    e: TrivializePattern,
    error: RuntimeErrors,
) -> Vec<TrivializePattern> {
    let info = e.info;
    match p {
        Pattern::Id(id) => {
            let expr = TrivializePattern::Assign(id, Box::new(e), info);
            vec![expr]
        }
        Pattern::Literal(literal) => {
            let expr = TrivializePattern::Apply(
                Box::new(TrivializePattern::Id("(==)".to_string(), info)),
                vec![e, TrivializePattern::Literal(literal, info)],
                info,
            );
            let expr = assert(Box::new(expr), error.clone());
            vec![expr]
        }
        Pattern::Array(array_pattern, eclispe) => {
            let mut patterns = vec![];
            let mut i = 0;
            for elem in array_pattern.into_iter() {
                let e = TrivializePattern::Apply(
                    Box::new(TrivializePattern::Id("([])".to_string(), info)),
                    vec![
                        TrivializePattern::Literal(SimpleLiteral::Int(i as _), info),
                        e.clone(),
                    ],
                    info,
                );
                patterns.extend(pattern_to_ifs(elem, e, error.clone()));
                i += 1;
            }
            patterns.push(assert(
                Box::new(TrivializePattern::Apply(
                    Box::new(TrivializePattern::Id("(>=)".to_string(), info)),
                    vec![
                        TrivializePattern::Apply(Box::new(TrivializePattern::Id("(len)".to_string(), info)), vec![
                            e.clone()
                        ], info),
                        TrivializePattern::Literal(SimpleLiteral::Int(i as _), info),
                    ],
                    info,
                )),
                error.clone(),
            ));

            if let Some(id) = eclispe {
                let expr = TrivializePattern::Apply(
                    Box::new(TrivializePattern::Id("([::])".to_string(), info)),
                    vec![
                        TrivializePattern::Literal(SimpleLiteral::Int(i as _), info),
                        TrivializePattern::Literal(SimpleLiteral::Void, info),
                        TrivializePattern::Literal(SimpleLiteral::Void, info),
                        e.clone(),
                    ],
                    info,
                );
                let expr = TrivializePattern::Assign(id.clone(), Box::new(expr), info);
                patterns.push(expr);
            }
            patterns
        }
        Pattern::Object(object_pattern, eclipse) => {
            let mut patterns = vec![];
            for (key, elem) in object_pattern.into_iter() {
                let e = TrivializePattern::Access(key, Box::new(e.clone()), info);
                patterns.extend(pattern_to_ifs(elem, e, error.clone()));
            }
            if let Some(id) = eclipse {
                let expr = TrivializePattern::Assign(id.clone(), Box::new(e), info);
                patterns.push(expr);
            }
            patterns
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error_reporting::{CompilePrompt, init_reporter, push_prompt, report_error};

    #[test]
    fn test() {
        init_reporter();
        let source = "
{
    ()
        ^[b, 2.5, ...a] ?= c ? 1 : 2
}
"
        .trim();
        let expr = sap_parser::parse_expr(source).unwrap();
        let filename = None;
        let prompt_label = &mut vec![];
        let ops_to_apply = super::Flatten::from(expr, filename, prompt_label);
        println!("{:#?}\n\n", ops_to_apply);
        let tri = super::TrivializePattern::from_flatten(ops_to_apply, prompt_label);
        println!("{:#?}\n\n", tri);
        report_error(|x| {
            for x in x.iter() {
                if x.labels.is_empty() {
                    continue;
                }
                println!("{:#?}", x);
            }
        });
    }
}
