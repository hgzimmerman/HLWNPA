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


    let _ = ast.evaluate_ast(&mut identifier_map);
}







#[test]
fn plus_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Plus,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(6) }),
    };
    assert_eq!(Datatype::Number(9), ast.evaluate_ast(&mut map).unwrap())
}

#[test]
fn string_plus_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Plus,
        expr1: Box::new(Ast::Literal {
            datatype: Datatype::String("Hello".to_string()),
        }),
        expr2: Box::new(Ast::Literal {
            datatype: Datatype::String(" World!".to_string()),
        }),
    };
    assert_eq!(
        Datatype::String("Hello World!".to_string()),
        ast.evaluate_ast(&mut map).unwrap()
    )
}

#[test]
fn minus_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Minus,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(6) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
    };
    assert_eq!(Datatype::Number(3), ast.evaluate_ast(&mut map).unwrap())
}

#[test]
fn minus_negative_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Minus,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(6) }),
    };
    assert_eq!(Datatype::Number(-3), ast.evaluate_ast(&mut map).unwrap())
}

#[test]
fn multiplication_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Multiply,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(6) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
    };
    assert_eq!(Datatype::Number(18), ast.evaluate_ast(&mut map).unwrap())
}

#[test]
fn division_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Divide,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(6) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
    };
    assert_eq!(Datatype::Number(2), ast.evaluate_ast(&mut map).unwrap())
}

#[test]
fn integer_division_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Divide,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(5) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
    };
    assert_eq!(Datatype::Number(1), ast.evaluate_ast(&mut map).unwrap())
}

#[test]
fn division_by_zero_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Divide,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(5) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(0) }),
    };
    assert_eq!(
        LangError::DivideByZero,
        ast.evaluate_ast(&mut map).err().unwrap()
    )
}

#[test]
fn modulo_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Modulo,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(8) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
    };
    assert_eq!(Datatype::Number(2), ast.evaluate_ast(&mut map).unwrap())
}

#[test]
fn equality_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Equals,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
    };
    assert_eq!(Datatype::Bool(true), ast.evaluate_ast(&mut map).unwrap())
}

#[test]
fn greater_than_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::GreaterThan,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(4) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
    };
    assert_eq!(Datatype::Bool(true), ast.evaluate_ast(&mut map).unwrap())
}

#[test]
fn less_than_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::LessThan,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(2) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
    };
    assert_eq!(Datatype::Bool(true), ast.evaluate_ast(&mut map).unwrap())
}



/// Assign the value 6 to the identifier "a"
/// Recall that identifier and add it to 5
#[test]
fn assignment_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::VecExpression {
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
    };
    assert_eq!(Datatype::Number(11), ast.evaluate_ast(&mut map).unwrap())
}



/// Assign the value 6 to "a".
/// Copy the value in "a" to "b".
/// Recall the value in "b" and add it to 5.
#[test]
fn variable_copy_test() {

    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::VecExpression {
        expressions: vec![
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier { ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal { datatype: Datatype::Number(6) }),
            },
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier { ident: "b".to_string() }),
                expr2: Box::new(Ast::ValueIdentifier { ident: "a".to_string() }),
            },
            Ast::Expression {
                operator: BinaryOperator::Plus,
                expr1: Box::new(Ast::ValueIdentifier { ident: "b".to_string() }),
                expr2: Box::new(Ast::Literal { datatype: Datatype::Number(5) }),
            },
        ],
    };
    assert_eq!(Datatype::Number(11), ast.evaluate_ast(&mut map).unwrap())
}

/// Assign the value 6 to a.
/// Assign the value 3 to a.
/// Recall the value in a and add it to 5, the value of a should be 3, equalling 8.
#[test]
fn reassignment_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::VecExpression {
        expressions: vec![
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier { ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal { datatype: Datatype::Number(6) }),
            },
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier { ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
            },
            Ast::Expression {
                operator: BinaryOperator::Plus,
                expr1: Box::new(Ast::ValueIdentifier { ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal { datatype: Datatype::Number(5) }),
            },
        ],
    };
    assert_eq!(Datatype::Number(8), ast.evaluate_ast(&mut map).unwrap())
}

#[test]
fn conditional_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Conditional {
        condition: Box::new(Ast::Literal { datatype: Datatype::Bool(true) }),
        true_expr: Box::new(Ast::Literal { datatype: Datatype::Number(7) }),
        false_expr: None,
    };
    assert_eq!(Datatype::Number(7), ast.evaluate_ast(&mut map).unwrap())
}

#[test]
fn conditional_with_else_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Conditional {
        condition: Box::new(Ast::Literal { datatype: Datatype::Bool(false) }),
        true_expr: Box::new(Ast::Literal { datatype: Datatype::Number(7) }),
        false_expr: Some(Box::new(Ast::Literal { datatype: Datatype::Number(2) })),
    };
    assert_eq!(Datatype::Number(2), ast.evaluate_ast(&mut map).unwrap())
}

#[test]
fn basic_function_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::VecExpression {
        expressions: vec![
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier { ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {
                    datatype: Datatype::Function {
                        parameters: Box::new(Ast::VecExpression { expressions: vec![] }), // empty parameters
                        body: (Box::new(Ast::Literal { datatype: Datatype::Number(32) })), // just return a number
                        return_type: Box::new(TypeInfo::Number), // expect a number
                    },
                }),
            },
            Ast::Expression {
                operator: BinaryOperator::ExecuteFn,
                expr1: Box::new(Ast::ValueIdentifier { ident: "a".to_string() }), // get the identifier for a
                expr2: Box::new(Ast::VecExpression { expressions: vec![] }), // provide the function parameters
            },
        ],
    };
    assert_eq!(Datatype::Number(32), ast.evaluate_ast(&mut map).unwrap())
}

#[test]
fn function_with_parameter_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::VecExpression {
        expressions: vec![
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier { ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {
                    datatype: Datatype::Function {
                        parameters: Box::new(Ast::VecExpression {
                            expressions: vec![
                                Ast::Expression {
                                    operator: BinaryOperator::FunctionParameterAssignment,
                                    expr1: Box::new(
                                        Ast::ValueIdentifier { ident: "b".to_string() }
                                    ), // the value's name is b
                                    expr2: Box::new(Ast::Type { datatype: TypeInfo::Number }), // fn takes a number
                                },
                            ],
                        }),
                        body: (Box::new(Ast::ValueIdentifier { ident: "b".to_string() })), // just return the number passed in.
                        return_type: Box::new(TypeInfo::Number), // expect a number to be returned
                    },
                }),
            },
            Ast::Expression {
                operator: BinaryOperator::ExecuteFn,
                expr1: Box::new(Ast::ValueIdentifier { ident: "a".to_string() }), // get the identifier for a
                expr2: Box::new(Ast::VecExpression {
                    expressions: vec![Ast::Literal { datatype: Datatype::Number(7) }],
                }), // provide the function parameters
            },
        ],
    };
    assert_eq!(Datatype::Number(7), ast.evaluate_ast(&mut map).unwrap())
}



#[test]
fn function_with_two_parameters_addition_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::VecExpression {
        expressions: vec![
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier {
                    ident: "add_two_numbers".to_string(),
                }),
                expr2: Box::new(Ast::Literal {
                    datatype: Datatype::Function {
                        parameters: Box::new(Ast::VecExpression {
                            expressions: vec![
                                Ast::Expression {
                                    operator: BinaryOperator::FunctionParameterAssignment,
                                    expr1: Box::new(
                                        Ast::ValueIdentifier { ident: "b".to_string() }
                                    ), // the value's name is b
                                    expr2: Box::new(Ast::Type { datatype: TypeInfo::Number }), // fn takes a number
                                },
                                Ast::Expression {
                                    operator: BinaryOperator::FunctionParameterAssignment,
                                    expr1: Box::new(
                                        Ast::ValueIdentifier { ident: "c".to_string() }
                                    ), // the value's name is b
                                    expr2: Box::new(Ast::Type  { datatype: TypeInfo::Number }), // fn takes a number
                                },
                            ],
                        }),
                        body: (Box::new(Ast::Expression {
                            // the body of the function will add the two passed in values together
                            operator: BinaryOperator::Plus,
                            expr1: Box::new(Ast::ValueIdentifier { ident: "b".to_string() }),
                            expr2: Box::new(Ast::ValueIdentifier { ident: "c".to_string() }),
                        })),

                        return_type: Box::new(TypeInfo::Number), // expect a number to be returned
                    },
                }),
            },
            Ast::Expression {
                operator: BinaryOperator::ExecuteFn,
                expr1: Box::new(Ast::ValueIdentifier {
                    ident: "add_two_numbers".to_string(),
                }), // get the identifier for a
                expr2: Box::new(Ast::VecExpression {
                    expressions: vec![
                        Ast::Literal { datatype: Datatype::Number(7) },
                        Ast::Literal { datatype: Datatype::Number(5) },
                    ],
                }), // provide the function parameters
            },
        ],
    };
    assert_eq!(Datatype::Number(12), ast.evaluate_ast(&mut map).unwrap())
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

    ast_with_function.evaluate_ast(&mut map); // insert the function into the hashmap

    let executor_ast: Ast = Ast::Expression {
        operator: BinaryOperator::ExecuteFn,
        expr1: Box::new(Ast::ValueIdentifier {ident: "add8ToValue".to_string()}),
        expr2: Box::new(Ast::VecExpression {expressions: vec![
            Ast::Literal {datatype: Datatype::Number(7)}
        ]})
    };

    assert_eq!(Datatype::Number(15), executor_ast.evaluate_ast(&mut map).unwrap()); // find the test function and pass the value 7 into it

}