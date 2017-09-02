# rust-lisp-thing
I figured that I could write a programming language in the timespan of a week, with 0 thought put into it beforehand, and maybe get a workable result.
This is that possibly workable result.

# Process
I defined what the AST should look like, then defined a syntax that would parse input to that AST, and then implemented a REPL.
I did this just to see what I could acomplish with no prior experience with language design or compiler theory.


# Features
* No arrays, structs, or objects. (coming soon?)
* Nothing to limit reassignment. If you want to assign a number to a function name, there is nothing stopping you.
* No meaningful parser error messages. If your syntax isn't 100% accurate, there is very little to indicate what you did wrong.
* A few runtime error messages.
* No control flow. Conditional evaluation exists in the AST, but nothing parses it yet. (Coming soon?)
* Uses reverse polish notation for expressions: `let value (* 4 6)`
* No early return from functions. This is ok because there are no control flow systems, but only the last expression in a function is returned.

# Actual Features
* Uuuuh, it has a REPL.
* I guess this is a functional programming language, in that everything is pass by value, everything lives on what I guess I would call a stack, and all functions must return a value. Functions can't mutate any external state, so they must return anything that should be preserved. 


# Example Program
```
let x 5
fn add_2_to_number( number: Number) -> Number {
  (+ number 2)
}
add_2_to_number(x)
```
Returns: `Number(7)`
