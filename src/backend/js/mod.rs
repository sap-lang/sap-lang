// FIXME: `=` 换成 get set binding
use crate::{
    ast::{SapAST, SapASTBody},
    parser::{
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
        let ids = find_all_ids_in_pattern(&p);

        if let Pattern::Literal(literal) = p {
            let literal = compile_literal(literal.clone());
            format!(
                "{literal} === {value} ? {literal} : ((()=>{{ throw new Error('Pattern {literal} not matched') }})())"
            )
        } else {
            let pattern = compile_inner(pattern.clone());
            format!("let {pattern} = {value};")
                + &ids
                    .into_iter()
                    .map(|id| format!("__new_binding__(__ENV__, '{id}', {id})"))
                    .collect::<Vec<String>>()
                    .join(";")
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

fn compile_literal(literal: crate::parser::literal::Literal) -> String {
    match literal {
        crate::parser::literal::Literal::Null => "null".into(),
        crate::parser::literal::Literal::Undefined => "undefined".into(),
        crate::parser::literal::Literal::Void => "undefined".into(),
        crate::parser::literal::Literal::Slot => "{slot: []}".into(),
        crate::parser::literal::Literal::Boolean(b) => b.to_string(),
        crate::parser::literal::Literal::Number(number) => match number {
            crate::parser::literal::number::Number::Int(i) => i.to_string(),
            crate::parser::literal::number::Number::Float(f) => f.to_string(),
            crate::parser::literal::number::Number::BigInt(_big_int) => unimplemented!(),
        },
        crate::parser::literal::Literal::String(string_literal) => match string_literal {
            crate::parser::literal::string::StringLiteral::SingleLine(s) => {
                format!("`{}`", s.replace("\\", "\\\\").replace("`", "\\`"))
            }
            crate::parser::literal::string::StringLiteral::MultiLine(s) => {
                format!("`{}`", s.replace("\\", "\\\\").replace("`", "\\`"))
            }
            crate::parser::literal::string::StringLiteral::Raw(s) => {
                format!("`{}`", s.replace("\\", "\\\\").replace("`", "\\`"))
            }
        },
        crate::parser::literal::Literal::Array(vec) => format!(
            "[{}]",
            vec.into_iter()
                .map(compile_inner)
                .collect::<Vec<String>>()
                .join(",")
        ),
        crate::parser::literal::Literal::Object(vec) => format!(
            "{{{}}}",
            vec.into_iter()
                .map(|(k, v)| format!("{}: {}", compile_inner(k), compile_inner(v)))
                .collect::<Vec<String>>()
                .join(",")
        ),
    }
}

fn find_all_ids_in_pattern(pattern: &Pattern) -> Vec<String> {
    match pattern {
        Pattern::Id(id) => vec![id.0.clone()],
        Pattern::Array(vec) => vec
            .into_iter()
            .map(|x| {
                if let SapASTBody::Pattern(p) = &x.body {
                    p
                } else {
                    unreachable!()
                }
            })
            .map(find_all_ids_in_pattern)
            .flatten()
            .collect(),
        Pattern::Object(vec) => vec
            .into_iter()
            .map(|object_inner| match object_inner {
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
            .flatten()
            .collect(),
        Pattern::Literal(_) => vec![],
        Pattern::Eclipse(EclipsePattern(id)) => vec![id.0.clone()],
    }
}

pub fn compile(ast: SapAST) -> String {
    APPEND_FILE.to_string() + &compile_inner(ast)
}

// __call__ will set the CONT variable in the environment
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

            let pattern = patterns
                .into_iter()
                .zip(args.iter())
                .map(|(pattern, arg)| {
                    if let SapASTBody::Pattern(Pattern::Literal(l)) = pattern.body {
                        let l = compile_literal(l);
                        format!("({l} === {arg})")
                    } else {
                        let pattern_assign = pattern_assign(pattern, arg.clone());
                        format!("(( ()=>{{ {pattern_assign}; return true; }} )())")
                    }
                })
                .collect::<Vec<String>>()
                .join("&&");

            let args = args.join(",");

            let implicit_params = if let Some(implicit_params) = implicit_params {
                implicit_params
                    .into_iter()
                    .map(compile_inner)
                    .map(|x| format!("__new_binding__(__ENV__, '{x}', __PENV__['{x}'])"))
                    .collect::<Vec<String>>()
                    .join(";")
            } else {
                String::new()
            };

            let guard = guard.map(|x| compile_inner(*x)).unwrap_or("true".into());

            let body = if let SapASTBody::Block(body) = body.body {
                compile_block(body)
            } else {
                let body = compile_inner(*body);
                format!("return {body};")
            };

            format!(
                "
                (
                    function*(__PENV__, {args}) {{
                        const __ENV__ = {{' CONT ':undefined}}; __ENV__.__proto__ = __PENV__;

                        if ({pattern}) {{
                            if ({guard}) {{
                                {implicit_params};
                                {body};
                            }} else {{
                                throw new Error('guard failed');
                            }}
                        }} else {{
                            throw new Error('pattern matching failed');
                        }}
                    }}
                )
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
                                ObjectInner::KV(k, v) => {
                                    format!("{}: {}", compile_inner(k), compile_inner(v))
                                }
                                ObjectInner::Eclipse(EclipsePattern(id)) => {
                                    format!("...{}", id.0)
                                }
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
                "((()=>{{const _env = this.__ENV__; const __ENV__ = {{' CONT ':undefined}}; __ENV__.__proto__ = _env; {block}}})())"
            )
        }
        crate::ast::SapASTBody::Literal(literal) => compile_literal(literal),
        crate::ast::SapASTBody::Typeof(sap_ast) => format!("(typeof {})", compile_inner(*sap_ast)),
        crate::ast::SapASTBody::Yield(sap_ast) => format!("(yield {})", compile_inner(*sap_ast)),

        crate::ast::SapASTBody::Assign(pattern, sap_ast1) => {
            let value = compile_inner(*sap_ast1);
            pattern_assign(*pattern, value)
        }
        crate::ast::SapASTBody::AssignGetCont(sap_ast, sap_ast1, sap_ast2) => {
            let b = compile_inner(*sap_ast1);

            let value = compile_inner(*sap_ast2);
            let ac = pattern_assign(*sap_ast, value);
            format!(
                "
                ((()=>{{
                {ac};
                __new_binding__(__ENV__, ' CONT ', {b});
                }})())
                "
            )
        }
        crate::ast::SapASTBody::AssignSlot(sap_ast, sap_ast1) => {
            let a = if let crate::ast::SapASTBody::Id(id) = &sap_ast.body {
                id.0.clone()
            } else {
                compile_inner(*sap_ast)
            };
            let b = compile_inner(*sap_ast1);
            format!("__new_slot_binding__(__ENV__, '{a}', {b})",)
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
            format!(
                "{}['{}']",
                compile_inner(*sap_ast),
                compile_inner(*sap_ast1)
            )
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