use std::collections::HashMap;

// FIXME: `=` 换成 get set binding
use crate::{
    ast::{SapAST, SapASTBody},
    backend::literal::compile_literal,
    parser::{
        literal::Literal,
        pattern::{EclipsePattern, ObjectInner, Pattern},
    },
};

use super::compile_inner;

pub enum PatternAssignMode {
    Assign,
    Match,
    AssignGetCont(String),
}

pub fn pattern_assign(
    pattern: SapAST,
    value: String,
    mode: PatternAssignMode,
    env: &str,
) -> String {
    if let pattern @ SapAST {
        span: _,
        body: SapASTBody::Pattern(p),
    } = &pattern
    {
        let mut i = 0;
        let (p, map) = replace_all_literal_in_pattern(p, &mut i);

        let ids = find_all_ids_in_pattern(&p);

        let pattern = compile_inner(SapAST {
            span: pattern.span.clone(),
            body: SapASTBody::Pattern(p),
        });

        "{".to_string()
                + &if matches!(mode , PatternAssignMode::Assign) || matches!(mode , PatternAssignMode::Match) {
                    format!("let {pattern} = __extract_return__({value});")
                } else {
                    format!("let {pattern} = {value};")
                }
                + &ids
                    .iter()
                    .map(|id| match mode {
                        PatternAssignMode::Assign | PatternAssignMode::Match => {
                            format!("__new_binding__({env}, '{id}', {id})")
                        }
                        PatternAssignMode::AssignGetCont(ref cid) => {
                            format!("__new_binding_cont__({env}, '{id}', '{cid}', {id})")
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(";")
                + ";" 
                + &match mode {
                    PatternAssignMode::Assign => String::new(),
                    PatternAssignMode::Match => ids
                        .iter()
                        .map(|id| {
                            format!(
                                "if({id} !== undefined){{}} else {{throw new Error('pattern matching failed')}}",
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(";"),
                    PatternAssignMode::AssignGetCont(_) => String::new(),
                }
                + ";" +

                    &map.into_iter().map(|(k, v) | {
                        let literal = compile_literal(v);
                        format!(
                            "__equals__(_l_{k},{literal}) ? {literal} : ((()=>{{ throw new Error('pattern matching failed') }})())"
                        )
                    }).collect::<Vec<String>>().join(";") + "}"
    } else if let SapAST {
        span,
        body: SapASTBody::Id(i),
    } = pattern
    {
        let pattern_ast = SapAST {
            span,
            body: SapASTBody::Pattern(Pattern::Id(i.clone())),
        };
        pattern_assign(pattern_ast, value, mode, env)
    } else {
        unreachable!("Expected pattern, got {:?}", pattern)
    }
}

fn replace_all_literal_in_pattern(
    pattern: &Pattern,
    i: &mut i32,
) -> (Pattern, HashMap<i32, Literal>) {
    let mut map = HashMap::new();
    match pattern {
        Pattern::Id(_) => (pattern.clone(), map),
        Pattern::Literal(l) => {
            let ii = *i;
            map.insert(ii, l.clone());
            *i += 1;
            (
                Pattern::Id(crate::parser::primary::id::Id(format!("_l_{ii}"))),
                map,
            )
        }
        Pattern::Array(vec) => {
            let mut new_vec = vec![];
            for x in vec {
                if let SapASTBody::Pattern(p) = &x.body {
                    let (new_x, new_map) = replace_all_literal_in_pattern(p, i);
                    new_vec.push(SapAST {
                        span: x.span.clone(),
                        body: SapASTBody::Pattern(new_x),
                    });
                    map.extend(new_map);
                } else {
                    unreachable!()
                }
            }
            (Pattern::Array(new_vec), map)
        }
        Pattern::Eclipse(_) => (pattern.clone(), map),
        Pattern::Object(vec) => {
            let mut new_vec = vec![];
            for object_inner in vec {
                match object_inner {
                    ObjectInner::KV(k, v) => {
                        let vs = v.span.clone();
                        let v = if let SapASTBody::Pattern(p) = &v.body {
                            p
                        } else {
                            unreachable!()
                        };
                        let (new_v, new_map) = replace_all_literal_in_pattern(v, i);
                        new_vec.push(ObjectInner::KV(k.clone(), SapAST {
                            span: vs,
                            body: SapASTBody::Pattern(new_v),
                        }));
                        map.extend(new_map);
                    }
                    ObjectInner::Eclipse(_) => {
                        new_vec.push(object_inner.clone());
                    }
                }
            }
            (Pattern::Object(new_vec), map)
        }
    }
}

fn find_all_ids_in_pattern(pattern: &Pattern) -> Vec<String> {
    match pattern {
        Pattern::Id(id) => vec![id.0.clone()],
        Pattern::Array(vec) => vec
            .iter()
            .map(|x| {
                if let SapASTBody::Pattern(p) = &x.body {
                    p
                } else {
                    unreachable!()
                }
            })
            .flat_map(find_all_ids_in_pattern)
            .collect(),
        Pattern::Object(vec) => vec
            .iter()
            .flat_map(|object_inner| match object_inner {
                ObjectInner::KV(_k, v) => {
                    let v = if let SapASTBody::Pattern(p) = &v.body {
                        p
                    } else {
                        unreachable!()
                    };
                    find_all_ids_in_pattern(v)
                }
                ObjectInner::Eclipse(EclipsePattern(id)) => vec![id.0.clone()],
            })
            .collect(),
        Pattern::Literal(_) => vec![],
        Pattern::Eclipse(EclipsePattern(id)) => vec![id.0.clone()],
    }
}
