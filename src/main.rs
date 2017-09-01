#![feature(discriminant_value)]
#![feature(trace_macros)]

#![macro_use]
extern crate nom;

use std::boxed::Box;
use std::collections::HashMap;

mod datatype;
mod lang_result;
mod ast;
mod parser;

use lang_result::*;
use datatype::{Datatype, TypeInfo};
use ast::*;

use parser::function;


fn main() {

    let mut identifier_map: HashMap<String, Datatype> = HashMap::new();

    let ast = Ast::UnaryExpression {
        operator: UnaryOperator::Print,
        expr: Box::new(Ast::VecExpression {
            expressions: vec![
                Ast::Expression {
                    operator: BinaryOperator::Assignment,
                    expr1: Box::new(Ast::ValueIdentifier { ident: "a".to_string() }),
                    expr2: Box::new(Ast::Literal { datatype: Datatype::Number(6) }),
                },
                Ast::Expression {
                    operator: BinaryOperator::Plus,
                    expr1: Box::new(Ast::ValueIdentifier { ident: "a".to_string() }),
                    expr2: Box::new(Ast::Literal { datatype: Datatype::Number(5) }),
                },
            ],
        }),
    };


    let _ = ast.evaluate(&mut identifier_map);
}






#[test]
fn simple_function_parse_and_execute_integration_test() {
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
