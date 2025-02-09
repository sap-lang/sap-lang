use sap_parser::literal::Literal;
use sap_parser::literal::Inner as LiteralInner;


#[derive(Debug, Clone, PartialEq)]
pub enum SimpleLiteral {
    Void,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}


impl From<Literal> for SimpleLiteral {
    fn from(value: Literal) -> Self {
        match value.inner {
            LiteralInner::Boolean(boolean) => SimpleLiteral::Bool(boolean.value),
            LiteralInner::Void(_) => SimpleLiteral::Void,
            LiteralInner::String(sap_string) => SimpleLiteral::String(sap_string.value()),
            LiteralInner::Number(sap_number) => match sap_number {
                sap_parser::literal::number::SapNumber::Int(int) => SimpleLiteral::Int(int.value()),
                sap_parser::literal::number::SapNumber::Float(float) => {
                    SimpleLiteral::Float(float.value())
                }
            },
        }
    }
}