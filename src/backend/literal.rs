use crate::parser::literal::{Literal, number::Number, string::StringLiteral};

use super::compile_inner;

pub fn compile_literal(literal: Literal) -> String {
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
