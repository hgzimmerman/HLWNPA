#![feature(discriminant_value)]
#![feature(trace_macros)]
#![feature(test)]

#![macro_use]
extern crate nom;
extern crate test;

use test::Bencher;

use std::boxed::Box;
use std::collections::HashMap;

mod datatype;
mod lang_result;
mod ast;
mod parser;
mod repl;

use lang_result::*;
use datatype::{Datatype, TypeInfo};
use ast::*;
use repl::repl;

use parser::{function, program};


fn main() {
    repl()
}



#[test]
fn function_parse_and_execute_separately_integration_test() {
    use nom::IResult;
    let mut map: HashMap<String, Datatype> = HashMap::new();

    let input_string = "fn add8ToValue ( a : Number ) -> Number { ( + a 8 ) }";
    let (_, ast_with_function) = match function(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };

    let _ = ast_with_function.evaluate(&mut map); // insert the function into the hashmap

    let executor_ast: Ast = Ast::Expression {
        operator: BinaryOperator::ExecuteFn,
        expr1: Box::new(Ast::ValueIdentifier { ident: "add8ToValue".to_string() }),
        expr2: Box::new(Ast::VecExpression {
            expressions: vec![Ast::Literal { datatype: Datatype::Number(7) }],
        }),
    };

    assert_eq!(
        Datatype::Number(15),
        executor_ast.evaluate(&mut map).unwrap()
    ); // find the test function and pass the value 7 into it
}


#[test]
fn program_parse_and_execute_integration_test_1() {
    use nom::IResult;
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let input_string = "
     let x 7
     fn test_function ( a : Number ) -> Number { ( + a 8 ) }
     test_function(x)";
    let (_, ast) = match program(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };

    assert_eq!(
    Datatype::Number(15),
    ast.evaluate(&mut map).unwrap()
    );
}


#[test]
fn program_parse_and_execute_integration_test_2() {
    use nom::IResult;
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let input_string = "
     fn test_function ( a : Number ) -> Number { ( + a 8 ) }
     test_function(8)";
    let (_, ast) = match program(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };

    assert_eq!(
    Datatype::Number(16),
    ast.evaluate(&mut map).unwrap()
    );
}

#[test]
fn program_parse_and_execute_integration_test_3() {
    use nom::IResult;
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let input_string = "
     fn test_function ( a : Number ) -> Number { ( + a 8 ) }
     test_function( ( + 6 2) )";
    let (_, ast) = match program(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };



    assert_eq!(
    Datatype::Number(16),
    ast.evaluate(&mut map).unwrap()
    );
}

/// Test multiple line functions
#[test]
fn program_parse_and_execute_integration_test_4() {
    use nom::IResult;
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let input_string = "
     fn test_function ( a : Number ) -> Number {
        ( + a 8 )
     }
     test_function(8)";
    let (_, ast) = match program(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };

    assert_eq!(
    Datatype::Number(16),
    ast.evaluate(&mut map).unwrap()
    );
}

#[test]
fn program_multiple_parameter_function_integration_test() {
    use nom::IResult;
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let input_string = "
     fn add_two_numbers ( a : Number b : Number) -> Number {
        ( + a b )
     }
     add_two_numbers(8 3)";
    let (_, ast) = match program(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };

    assert_eq!(
    Datatype::Number(11),
    ast.evaluate(&mut map).unwrap()
    );
}


#[test]
fn program_function_internals_does_not_clobber_outer_stack_integration_test() {
    use nom::IResult;
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let input_string = "
     let a 2
     fn add_two_numbers ( a : Number b : Number) -> Number {
        ( + a b )
     }
     add_two_numbers(8 3)
     (+ a 0)
     ";
    let (_, ast) = match program(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };

    assert_eq!(
    Datatype::Number(2),
    ast.evaluate(&mut map).unwrap()
    );
}


/// Test the assignment of a string, then passing it into a function that takes a string.
/// The function should then add a number to the string, creating a new string.
#[test]
fn program_string_coercion_integration_test() {
    use nom::IResult;
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let input_string = r##"
     let x "Hi "
     fn test_function ( a : String ) -> String { ( + a 5 ) }
     test_function(x)"##;
    let (_, ast) = match program(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };

    assert_eq!(
    Datatype::String("Hi 5".to_string()),
    ast.evaluate(&mut map).unwrap()
    );
}


#[bench]
fn simple_program_bench(b: &mut Bencher) {
    b.iter(|| program_string_coercion_integration_test())
}
