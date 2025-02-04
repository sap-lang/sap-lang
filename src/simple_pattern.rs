use sap_parser::{
    id::{Id, NormalId},
    pattern::{Pattern, object::ObjectPatternKv},
};

use crate::{
    error_reporting::{CompilePromptCode, CompilePromptLabel},
    passes::CommonMetaInfo,
    simple_literal::SimpleLiteral,
};

#[derive(Debug, Clone)]
pub enum SimplePattern {
    Id(String),
    Literal(SimpleLiteral),
    Array(Vec<SimplePattern>, Option<String>),
    Object(Vec<(String, SimplePattern)>, Option<String>),
}

impl SimplePattern {
    pub fn from(
        value: Pattern,
        filename: Option<&'static str>,
        prompt_labels: &mut Vec<CompilePromptLabel>,
    ) -> Self {
        match value {
            Pattern::Id(id) => SimplePattern::Id(id.value()),
            Pattern::Literal(literal) => {
                let diag = literal.diag;
                let literal = SimpleLiteral::from(literal);
                if let SimpleLiteral::Float(_) = literal {
                    let info = CommonMetaInfo::new(diag, filename);
                    prompt_labels.push(CompilePromptLabel::new(
                        CompilePromptCode::FloatLiteralCouldNotBeUsedAsPattern,
                        info,
                    ));
                }
                SimplePattern::Literal(literal)
            }
            Pattern::ArrayPattern(array_pattern) => {
                let mut elems = vec![];
                let mut eclipse = None;
                for elem in array_pattern.body.elems {
                    match elem {
                        sap_parser::pattern::array::ArrayPatternElem::EclipsePattern(
                            eclipse_pattern,
                        ) => {
                            if eclipse.is_some() {
                                let diag = eclipse_pattern.diag;
                                let info = CommonMetaInfo::new(diag, filename);
                                prompt_labels.push(CompilePromptLabel::new(
                                    CompilePromptCode::MoreThanOneEclipsePatternInPattern,
                                    info,
                                ));
                            }
                            eclipse = Some(eclipse_pattern.value.value());
                        }
                        sap_parser::pattern::array::ArrayPatternElem::Pattern(pattern) => {
                            elems.push(SimplePattern::from(pattern, filename, prompt_labels));
                        }
                    }
                }
                SimplePattern::Array(elems, eclipse)
            }
            Pattern::ObjectPattern(object_pattern) => {
                let mut elems = vec![];
                let mut eclipse = None;
                for elem in object_pattern.body.body {
                    match elem {
                        sap_parser::pattern::object::ObjectPatternElem::ObjectPatternKv(
                            ObjectPatternKv { key, value },
                        ) => {
                            let value = value.unwrap_or(Pattern::Id(Id::NormalId(NormalId {
                                value: key.value().clone(),
                            })));
                            elems.push((
                                key.value(),
                                SimplePattern::from(value, filename, prompt_labels),
                            ));
                        }
                        sap_parser::pattern::object::ObjectPatternElem::EclipsePattern(
                            eclipse_pattern,
                        ) => {
                            if eclipse.is_some() {
                                let diag = eclipse_pattern.diag;
                                let info = CommonMetaInfo::new(diag, filename);
                                prompt_labels.push(CompilePromptLabel::new(
                                    CompilePromptCode::MoreThanOneEclipsePatternInPattern,
                                    info,
                                ));
                            }
                            eclipse = Some(eclipse_pattern.value.value());
                        }
                    }
                }
                SimplePattern::Object(elems, eclipse)
            }
        }
    }
}
