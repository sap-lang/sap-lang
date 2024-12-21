use pest::Span;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum SapParserErrorCode {
    NOP = 0,

    InvalidLambdaExpr,
    InvalidKVPair,

    ExpectedId,

    AssignExprLHSNotAssignable,
    LHSIsNotSlotNorReferring,
    PatternShouldNotBeOperand,
}

#[derive(Error, Debug, PartialEq, Clone, Serialize)]
pub struct SapParserError {
    pub span: SapDiagnosticSpan,
    pub code: SapParserErrorCode,
    pub message: String,
}

impl std::fmt::Display for SapParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser error at {:?}", self.span)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SapDiagnosticSpan {
    pub start_line: usize,
    pub start_col: usize,
    pub start_offset: usize,

    pub end_line: usize,
    pub end_col: usize,
    pub end_offset: usize,

    pub source: String,
}

impl SapDiagnosticSpan {
    pub fn from_pest_span(span: &Span<'_>) -> Self {
        let (start_line, start_col) = span.start_pos().line_col();
        let (end_line, end_col) = span.end_pos().line_col();
        let start_offset = span.start();
        let end_offset = span.end();
        let source = span.as_str().to_string();
        Self {
            start_line,
            start_col,
            start_offset,
            end_line,
            end_col,
            end_offset,
            source,
        }
    }
}

impl ariadne::Span for SapDiagnosticSpan {
    type SourceId = String;

    fn source(&self) -> &Self::SourceId {
        &self.source
    }

    fn start(&self) -> usize {
        self.start_offset
    }

    fn end(&self) -> usize {
        self.end_offset
    }
}
