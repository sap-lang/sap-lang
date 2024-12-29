
pub mod literal;
pub mod module;
pub mod pattern;

use literal::compile_literal;
use pattern::{PatternAssignMode, pattern_assign};

use crate::{
    ast::{SapAST, SapASTBody},
    parser::{
        pattern::{EclipsePattern, ObjectInner},
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

fn compile_inner(ast: SapAST) -> String {
    match ast.body {
        crate::ast::SapASTBody::Id(id) => format!("__ENV__['{}']", id.0),
        crate::ast::SapASTBody::Macro(_id) => todo!(),
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
                            pattern_assign(
                                pattern,
                                arg.clone(),
                                PatternAssignMode::Match,
                                "__ENV__"
                            )
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
        crate::ast::SapASTBody::YieldChild(sap_ast) => {
            format!(
                "(yield* __yield_child__(__ENV__, {}))",
                compile_inner(*sap_ast)
            )
        }

        crate::ast::SapASTBody::Assign(pattern, sap_ast1) => pattern_assign(
            *pattern,
            compile_inner(*sap_ast1),
            PatternAssignMode::Assign,
            "__ENV__",
        ),
        crate::ast::SapASTBody::MatchEquals(pattern, sap_ast1) => {
            format!(
                "((()=>{{try{{{}; return true}}catch(e){{return false}}}})())",
                pattern_assign(
                    *pattern,
                    compile_inner(*sap_ast1),
                    PatternAssignMode::Match,
                    "__ENV__"
                )
            )
        }
        crate::ast::SapASTBody::AssignGetCont(sap_ast, sap_ast1, sap_ast2) => {
            if let SapASTBody::Id(id) = sap_ast1.body {
                pattern_assign(
                    *sap_ast,
                    compile_inner(*sap_ast2),
                    PatternAssignMode::AssignGetCont(id.0),
                    "__ENV__",
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
            if let SapASTBody::Macro(id) = sap_ast.body {
                // @import and @export

                // in the future, we will have a macro system
                // but not in stage 0

                match id.0.as_str() {
                    "@import" => {
                        fn access_chain_to_list(
                            mut state: Vec<String>,
                            ast: SapASTBody,
                        ) -> Vec<String> {
                            match ast {
                                SapASTBody::Access(ast, id) => {
                                    state.push(id.0);
                                    access_chain_to_list(state, ast.body)
                                }
                                SapASTBody::Id(id) => {
                                    state.push(id.0);
                                    state
                                }
                                _ => panic!("Expected id, got {:?}", ast),
                            }
                        }

                        // the first argument must be Id
                        let id = access_chain_to_list(vec![], vec[0].body.clone())
                            .into_iter()
                            .rev()
                            .collect::<Vec<String>>();
                        let cid = id.join("_");
                        let id = id.join(".");
                        if vec.len() == 1 {
                            format!(
                                "
import {cid} from './{id}.js';
for (let key in {cid}) {{
    __ENV__[key] = {cid}[key];
}};
",
                            )
                        } else if vec.len() == 3 {
                            let as_id = vec[2].clone().get_id().0;
                            format!(
                                "
import {cid} from './{id}.js';
__ENV__['{as_id}'] = {cid};
"
                            )
                        } else {
                            panic!("Expected 1 or 3 arguments, got {}", vec.len())
                        }
                    }
                    "@export" => {
                        let exports_object = vec[0].clone();
                        let exports_object = compile_inner(exports_object);
                        format!("export default {exports_object};")
                    }
                    _ => {
                        panic!("Unknown macro {:?}", id.0)
                    }
                }
            } else {
                let f = compile_inner(*sap_ast);
                let args = vec
                    .into_iter()
                    .map(compile_inner)
                    .collect::<Vec<String>>()
                    .join(",");
                format!("__call__(__ENV__, {f}, {args})")
            }
        }
        crate::ast::SapASTBody::Error(sap_parser_error) => {
            panic!("Error: {:?}", sap_parser_error)
        }
    }
}
