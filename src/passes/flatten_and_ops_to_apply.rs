use sap_parser::{
    expr::{Expr, ExprInner, Inner as PrimaryInner, Primary},
    function::ImplicitParams,
    id::{Id, MagicFnId, NormalId},
    literal::{Inner as LiteralInner, Literal, object::ObjectKey},
    pattern::object::ObjectPatternKv,
};

use crate::{def_pass_with_metainfo, simple_literal::SimpleLiteral, simple_pattern::SimplePattern};

use super::CommonMetaInfo;

use crate::error_reporting::{CompilePrompt, CompilePromptLabel, push_prompt};

#[derive(Debug, Clone)]
pub struct FlattenLambdaExpr {
    pub patterns: Vec<SimplePattern>,
    pub implicit_params: Option<Vec<String>>,
    pub guard: Option<Box<Flatten>>,
    pub body: Box<Flatten>,
}

def_pass_with_metainfo! {
    Flatten {
        Block(exprs: Vec<Flatten>),
        LambdaExpr(lambda_expr: FlattenLambdaExpr),
        IfThenElse(cond: Box<Flatten>, true_expr: Box<Flatten>, false_expr: Box<Flatten>),

        Array(elems: Vec<Flatten>),
        Object(body: Vec<(String, Flatten)>),
        Pattern(pattern: SimplePattern),
        Literal(literal: SimpleLiteral),
        Id(id: String),
        MacroId(id: String),
        Yield(yielding: Box<Flatten>),
        Access(id: String, expr: Box<Flatten>),
        Extend(base: Box<Flatten>, parent: Box<Flatten>),
        FindAndCallWithThis(calling: Box<Flatten>, id: Box<Flatten>),
        Apply(call: Box<Flatten>, args: Vec<Flatten>),

        MatchEquals(matching: Box<Flatten>, eq: Box<Flatten>),
        Assign(left: Box<Flatten>, right: Box<Flatten>),
        Set(left: Box<Flatten>, right: Box<Flatten>),
        AssignSlot(leftslot: Box<Flatten>, right: Box<Flatten>),
    }
}

impl Flatten {
    pub fn from(
        value: Expr,
        filename: Option<&'static str>,
        prompt_label: &mut Vec<CompilePromptLabel>,
    ) -> Self {
        let Expr { inner: value, diag } = value;
        let info = CommonMetaInfo::new(diag, filename);
        match value {
            ExprInner::Prefix(prefix, expr) => match prefix {
                sap_parser::expr::prefix::Prefix::Not => Flatten::Apply(
                    Box::new(Flatten::Id("(!)".to_string(), info)),
                    vec![Flatten::from(*expr, filename, prompt_label)],
                    info,
                ),
                sap_parser::expr::prefix::Prefix::BitNot => Flatten::Apply(
                    Box::new(Flatten::Id("(~)".to_string(), info)),
                    vec![Flatten::from(*expr, filename, prompt_label)],
                    info,
                ),
                sap_parser::expr::prefix::Prefix::Neg => Flatten::Apply(
                    Box::new(Flatten::Id("(neg)".to_string(), info)),
                    vec![Flatten::from(*expr, filename, prompt_label)],
                    info,
                ),
                sap_parser::expr::prefix::Prefix::Yield => {
                    Flatten::Yield(Box::new(Flatten::from(*expr, filename, prompt_label)), info)
                }
                sap_parser::expr::prefix::Prefix::AnnotativeMacroCall(macro_id, e) => {
                    Flatten::Apply(
                        Box::new(Flatten::MacroId(macro_id.value, info)),
                        if let Some(e) = e {
                            vec![
                                Flatten::from(*e, filename, prompt_label),
                                Flatten::from(*expr, filename, prompt_label),
                            ]
                        } else {
                            vec![Flatten::from(*expr, filename, prompt_label)]
                        },
                        info,
                    )
                }
            },
            ExprInner::Primary(primary) => match primary.inner {
                PrimaryInner::Block(block) => {
                    if block.exprs.is_empty() {
                        prompt_label.push(CompilePromptLabel::new(
                            crate::error_reporting::CompilePromptCode::EmptyBlockAsVoid,
                            info,
                        ));
                        return Flatten::Literal(SimpleLiteral::Void, info);
                    } else if block.exprs.len() == 1 {
                        prompt_label.push(CompilePromptLabel::new(
                            crate::error_reporting::CompilePromptCode::BlockShouldBeRemovedOrUsingParentheses,
                            info,
                        ));
                        return Flatten::from(
                            block.exprs.into_iter().next().unwrap(),
                            filename,
                            prompt_label,
                        );
                    }
                    let mut prompt_labels_s = vec![];
                    let prompt_labels = &mut prompt_labels_s;
                    let r = Flatten::Block(
                        block
                            .exprs
                            .into_iter()
                            .map(|x| Flatten::from(x, filename, prompt_labels))
                            .collect(),
                        info,
                    );
                    push_prompt(CompilePrompt::new(prompt_labels_s, info));
                    r
                }
                PrimaryInner::LambdaExpr(lambda_expr) => {
                    let mut prompt_labels_s = vec![];
                    let prompt_labels = &mut prompt_labels_s;

                    let r = match lambda_expr {
                        sap_parser::function::LambdaExpr::TrLambda(tr_lambda) => {
                            if tr_lambda.patterns.is_empty()
                                && tr_lambda.implicit_params.is_none()
                                && tr_lambda.guard.is_none()
                            {
                                prompt_label.push(CompilePromptLabel::new(
                                    crate::error_reporting::CompilePromptCode::BetterUsingNoParamLambdaExpr,
                                    info,
                                ));
                            }
                            Flatten::LambdaExpr(
                                FlattenLambdaExpr {
                                    patterns: tr_lambda
                                        .patterns
                                        .into_iter()
                                        .map(|x| SimplePattern::from(x, filename, prompt_labels))
                                        .collect(),
                                    implicit_params: tr_lambda.implicit_params.map(|ps| {
                                        ps.params.into_iter().map(|p| p.value()).collect()
                                    }),
                                    guard: tr_lambda.guard.map(|g| {
                                        Box::new(Flatten::from(*g.expr, filename, prompt_labels))
                                    }),
                                    body: Box::new(Flatten::from(
                                        *tr_lambda.body,
                                        filename,
                                        prompt_labels,
                                    )),
                                },
                                info,
                            )
                        }
                        sap_parser::function::LambdaExpr::NoParamLambdaExpr(
                            no_param_lambda_expr,
                        ) => Flatten::LambdaExpr(
                            FlattenLambdaExpr {
                                patterns: vec![],
                                implicit_params: None,
                                guard: None,
                                body: Box::new(Flatten::Block(
                                    no_param_lambda_expr
                                        .body
                                        .into_iter()
                                        .map(|x| Flatten::from(x, filename, prompt_labels))
                                        .collect(),
                                    info,
                                )),
                            },
                            info,
                        ),
                    };

                    push_prompt(CompilePrompt::new(prompt_labels_s, info));
                    r
                }
                PrimaryInner::ParenExpr(paren_expr) => {
                    Flatten::from(*paren_expr.expr, filename, prompt_label)
                }
                PrimaryInner::CompoundLiteral(compound_literal) => match compound_literal {
                    sap_parser::literal::CompoundLiteral::ArrayLiteral(array_body) => {
                        Flatten::Array(
                            array_body
                                .elems
                                .into_iter()
                                .map(|x| Flatten::from(x, filename, prompt_label))
                                .collect(),
                            info,
                        )
                    }
                    sap_parser::literal::CompoundLiteral::ObjectLiteral(object_body) => {
                        Flatten::Object(
                            object_body
                                .body
                                .into_iter()
                                .map(|kv| {
                                    let value = if let Some(v) = kv.value {
                                        Flatten::from(v, filename, prompt_label)
                                    } else {
                                        Flatten::Id(kv.key.value(), info)
                                    };
                                    (kv.key.value(), value)
                                })
                                .collect(),
                            info,
                        )
                    }
                    sap_parser::literal::CompoundLiteral::Literal(literal) => {
                        Flatten::Literal(SimpleLiteral::from(literal), info)
                    }
                },
                PrimaryInner::Id(id) => {
                    if let Id::MacroId(m) = id {
                        Flatten::MacroId(m.value, info)
                    } else {
                        Flatten::Id(id.value(), info)
                    }
                }
                PrimaryInner::Pattern(pattern) => {
                    Flatten::Pattern(SimplePattern::from(pattern, filename, prompt_label), info)
                }
            },
            ExprInner::Postfix(postfix, expr) => match postfix {
                sap_parser::expr::postfix::Postfix::Trinary(trinary) => {
                    let mut prompt_labels_s = vec![];
                    let prompt_labels = &mut prompt_labels_s;
                    let r = Flatten::IfThenElse(
                        Box::new(Flatten::from(*expr, filename, prompt_labels)),
                        Box::new(Flatten::from(*trinary.true_expr, filename, prompt_labels)),
                        Box::new(Flatten::from(*trinary.false_expr, filename, prompt_labels)),
                        info,
                    );
                    push_prompt(CompilePrompt::new(prompt_labels_s, info));
                    r
                }
                sap_parser::expr::postfix::Postfix::Slice(slice) => Flatten::Apply(
                    Box::new(Flatten::Id("([::])".to_string(), info)),
                    vec![
                        slice
                            .start
                            .map(|s| Flatten::from(*s, filename, prompt_label))
                            .unwrap_or(Flatten::Literal(SimpleLiteral::Void, info)),
                        slice
                            .end
                            .map(|s| Flatten::from(*s, filename, prompt_label))
                            .unwrap_or(Flatten::Literal(SimpleLiteral::Void, info)),
                        slice
                            .step
                            .map(|s| Flatten::from(*s, filename, prompt_label))
                            .unwrap_or(Flatten::Literal(SimpleLiteral::Void, info)),
                        Flatten::from(*expr, filename, prompt_label),
                    ],
                    info,
                ),
                sap_parser::expr::postfix::Postfix::Index(index) => Flatten::Apply(
                    Box::new(Flatten::Id("([])".to_string(), info)),
                    vec![
                        Flatten::from(*index.postfix_index, filename, prompt_label),
                        Flatten::from(*expr, filename, prompt_label),
                    ],
                    info,
                ),
                sap_parser::expr::postfix::Postfix::Access(access) => Flatten::Access(
                    access.id.value(),
                    Box::new(Flatten::from(*expr, filename, prompt_label)),
                    info,
                ),

                _ => {
                    prompt_label.push(CompilePromptLabel::new(
                        crate::error_reporting::CompilePromptCode::InternalErrorShouldBeFlatten,
                        info,
                    ));
                    Flatten::Literal(SimpleLiteral::Void, info)
                }
            },
            ExprInner::Infix(infix, expr, expr1) => {
                let infix_id = match infix {
                    sap_parser::expr::infix::Infix::Assign => {
                        return Flatten::Assign(
                            Box::new(Flatten::from(*expr, filename, prompt_label)),
                            Box::new(Flatten::from(*expr1, filename, prompt_label)),
                            info,
                        );
                    }
                    sap_parser::expr::infix::Infix::Set => {
                        return Flatten::Set(
                            Box::new(Flatten::from(*expr, filename, prompt_label)),
                            Box::new(Flatten::from(*expr1, filename, prompt_label)),
                            info,
                        );
                    }
                    sap_parser::expr::infix::Infix::AssignSlot => {
                        return Flatten::AssignSlot(
                            Box::new(Flatten::from(*expr, filename, prompt_label)),
                            Box::new(Flatten::from(*expr1, filename, prompt_label)),
                            info,
                        );
                    }

                    sap_parser::expr::infix::Infix::MatchEquals => {
                        return Flatten::MatchEquals(
                            Box::new(Flatten::from(*expr, filename, prompt_label)),
                            Box::new(Flatten::from(*expr1, filename, prompt_label)),
                            info,
                        );
                    }
                    sap_parser::expr::infix::Infix::FindAndCallWithThis => {
                        return Flatten::FindAndCallWithThis(
                            Box::new(Flatten::from(*expr, filename, prompt_label)),
                            Box::new(Flatten::from(*expr1, filename, prompt_label)),
                            info,
                        );
                    }

                    sap_parser::expr::infix::Infix::Add => "(+)".to_string(),
                    sap_parser::expr::infix::Infix::Sub => "(-)".to_string(),
                    sap_parser::expr::infix::Infix::Mul => "(*)".to_string(),
                    sap_parser::expr::infix::Infix::Div => "(/)".to_string(),
                    sap_parser::expr::infix::Infix::Mod => "(%)".to_string(),
                    sap_parser::expr::infix::Infix::Eq => "(==)".to_string(),
                    sap_parser::expr::infix::Infix::Neq => "(!=)".to_string(),
                    sap_parser::expr::infix::Infix::Le => "(<=)".to_string(),
                    sap_parser::expr::infix::Infix::Ge => "(>=)".to_string(),
                    sap_parser::expr::infix::Infix::Lt => "(<)".to_string(),
                    sap_parser::expr::infix::Infix::Gt => "(>)".to_string(),
                    sap_parser::expr::infix::Infix::And => "(&&)".to_string(),
                    sap_parser::expr::infix::Infix::Pipe => "(|>)".to_string(),
                    sap_parser::expr::infix::Infix::Or => "(||)".to_string(),
                    sap_parser::expr::infix::Infix::BitOr => "(|)".to_string(),
                    sap_parser::expr::infix::Infix::BitAnd => "(&)".to_string(),
                    sap_parser::expr::infix::Infix::BitXor => "(^)".to_string(),
                    sap_parser::expr::infix::Infix::BitShiftL => "(<<)".to_string(),
                    sap_parser::expr::infix::Infix::BitShiftR => "(>>)".to_string(),
                    sap_parser::expr::infix::Infix::AssignYield => {
                        // sugar of e0 = <- e1
                        return Flatten::Assign(
                            Box::new(Flatten::from(*expr, filename, prompt_label)),
                            Box::new(Flatten::Yield(
                                Box::new(Flatten::from(*expr1, filename, prompt_label)),
                                info,
                            )),
                            info,
                        );
                    }
                    sap_parser::expr::infix::Infix::Extends => {
                        return Flatten::Extend(
                            Box::new(Flatten::from(*expr, filename, prompt_label)),
                            Box::new(Flatten::from(*expr1, filename, prompt_label)),
                            info,
                        );
                    }
                    sap_parser::expr::infix::Infix::Function(id) => {
                        return Flatten::Apply(
                            Box::new(Flatten::Id(id.value(), info)),
                            vec![
                                Flatten::from(*expr, filename, prompt_label),
                                Flatten::from(*expr1, filename, prompt_label),
                            ],
                            info,
                        );
                    }
                };
                Flatten::Apply(
                    Box::new(Flatten::Id(infix_id, info)),
                    vec![
                        Flatten::from(*expr, filename, prompt_label),
                        Flatten::from(*expr1, filename, prompt_label),
                    ],
                    info,
                )
            }
            ExprInner::CApply(expr, vec) => Flatten::Apply(
                Box::new(Flatten::from(*expr, filename, prompt_label)),
                vec.into_iter()
                    .map(|x| Flatten::from(x, filename, prompt_label))
                    .collect(),
                info,
            ),
            // remove the void literal from the apply
            ExprInner::MLApply(expr, vec) => {
                if vec.len() == 1
                    && matches!(
                        vec.first().unwrap().inner,
                        ExprInner::Primary(Primary {
                            inner: PrimaryInner::CompoundLiteral(
                                sap_parser::literal::CompoundLiteral::Literal(Literal {
                                    inner: LiteralInner::Void(_),
                                    ..
                                })
                            ),
                            ..
                        }),
                    )
                {
                    Flatten::Apply(
                        Box::new(Flatten::from(*expr, filename, prompt_label)),
                        vec![],
                        info,
                    )
                } else {
                    Flatten::Apply(
                        Box::new(Flatten::from(*expr, filename, prompt_label)),
                        vec.into_iter()
                            .map(|x| Flatten::from(x, filename, prompt_label))
                            .collect(),
                        info,
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error_reporting::{CompilePrompt, init_reporter, push_prompt, report_error};

    #[test]
    fn test() {
        init_reporter();
        let source = "a = \\ ^[1,...b, ...c, 2.3] -> b \\-> ()";
        let expr = sap_parser::parse_expr(source).unwrap();
        let filename = None;
        let prompt_label = &mut vec![];
        let ops_to_apply = super::Flatten::from(expr, filename, prompt_label);
        println!("{:#?}\n\n", ops_to_apply);
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
