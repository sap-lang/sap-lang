#![feature(box_patterns)]

use std::io::Write;

pub mod ast;
pub mod error_diag;
pub mod parser;

pub mod backend;

fn main() {
    let source = include_str!("../examples/effects.sap");
    let ast = parser::parse(source);
    let mut file = std::fs::File::create("test.js").unwrap();
    let mut ast_file = std::fs::File::create("test.ast.json").unwrap();
    let ast_json = serde_json::to_string_pretty(&ast).unwrap();
    let code = backend::module::compile("../examples/effects.sap".to_string(), ast, false);
    file.write_all(code.as_bytes()).unwrap();
    ast_file.write_all(ast_json.as_bytes()).unwrap();
}
