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
        puts (fib 10)
    }"###;

    let cont = r###"{
        cont = \x -> {
            // operator overloading
            (+) ::= \a 0 : $a == "number" -> { puts "4" ; a }
            (+) ::= \0 b : $b == "number" -> { puts "3" ; b }
            (+) ::= \a b : $a == "number" -> { puts "2" ; __op_add__ (a * 2) b }
            (+) ::= \a b : $a == "string" -> { puts "1" ; __op_add__ a b }

            // will effect current scope only
            puts ("x: " + x)
            puts ("0+1: " + (0 + 1))
            puts ("c1: " + (<- x + 1))
            puts ("c2: " + (<- x + 2))
            x + 4
        }
        a11 -> cont = cont 10
        puts ("a11: " + a11)
        a12 -> cont = cont 11
        puts ("a12: " + a12)
        _a -> cont = cont 12
        puts _a
        puts cont
        puts (1 + 1)
    }"###;

    let slice = r###"{
        v = [1,2,3,4,5,6,7,8,9][1::3]
        puts v
    }"###;

    let simple_cont = r###"{
        f = \x -> {
            <- x + 2
            <- x + 3
            <- x + 4
        }

        g = \y -> {
            <- y + 1
            <<- f y
            <- y + 5
        }

        a = 0
        a -> next = g a
        puts a
        a -> next = g a
        puts a
        a -> next = g a
        puts a
        a -> next = g a
        puts a
        a -> next = g a
        puts a
        a -> next = g a
        puts a
    }"###;

    let simple_coeffect = r###"{
        f = \x ? y -> {
            puts (x + y)
        }

        g = \ null -> {
            y = 10
            puts (f 1)
            puts (f 2)
        }

        y = 5
        puts (f 1)
        puts (f 2)

        g null
    }"###;

    let ast = parser::parse(simple_coeffect);
    let mut file = std::fs::File::create("test.js").unwrap();
    let code = backend::js::compile(ast);
    file.write_all(code.as_bytes()).unwrap();
}
