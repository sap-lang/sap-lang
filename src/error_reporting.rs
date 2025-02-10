use std::{
    collections::VecDeque,
    sync::{
        Mutex, OnceLock,
    },
};

use crate::passes::CommonMetaInfo;

static __PROMPTS: OnceLock<Mutex<VecDeque<CompilePrompt>>> = OnceLock::new();

pub fn init_reporter() {
    __PROMPTS.get_or_init(|| Mutex::new(VecDeque::new()));
}

pub fn push_prompt(prompt: CompilePrompt) {
    let l = __PROMPTS.get().unwrap();
    let mut lock = l.lock().unwrap();
    lock.push_back(prompt);
}

pub fn report_error(reporter: impl Fn(&VecDeque<CompilePrompt>)) {
    let l = __PROMPTS.get().unwrap();
    let lock = l.lock().unwrap();
    reporter(&lock);
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum CompilePromptCode {
    // errors
    Error = 0,
    InternalErrorShouldBeFlatten,
    ExpectedPatternFindExpression,
    ExpectedLambdaExpr,
    ExpectedExpressionFindPattern,
    ExpectedValidLHS,
    Sap0DoesNotSupportMacro,
    PatternShouldBeUsed,
    SetShouldOnlyBeUsedForAccess,
    MoreThanOneEclipsePatternInPattern,
    PatternAfterEclipsePattern,
    
    // warnings
    Warning = 1000,
    FloatLiteralCouldNotBeUsedAsPattern,
    DuplicateKeyInObject,

    // suggestions
    Suggestion = 2000,
    EmptyBlockAsVoid,
    BlockShouldBeRemovedOrUsingParentheses,
    BetterUsingNoParamLambdaExpr
}

#[derive(Debug, Clone, Copy)]
pub struct CompilePromptLabel {
    pub code: CompilePromptCode,
    pub info: CommonMetaInfo,
}

impl CompilePromptLabel {
    pub fn new(code: CompilePromptCode, info: CommonMetaInfo) -> Self {
        Self { code, info }
    }
}

#[derive(Debug, Clone)]
pub struct CompilePrompt {
    pub labels: Vec<CompilePromptLabel>,
    pub info: CommonMetaInfo,
}

impl CompilePrompt {
    pub fn new(labels: Vec<CompilePromptLabel>, info: CommonMetaInfo) -> Self {
        Self { labels, info }
    }
}

#[derive(Debug, Clone)]
pub enum RuntimeErrors {
    CustomError(String),

    // internal panics
    DestructorPatternMatchFailed,

    // internal proccessable errors
    FunctionGuardOrMatchGuardFailed,
}
