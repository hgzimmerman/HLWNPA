# HLWNPA - Henry's Language With No Pronounceable Acronym
I figured that I could write a programming language in the timespan of about a week, with 0 thought put into it beforehand, and maybe get a workable result.
This is that possibly workable result.
I did this just to see what I could acomplish with no prior experience with language design or compiler theory.

# Process
I defined what the AST should look like, then defined a syntax that would parse input to that AST, and then implemented a REPL.
I then proceeded to graft things onto the AST and syntax parser once I had a minimal language.



# Features

* Nothing to limit reassignment. If you want to assign a number to a function name, there is nothing stopping you.
* No meaningful parser error messages. If your syntax isn't 100% accurate, there is very little to indicate what you did wrong. Sometimes the program will parse, but may leave out a section of the AST without errors if syntax isn't exact. The result is if you define a function incorrectly, that function will not exist, and you won't know until you try to call it.
* A few runtime error messages.
* No early return from functions. The last statement in the body of a function, if, or loop block will be returned.
* Type System. Runtime checking only.
* Statements are delimited by `()`, because the language has no concept of order of operations.
* Functions and structs must be declared in the order they are used.
* I don't think I have support for functions returning arrays.
* No floating point support, only signed 32 bit numbers.


# Actual Features
* REPL.
* Supports Functions, while loops, if statements, as well as the primative types: Number (signed 32), String, Booleans, and Arrays (partially). As well as Structs.
* Assignment looks like: `let value := ( 4 * 6 )`


# Example Program
### Program 1
```
let x := 5
fn add_2_to_number( number: Number) -> Number {
  ( number + 2 )
}
add_2_to_number(x)
```
Returns: `Number(7)`

### Program 2
```
struct MyStruct {
    a : Number
    b : Number
}

let a := 3

fn create_new_MyStruct( value: Number ) -> MyStruct {
    new MyStruct {
        a: 8
        b: value
    }
}

fn addContents( s: MyStruct ) -> Number {
    (s.a + s.b)
}

let instance := create_new_MyStruct( a )

addContents( instance )
```
Returns: `Number(11)`

# TODO
* Switch to using S-Expressions, where each operator holds one or more operators or literals. This would replace the current implementation that just has binary and unary expressions that hold the operator and its operands.
* Introduce mutability rules. `const` vs. `let`.
* Prevent reassignment of Struct and Function names. Currently, you are allowed to set the identifier for a struct's type to be a number, this has wonky concequences for the type system.
* Investigate Nom's custom error messages. 
* Figure out how to display a line number for a parser error and highlight the part of syntax that failed.
* Flesh out the runtime error messages, give them more data related to the error, and implement Display for them so they are printed out nicely when an error ocurrs.
* When executing a file, hoist the functions and struct declarations, search for a main function, evaluate it if found, otherwise, evaluate AST nodes that exist outside of functions. If the file only contains functions and structs and no main function, throw an error.
* Allow the REPL to read a file at startup and access its functions, structs, and variables.
* Implement an `Include <filename>` keyword that will parse another file and alter the AST accordingly.
* Possibly implement loop unrolling inside of functions, as well as precomputation of S-Expressions with literals. So code that looks like `let a := 3 + 8` would be optimized to `let a := 11` if it exists within a function.
