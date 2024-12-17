

pub mod ast;
pub mod error_diag;
pub mod parser;

fn main() {
    let source = r###"fib ::= \x -> fib (x - 1) + fib (x - 2)"###;
    parser::parse(source);
}
