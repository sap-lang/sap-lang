use std::collections::HashMap;

// FIXME: `=` 换成 get set binding
use crate::{
    ast::{SapAST, SapASTBody},
    parser::{
        literal::{Literal, number::Number, string::StringLiteral},
        pattern::{EclipsePattern, ObjectInner, Pattern},
        primary::lambda_expr::LambdaExpr,
    },
};

fn compile_block(mut vec: Vec<SapAST>) -> String {
    if let Some(ret) = vec.pop() {
        vec.into_iter()
            .map(compile_inner)
            .collect::<Vec<String>>()
            .join(";")
            + &format!(";return {};", compile_inner(ret))
    } else {
        "return;".into()
    }
}

enum PatternAssignMode {
    Assign,
    Match,
    AssignGetCont(String),
}

fn pattern_assign(pattern: SapAST, value: String, mode: PatternAssignMode) -> String {
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
                + &format!("let {pattern} = __extract_return__({value});")
                + &ids
                    .iter()
                    .map(|id| match mode {
                        PatternAssignMode::Assign | PatternAssignMode::Match => {
                            format!("__new_binding__(__ENV__, '{id}', {id})")
                        }
                        PatternAssignMode::AssignGetCont(ref cid) => {
                            format!("__new_binding_cont__(__ENV__, '{id}', '{cid}', {id})")
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
                                "if({id} !== undefined){{}} else {{throw new Error('{id} is not destructed')}}"
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
                            "__equals__(_l_{k},{literal}) ? {literal} : ((()=>{{ throw new Error('Pattern {literal} not matched') }})())"
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
        pattern_assign(pattern_ast, value, mode)
    } else {
        unreachable!("Expected pattern, got {:?}", pattern)
    }
}

fn compile_literal(literal: Literal) -> String {
    match literal {
        Literal::Null => "null".into(),
        Literal::Undefined => "undefined".into(),
        Literal::Void => "{__void__:true}".into(),
        Literal::Slot => "{slot: []}".into(),
        Literal::Boolean(b) => b.to_string(),
        Literal::Number(number) => match number {
            Number::Int(i) => i.to_string(),
            Number::Float(f) => f.to_string(),
            // Number::BigInt(_big_int) => unimplemented!(),
        },
        Literal::String(string_literal) => match string_literal {
            StringLiteral::SingleLine(s) => {
                format!("`{}`", s.replace("\\", "\\\\").replace("`", "\\`"))
            }
            StringLiteral::MultiLine(s) => {
                format!("`{}`", s.replace("\\", "\\\\").replace("`", "\\`"))
            }
            StringLiteral::Raw(s) => {
                format!("`{}`", s.replace("\\", "\\\\").replace("`", "\\`"))
            }
        },
        Literal::Array(vec) => format!(
            "[{}]",
            vec.into_iter()
                .map(compile_inner)
                .map(|x| format!("__extract_return__({x})"))
                .collect::<Vec<String>>()
                .join(",")
        ),
        Literal::Object(vec) => format!(
            "{{{}}}",
            vec.into_iter()
                .map(|(k, v)| format!("{}: __extract_return__({})", k, compile_inner(v)))
                .collect::<Vec<String>>()
                .join(",")
        ),
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

pub fn compile(ast: Vec<SapAST>) -> String {
    let append_file = append_file("src/backend/prelude");
    let body = ast
        .into_iter()
        .map(compile_inner)
        .collect::<Vec<String>>()
        .join(";\n");
    format!(
        "
{append_file}

async function __main__() {{
    let main = (function*(){{
        {body}
    }});

    let main_process = main();
    try {{
        let cont = undefined;
        let ret = undefined;
        while (1) {{
            ret = main_process.next(cont)
            if (ret.done) {{
                break;
            }}
            cont = await ret.value;
        }}
    }} catch (e) {{
        console.error(e);
        return;
    }}
}}

await __main__();
__rl.close();

"
    )
}

fn compile_inner(ast: SapAST) -> String {
    match ast.body {
        crate::ast::SapASTBody::Id(id) => format!("__ENV__['{}']", id.0),
        crate::ast::SapASTBody::LambdaExpr(LambdaExpr {
            patterns,
            implicit_params,
            guard,
            body,
        }) => {
            let args = (0..patterns.len())
                .map(|i| format!("_{}", i))
                .collect::<Vec<String>>();

            let pattern = if !patterns.is_empty() {
                patterns
                    .into_iter()
                    .zip(args.iter())
                    .map(|(pattern, arg)| {
                        format!(
                            "((()=>{{{}; return true}})())",
                            pattern_assign(pattern, arg.clone(), PatternAssignMode::Match)
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("&&")
            } else {
                "true".into()
            };

            let args = args.join(",");

            let implicit_params = if let Some(implicit_params) = implicit_params {
                implicit_params
                    .into_iter()
                    .map(|x| {
                        if let SapASTBody::Id(id) = x.body {
                            id.0
                        } else {
                            unreachable!()
                        }
                    })
                    .map(|x| format!("__new_binding__(__ENV__, '{x}', __PENV__['{x}'])"))
                    .collect::<Vec<String>>()
                    .join(";")
            } else {
                String::new()
            };

            let body = if let SapASTBody::Block(body) = body.body {
                compile_block(body)
            } else {
                let body = compile_inner(*body);
                format!("return {body};")
            };

            let body = if let Some(guard) = guard {
                format!(
                    "if (__extract_return__({guard})) {{ {implicit_params}; {body} }} else {{ throw new Error('guard failed'); }}",
                    guard = compile_inner(*guard),
                )
            } else {
                format!("{implicit_params}; {body}")
            };

            format!(
                "
(function*(__PENV__, {args}) {{const __ENV__ = {{ }}; __ENV__.__proto__ = __PENV__;
        {pattern}
        {body}
}})
"
            )
        }
        crate::ast::SapASTBody::Pattern(pattern) => match pattern {
            crate::parser::pattern::Pattern::Id(id) => id.0,
            crate::parser::pattern::Pattern::Array(vec) => {
                format!(
                    "[{}]",
                    vec.into_iter()
                        .map(compile_inner)
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            crate::parser::pattern::Pattern::Object(vec) => {
                format!(
                    "{{{}}}",
                    vec.into_iter()
                        .map(|oi| {
                            match oi {
                                ObjectInner::KV(
                                    SapAST {
                                        body: SapASTBody::Id(k),
                                        ..
                                    },
                                    v,
                                ) => {
                                    format!("{}: {}", k.0, compile_inner(v))
                                }
                                ObjectInner::Eclipse(EclipsePattern(id)) => {
                                    format!("...{}", id.0)
                                }
                                _ => unimplemented!("Invalid object inner {:?}", oi),
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            crate::parser::pattern::Pattern::Literal(literal) => compile_literal(literal),
            crate::parser::pattern::Pattern::Eclipse(EclipsePattern(id)) => {
                format!("...{}", id.0)
            }
        },
        crate::ast::SapASTBody::Block(vec) => {
            let block = compile_block(vec);
            format!(
                "(__call__(__ENV__, (function*(){{ {} }}), {{__void__:true}}))",
                block
            )
        }
        crate::ast::SapASTBody::Literal(literal) => compile_literal(literal),
        crate::ast::SapASTBody::Typeof(sap_ast) => format!("(typeof {})", compile_inner(*sap_ast)),
        crate::ast::SapASTBody::Yield(sap_ast) => format!("(yield {})", compile_inner(*sap_ast)),
        crate::ast::SapASTBody::YieldChild(sap_ast) => format!(
            "(yield* (function*(){{ const r = {0}; if (__is_return__(r)){{return (yield r.value)}} else {{return (yield* r)}} }})())",
            compile_inner(*sap_ast)
        ),

        crate::ast::SapASTBody::Assign(pattern, sap_ast1) => pattern_assign(
            *pattern,
            compile_inner(*sap_ast1),
            PatternAssignMode::Assign,
        ),
        crate::ast::SapASTBody::MatchEquals(pattern, sap_ast1) => {
            format!(
                "((()=>{{try{{{}; return true}}catch(e){{return false}}}})())",
                pattern_assign(*pattern, compile_inner(*sap_ast1), PatternAssignMode::Match)
            )
        }
        crate::ast::SapASTBody::AssignGetCont(sap_ast, sap_ast1, sap_ast2) => {
            if let SapASTBody::Id(id) = sap_ast1.body {
                pattern_assign(
                    *sap_ast,
                    compile_inner(*sap_ast2),
                    PatternAssignMode::AssignGetCont(id.0),
                )
            } else {
                unimplemented!("Expected id, got {:?}", sap_ast1)
            }
        }
        crate::ast::SapASTBody::AssignSlot(sap_ast, sap_ast1) => {
            let a = if let crate::ast::SapASTBody::Id(id) = &sap_ast.body {
                id.0.clone()
            } else {
                unimplemented!("Expected id, got {:?}", sap_ast)
            };
            let b = compile_inner(*sap_ast1);
            format!("__new_slot_binding__(__ENV__, '{a}', {b})",)
        }

        crate::ast::SapASTBody::If(cond, then, else_) => {
            let cond = compile_inner(*cond);
            let then = compile_inner(*then);
            let else_ = compile_inner(*else_);
            format!(
                "((()=>{{if (__extract_return__({cond})) {{return {then}}} else {{return {else_}}}}})())"
            )
        }

        crate::ast::SapASTBody::Not(sap_ast) => {
            format!(
                "__call__(__ENV__, __ENV__['(!)'], {})",
                compile_inner(*sap_ast)
            )
        }
        crate::ast::SapASTBody::Neg(sap_ast) => {
            format!(
                "__call__(__ENV__, __ENV__['(neg)'], {})",
                compile_inner(*sap_ast)
            )
        }
        crate::ast::SapASTBody::BitNot(sap_ast) => {
            format!(
                "__call__(__ENV__, __ENV__['(~)'], {})",
                compile_inner(*sap_ast)
            )
        }
        crate::ast::SapASTBody::Add(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(+)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::Sub(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(-)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::Mul(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(*)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::Div(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(/)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::Mod(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(%)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::Eq(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(==)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::Neq(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(!=)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::Le(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(<=)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::Ge(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(>=)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::Lt(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(<)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::Gt(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(>)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::And(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(&&)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::Or(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(||)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::BitOr(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(|)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::BitAnd(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(&)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::BitXor(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(^)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::BitShiftL(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(<<)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::BitShiftR(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['(>>)'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::Extends(sap_ast, sap_ast1) => {
            let a = compile_inner(*sap_ast);
            let b = compile_inner(*sap_ast1);
            format!("((()=>{{let a = {a};let b = {b}; a.__proto__ = b; return a;}})())")
        }
        crate::ast::SapASTBody::Slice(sap_ast, sap_ast1, sap_ast2, sap_ast3) => format!(
            "__call__(__ENV__, __ENV__['(slice)'], {}, {}, {}, {})",
            compile_inner(*sap_ast),
            sap_ast1
                .map(|x| *x)
                .map(compile_inner)
                .unwrap_or(0.to_string()),
            sap_ast2
                .map(|x| *x)
                .map(compile_inner)
                .unwrap_or("-1".to_string()),
            sap_ast3
                .map(|x| *x)
                .map(compile_inner)
                .unwrap_or("1".to_string())
        ),
        crate::ast::SapASTBody::Access(sap_ast, sap_ast1) => {
            format!("{}['{}']", compile_inner(*sap_ast), sap_ast1.0)
        }
        crate::ast::SapASTBody::Index(sap_ast, sap_ast1) => format!(
            "__call__(__ENV__, __ENV__['([])'], {}, {})",
            compile_inner(*sap_ast),
            compile_inner(*sap_ast1)
        ),
        crate::ast::SapASTBody::App(sap_ast, vec) => {
            let f = compile_inner(*sap_ast);
            let args = vec
                .into_iter()
                .map(compile_inner)
                .collect::<Vec<String>>()
                .join(",");
            format!("__call__(__ENV__, {f}, {args})")
        }
        crate::ast::SapASTBody::Error(sap_parser_error) => {
            panic!("Error: {:?}", sap_parser_error)
        }
    }
}

pub fn append_file(dir: &str) -> String {
    const APPEND_FILE: &str = include_str!("prelude.js");

    let dir = std::fs::read_dir(dir).unwrap();
    let mut files = vec![];
    for file in dir {
        let file = file.unwrap();
        let file_content = std::fs::read_to_string(file.path()).unwrap();
        files.push(file_content);
    }
    let std = files.join("\n");

    format!("{}\n{}", APPEND_FILE, std)
}
