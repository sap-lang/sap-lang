#![feature(box_patterns)]

use std::io::Write;

pub mod ast;
pub mod error_diag;
pub mod parser;

pub mod backend;

fn main() {
    let fib = r###"{
        fib ::= \0 -> 0
        fib ::= \1 -> 1
        fib ::= \ x -> fib (x-1) + fib (x-2)
        log (fib 10)
    }"###;

    let cont = r###"{
        cont = \x -> {
            // operator overloading
            (+) ::= \a b -> __op_add__ (a * 2) b
            // will effect current scope only
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

    let slice = r###"{
        v = [1,2,3,4,5,6,7,8,9][1::3]
        log v
    }"###;

    let ast = parser::parse(slice);
    let mut file = std::fs::File::create("test.js").unwrap();
    let code = backend::js::compile(ast);
    file.write_all(code.as_bytes()).unwrap();
}
