# rust-lisp-thing
I figured that I could write a programming language in the timespan of about a week, with 0 thought put into it beforehand, and maybe get a workable result.
This is that possibly workable result.

# Process
I defined what the AST should look like, then defined a syntax that would parse input to that AST, and then implemented a REPL.
I then proceeded to graft things onto the AST and syntax parser once I had a minimal language.
I did this just to see what I could acomplish with no prior experience with language design or compiler theory.


# Features

* Nothing to limit reassignment. If you want to assign a number to a function name, there is nothing stopping you.
* No meaningful parser error messages. If your syntax isn't 100% accurate, there is very little to indicate what you did wrong. Sometimes the program will parse, but may leave out a section of the AST without errors if syntax isn't exact. The result is if you define a function incorrectly, that function will not exist, and you won't know until you try to call it.
* A few runtime error messages.
* No early return from functions. The last statement in the body of a function, if, or loop block will be returned.
* Type System. Runtime checking only.
* Statements are delimited by `()`, because the language has no concept of order of operations.
* Functions and structs must be declared in the order they are used.


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

# Note
Needless to say, don't actually try to use this.
