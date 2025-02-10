| id   | compile time execution      | macro                                        | type inference | emit       |
| ---- | --------------------------- | -------------------------------------------- | -------------- | ---------- |
| sap0 | no                          | no                                           | no             | js/quickjs |
| sap1 | yes, l0 will eval           | no                                           | no             | l0         |
| sap3 | yes, l0 will eval           | yes, l2 will expand                          | no             | l1         |
| saps | yes, l3 will eval           | yes, l3 will expand                          | yes            | l3         |
| sap  | yes, l4 will eval with type | yes, l4 will expand + other primitive macros | yes            | l4, other  |

## sap0
sap0 is a simple language with first class cofunctions, it is a transpiler to js/quickjs, it has no compile time execution, no macro, no type inference.

it is extremely useful for checking the nano-pass implementation of the language.

with the assertion of `sap-lang` made, the `sap0` can emit very fast and efficient js code.

## sap1
sap1 is sap0 with compile time execution, it can color which part of the code will be executed at compile time, and execute it in sap0.

## sap3
sap3 is sap2 with second order macro, it can expand second order macro.
at the same time, macro in compile time execution will be expanded.

## saps
saps is the `dev` platform of sap-lang, it is very useful for debugging the library and running the test cases.
in addition to sap3, it has type inference, it can infer the type of the variable and function.
at the runtime of saps, it likes `typescript` for `javascript`, it does the type checking at compile time, and erase the type information at runtime.

## sap
sap is the `prod` platform of sap-lang.
it has all the features of saps, and it is a static typed, compiled language,
the use case of sap is to `AOT` compile the program to `single-executable`.

### platform support
- `native`: emit `MLIR` code, and compile it to `LLVM IR` with Runtime static linking.
- `tinygo`: emit `go` code, with no go-lang's large ecosystem.
- `js`: emit `js` code.

