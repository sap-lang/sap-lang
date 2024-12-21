#![feature(box_patterns)]

use std::io::Write;

pub mod ast;
pub mod error_diag;
pub mod parser;

pub mod backend;

fn main() {
    let source = include_str!("../examples/object.sap");
    let ast = parser::parse(source);
    let mut file = std::fs::File::create("test.js").unwrap();
    let code = backend::js::compile(ast);
    file.write_all(code.as_bytes()).unwrap();
}
