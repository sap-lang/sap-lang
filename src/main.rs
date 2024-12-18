#![feature(box_patterns)]

use std::io::Write;

pub mod ast;
pub mod error_diag;
pub mod parser;

pub mod backend;

fn main() {
    let source = r###"{
        // fib ::= \0 -> 0
        // fib ::= \1 -> 1
        // fib ::= \ x -> fib (x-1) + fib (x-2)
        // log (fib 10)

        cont = \x -> {
            (+) ::= \a b -> __op_add__ (a * 2) b

            log (<- x + 1)
            log (<- x + 2)
            log (<- x + 3)
            x + 4
        }
        a11 -> cont = cont 10
        log a11
        a12 -> cont = cont 11
        log a12
        a13 -> cont = cont 12
        log a13
        _a ->  cont = cont 13
        
        log _a
        log cont
        log (1 + 1)

    }"###;
    let ast = parser::parse(source);
    let mut file = std::fs::File::create("test.js").unwrap();
    let code = backend::js::compile(ast);
    file.write_all(code.as_bytes()).unwrap();
}
