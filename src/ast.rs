use serde::Serialize;

use crate::{
    error_diag::{SapDiagnosticSpan, SapParserError},
    parser::{
        literal::Literal,
        pattern::Pattern,
        primary::{id::Id, lambda_expr::LambdaExpr},
    },
};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum SapASTBody {
    Error(SapParserError),
    Id(Id),
    LambdaExpr(LambdaExpr),
    Pattern(Pattern),
    Block(Vec<SapAST>),
    Literal(Literal),

    // prefix
    Typeof(Box<SapAST>),
    Not(Box<SapAST>),
    Neg(Box<SapAST>),
    BitNot(Box<SapAST>),
    Yield(Box<SapAST>),
    YieldChild(Box<SapAST>),

    // postfix

    // infix
    Assign(
        // pattern
        Box<SapAST>,
        // expr
        Box<SapAST>,
    ),
    AssignGetCont(
        // pattern
        Box<SapAST>,
        // cont
        Box<SapAST>,
        // expr
        Box<SapAST>,
    ),
    AssignSlot(
        // id or access
        Box<SapAST>,
        // expr
        Box<SapAST>,
    ),
    Add(Box<SapAST>, Box<SapAST>),
    Sub(Box<SapAST>, Box<SapAST>),
    Mul(Box<SapAST>, Box<SapAST>),
    Div(Box<SapAST>, Box<SapAST>),
    Mod(Box<SapAST>, Box<SapAST>),
    Eq(Box<SapAST>, Box<SapAST>),
    Neq(Box<SapAST>, Box<SapAST>),
    Extends(Box<SapAST>, Box<SapAST>),
    Le(Box<SapAST>, Box<SapAST>),
    Ge(Box<SapAST>, Box<SapAST>),
    Lt(Box<SapAST>, Box<SapAST>),
    Gt(Box<SapAST>, Box<SapAST>),
    And(Box<SapAST>, Box<SapAST>),
    Or(Box<SapAST>, Box<SapAST>),
    BitOr(Box<SapAST>, Box<SapAST>),
    BitAnd(Box<SapAST>, Box<SapAST>),
    BitXor(Box<SapAST>, Box<SapAST>),
    BitShiftL(Box<SapAST>, Box<SapAST>),
    BitShiftR(Box<SapAST>, Box<SapAST>),

    // chain
    If(
        // cond
        Box<SapAST>,
        // then
        Box<SapAST>,
        // else
        Box<SapAST>,
    ),

    Slice(
        // expr
        Box<SapAST>,
        // from expr
        Option<Box<SapAST>>,
        // to expr
        Option<Box<SapAST>>,
        // step expr
        Option<Box<SapAST>>,
    ),
    Access(
        // expr
        Box<SapAST>,
        // id
        Id,
    ),
    Index(
        // expr
        Box<SapAST>,
        // expr
        Box<SapAST>,
    ),
    App(
        // expr
        Box<SapAST>,
        // expr
        Vec<SapAST>,
    ),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct SapAST {
    pub span: SapDiagnosticSpan,
    pub body: SapASTBody,
}

impl SapAST {
    pub(crate) fn get_id(self) -> Id {
        match self.body {
            SapASTBody::Id(id) => id,
            _ => panic!("Expected Id, found {:?}", self.body),
        }
    }
}

impl SapAST {
    pub fn error(error: SapParserError) -> Self {
        Self {
            span: error.span.clone(),
            body: SapASTBody::Error(error),
        }
    }
}
