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

fn pattern_assign(pattern: SapAST, value: String) -> String {
    if let pattern @ SapAST {
        span: _,
        body: SapASTBody::Pattern(p),
    } = &pattern
    {
        // sap_ast is a pattern
        let ids = find_all_ids_in_pattern(p);

        if let Pattern::Literal(literal) = p {
            let literal = compile_literal(literal.clone());
            format!(
                "__equals__({literal},{value}) ? {literal} : ((()=>{{ throw new Error('Pattern {literal} not matched') }})())"
            )
        } else {
            let pattern = compile_inner(pattern.clone());
            "{".to_string()
                + &format!("let {pattern} = {value};")
                + &ids
                    .into_iter()
                    .map(|id| format!("__new_binding__(__ENV__, '{id}', {id})"))
                    .collect::<Vec<String>>()
                    .join(";")
                + "}"
        }
    } else if let SapAST {
        span,
        body: SapASTBody::Id(i),
    } = pattern
    {
        let pattern_ast = SapAST {
            span,
            body: SapASTBody::Pattern(Pattern::Id(i.clone())),
        };
        pattern_assign(pattern_ast, value)
    } else {
        unreachable!("Expected pattern, got {:?}", pattern)
    }
}

fn pattern_match_assign(pattern: SapAST, value: String) -> String {
    if let pattern @ SapAST {
        span: _,
        body: SapASTBody::Pattern(p),
    } = &pattern
    {
        // sap_ast is a pattern
        let ids = find_all_ids_in_pattern(p);

        let exprs = if let Pattern::Literal(literal) = p {
            let literal = compile_literal(literal.clone());
            format!(
                "__equals__({literal}, {value}) ? {literal} : ((()=>{{ throw new Error('Pattern {literal} not matched') }})())"
            )
        } else {
            let pattern = compile_inner(pattern.clone());
            "{".to_string()
                + &format!("let {pattern} = {value};")
                + &ids
                    .iter()
                    .map(|id| format!("__new_binding__(__ENV__, '{id}', {id})"))
                    .collect::<Vec<String>>()
                    .join(";")
                + ";"
                + &ids
                    .iter()
                    .map(|id| {
                        format!("if({id}){{}} else {{throw new Error('{id} is not destructed')}}")
                    })
                    .collect::<Vec<String>>()
                    .join(";")
                + "}"
        };

        format!("((()=>{{ try {{{exprs}; return true}} catch (e) {{return false}} }})())")
    } else if let SapAST {
        span,
        body: SapASTBody::Id(i),
    } = pattern
    {
        let pattern_ast = SapAST {
            span,
            body: SapASTBody::Pattern(Pattern::Id(i.clone())),
        };
        pattern_match_assign(pattern_ast, value)
    } else {
        unreachable!("Expected pattern, got {:?}", pattern)
    }
}

fn pattern_assign_get_cont(pattern: SapAST, cid: String, value: String) -> String {
    if let pattern @ SapAST {
        span: _,
        body: SapASTBody::Pattern(p),
    } = &pattern
    {
        // sap_ast is a pattern
        let ids = find_all_ids_in_pattern(p);
        if let Pattern::Literal(literal) = p {
            let literal = compile_literal(literal.clone());
            format!(
                "__equals__({literal}, {value}) ? {literal} : ((()=>{{ throw new Error('Pattern {literal} not matched') }})())"
            )
        } else {
            let pattern = compile_inner(pattern.clone());
            "{".to_string()
                + &format!("let {pattern} = {value};")
                + &ids
                    .into_iter()
                    .map(|id| format!("__new_binding_cont__(__ENV__, '{id}', '{cid}', {id})"))
                    .collect::<Vec<String>>()
                    .join(";")
                + "}"
        }
    } else if let SapAST {
        span,
        body: SapASTBody::Id(i),
    } = pattern
    {
        let pattern_ast = SapAST {
            span,
            body: SapASTBody::Pattern(Pattern::Id(i.clone())),
        };
        pattern_assign_get_cont(pattern_ast, cid, value)
    } else {
        unreachable!("Expected pattern, got {:?}", pattern)
    }
}

fn compile_literal(literal: Literal) -> String {
    match literal {
        Literal::Null => "null".into(),
        Literal::Undefined => "undefined".into(),
        Literal::Void => "undefined".into(),
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
    let append_file = APPEND_FILE;
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
                        if let SapASTBody::Pattern(Pattern::Literal(l)) = pattern.body {
                            let l = compile_literal(l);
                            format!("(__equals__({l},{arg}))")
                        } else {
                            let pattern_assign = pattern_assign(pattern, arg.clone());
                            format!("(( ()=>{{ {pattern_assign}; return true; }} )())")
                        }
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
        if (__extract_return__({pattern})) {{
        {body}
        }} else {{throw new Error('pattern matching failed');}}
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
                "(__call__(__ENV__, (function*(){{ {} }}), undefined))",
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

        crate::ast::SapASTBody::Assign(pattern, sap_ast1) => {
            pattern_assign(*pattern, compile_inner(*sap_ast1))
        }
        crate::ast::SapASTBody::MatchEquals(pattern, sap_ast1) => {
            pattern_match_assign(*pattern, compile_inner(*sap_ast1))
        }
        crate::ast::SapASTBody::AssignGetCont(sap_ast, sap_ast1, sap_ast2) => {
            if let SapASTBody::Id(id) = sap_ast1.body {
                pattern_assign_get_cont(*sap_ast, id.0, compile_inner(*sap_ast2))
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

const APPEND_FILE: &str = include_str!("js_prelude.js");
