#![feature(discriminant_value)]
use std::mem::discriminant;

use std::boxed::Box;
use std::collections::HashMap;

mod datatype;
mod lang_result;

use lang_result::*;
use datatype::Datatype;


fn main() {

    let mut identifier_map: HashMap<String, Datatype> = HashMap::new();

    let ast = Ast::UnaryExpression  {
        operator: UnaryOperator::Print,
        expr: Box::new(Ast::VecExpression {
            expressions: vec!(
                Ast::Expression {
                    operator: BinaryOperator::Assignment,
                    expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                    expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 6 ) })
                },
                Ast::Expression {
                    operator: BinaryOperator::Plus,
                    expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                    expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 5 ) })
                }
             )
        })
    };


    let _ = evaluate_ast(ast, &mut identifier_map);
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Ast {
    Expression { operator: BinaryOperator, expr1: Box<Ast>, expr2: Box<Ast>  },
    UnaryExpression{ operator: UnaryOperator, expr: Box<Ast>},
    VecExpression { expressions: Vec<Ast>}, // uesd for structuring execution of the AST.
    Conditional {condition: Box<Ast>, true_expr: Box<Ast>, false_expr: Option<Box<Ast>>},
    Literal { datatype: Datatype }, // consider making the Literal another enum with supported default datatypes.
    ValueIdentifier { ident: String}, // gets the value mapped to a hashmap
}


#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equals,
    GreaterThan,
    LessThan,
    Assignment,
    ExecuteFn,
    FunctionParameterAssignment
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum UnaryOperator {
    Print,
    Invert,
    Increment,
    Decrement,
}


fn evaluate_ast(ast: Ast, map: &mut HashMap<String, Datatype>) -> LangResult {
    match ast {
        Ast::Expression {operator, expr1, expr2 } => {
            match operator {
                BinaryOperator::Plus => {
                    evaluate_ast(*expr1, map)? + evaluate_ast(*expr2, map)?
                },
                BinaryOperator::Minus => {
                    evaluate_ast(*expr1, map)? - evaluate_ast(*expr2, map)?
                },
                BinaryOperator::Multiply => {
                    evaluate_ast(*expr1, map)? * evaluate_ast(*expr2, map)?
                },
                BinaryOperator::Divide => {
                    evaluate_ast(*expr1, map)? / evaluate_ast(*expr2, map)?
                },
                BinaryOperator::Modulo => {
                    evaluate_ast(*expr1, map)? % evaluate_ast(*expr2, map)?
                },
                BinaryOperator::Equals => {
                   if evaluate_ast(*expr1, map)? == evaluate_ast(*expr2, map)? {
                       return Ok(Datatype::Bool(true))
                   } else {
                       return Ok(Datatype::Bool(false))
                   }
                },
                BinaryOperator::GreaterThan => {
                    if evaluate_ast(*expr1, map)? >= evaluate_ast(*expr2, map)? {
                        return Ok(Datatype::Bool(true))
                    } else {
                        return Ok(Datatype::Bool(false))
                    }
                },
                BinaryOperator::LessThan => {
                    if evaluate_ast(*expr1, map)? <= evaluate_ast(*expr2, map)? {
                        return Ok(Datatype::Bool(true))
                    } else {
                        return Ok(Datatype::Bool(false))
                    }
                },
                BinaryOperator::Assignment => {
                    if let Ast::ValueIdentifier{ident} = *expr1 {
                        let mut cloned_map = map.clone(); // since this is a clone, the required righthand expressions will be evaluated in their own 'stack', this modified hashmap will be cleaned up post assignment.
                        let evaluated_right_hand_side = evaluate_ast(*expr2, &mut cloned_map)?;
                        let cloned_evaluated_rhs = evaluated_right_hand_side.clone();
                        map.insert(ident, evaluated_right_hand_side);
                        return Ok(cloned_evaluated_rhs);
                    }
                    else { Err(LangError::IdentifierDoesntExist) }
                },
                BinaryOperator::FunctionParameterAssignment => { // does the same thing as assignment, but I want a separate type for this.
                    if let Ast::ValueIdentifier {ident} = *expr1 {
                        let mut cloned_map = map.clone(); // since this is a clone, the required righthand expressions will be evaluated in their own 'stack', this modified hashmap will be cleaned up post assignment.
                        let evaluated_right_hand_side = evaluate_ast(*expr2, &mut cloned_map)?;
                        let cloned_evaluated_rhs = evaluated_right_hand_side.clone();
                        map.insert(ident, evaluated_right_hand_side);
                        return Ok(cloned_evaluated_rhs);
                    } else { Err(LangError::IdentifierDoesntExist) }
                },
                BinaryOperator::ExecuteFn => {
                    // evaluate the parameters

                    let mut cloned_map = map.clone(); // clone the map, to create a temporary new "stack" for the life of the function

                    let evaluated_parameters: Vec<Datatype> = match *expr2 {
                        Ast::VecExpression {expressions} => {

                            let mut evaluated_expressions: Vec<Datatype> = vec!();
                            for e in expressions {
                                match evaluate_ast(e, &mut cloned_map) {
                                    Ok(dt) => evaluated_expressions.push(dt),
                                    Err(err) => return Err(err)
                                }
                            }
                            evaluated_expressions
                        }
                        _ => return Err(LangError::FunctionParametersShouldBeVecExpression)
                    };


                    match evaluate_ast(*expr1, &mut cloned_map)? {
                        Datatype::Function {parameters, body, output_type} => {
                            match *parameters {
                                // The parameters should be in the form VecExpression(expression_with_fn_assignment, expression_with_fn_assignment, ...) This way, functions should be able to support arbitrary numbers of parameters.
                                Ast::VecExpression{expressions} => {
                                    // zip the values of the evaluated parameters into the expected parameters for the function
                                    if evaluated_parameters.len() == expressions.len() {
                                        // create an ast::VecExpression that
                                        let rhs_replaced_with_evaluated_parameters_results: Vec<Result<Ast, LangError>> = expressions.iter().zip(evaluated_parameters).map(|expressions_with_parameters: (&Ast, Datatype)| {
                                            let (e, d) = expressions_with_parameters; // assign out of tuple.
                                            if let Ast::Expression {ref operator, ref expr1, ref expr2} = *e {
                                                let operator = operator.clone();
                                                let expr1 = expr1.clone();

                                                if operator == BinaryOperator::FunctionParameterAssignment {
                                                    return Ok(Ast::Expression {operator: operator, expr1: expr1, expr2: Box::new(Ast::Literal {datatype: d})}); // return a new FunctionParameterAssignment Expression with a replaced expr2.
                                                } else {
                                                    return Err(LangError::InvalidFunctionPrototypeFormatting)
                                                }
                                            } else {
                                                return Err(LangError::InvalidFunctionPrototypeFormatting)
                                            }
                                        }
                                        ).collect();


                                        for rhs in rhs_replaced_with_evaluated_parameters_results {
                                            let rhs = rhs?; // return the error if present
                                            evaluate_ast( rhs, &mut cloned_map)?; // create the assignment
                                        }


                                    } else {
                                       return Err(LangError::ParameterLengthMismatch)
                                    }

                                    // Evaluate the body of the
                                    let output = evaluate_ast(*body, &mut cloned_map)?;
                                    if discriminant(&output) == discriminant(&output_type) {
                                        return Ok(output)
                                    }
                                    else {
                                        return Err(LangError::ReturnTypeDoesNotMatchReturnValue)
                                    }
                                },
                                _ => return Err(LangError::ParserShouldHaveRejected) // The parser should have put the parameters in the form VecExpression(expression_with_assignment, expression_with_assignment, ...)
                            }
                        }
                        _ => Err(LangError::ExecuteNonFunction)
                    }
                }
            }
        },
        Ast::UnaryExpression {operator, expr} => {
            match operator {
                UnaryOperator::Print => {
                    print!("{:?}", evaluate_ast(*expr, map)?); // todo use: std::fmt::Display::fmt instead
                    Ok(Datatype::None)
                },
                UnaryOperator::Invert => {
                    match evaluate_ast(*expr, map)? {
                        Datatype::Bool(bool) => Ok(Datatype::Bool(!bool)),
                        _ => Err(LangError::InvertNonBoolean)
                    }
                },
                UnaryOperator::Increment => {
                    match evaluate_ast(*expr, map)? {
                        Datatype::Number(number) => Ok(Datatype::Number( number + 1 )),
                        _ => Err(LangError::IncrementNonNumber)
                    }
                },
                UnaryOperator::Decrement => {
                    match evaluate_ast(*expr, map)? {
                        Datatype::Number(number) => Ok(Datatype::Number( number - 1 )),
                        _ => Err(LangError::DecrementNonNumber)
                    }
                },

          }
        },
        //Evaluate multiple expressions and return the result of the last one.
        Ast::VecExpression {expressions} => {
            let mut val: Datatype = Datatype::None;
            for e in expressions {
                val = evaluate_ast(e, map)?;
            };
            Ok(val) // return the last evaluated expression;
        },
        Ast::Conditional {condition, true_expr, false_expr} => {
            match evaluate_ast(*condition, map)? {
                Datatype::Bool(bool) => {
                    match bool {
                        true => Ok(evaluate_ast(*true_expr, map)?),
                        false => {
                            match false_expr {
                                Some(e) =>  Ok(evaluate_ast(*e, map)?),
                                _ => {Ok(Datatype::None )}
                            }
                        }
                    }
                }
                _ => Err(LangError::ConditionOnNonBoolean)
            }
        },
        Ast::Literal {datatype} => { Ok(datatype) },
        Ast::ValueIdentifier {ident} => {
            match map.get(&ident) {
                Some(value) => Ok(value.clone()),
                None => panic!("Variable {} hasn't been assigned yet", ident) // identifier hasn't been assigned yet.
            }
        }
    }
}








#[test]
fn plus_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Plus,
        expr1: Box::new(Ast::Literal {datatype: Datatype::Number( 3)}),
        expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 6)})
    };
    assert_eq!(Datatype::Number(9), evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn string_plus_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Plus,
        expr1: Box::new(Ast::Literal {datatype: Datatype::String( "Hello".to_string())}),
        expr2: Box::new(Ast::Literal {datatype: Datatype::String( " World!".to_string())})
    };
    assert_eq!(Datatype::String("Hello World!".to_string()), evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn minus_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Minus,
        expr1: Box::new(Ast::Literal {datatype: Datatype::Number( 6 )}),
        expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 3 )})
    };
    assert_eq!(Datatype::Number(3), evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn minus_negative_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Minus,
        expr1: Box::new(Ast::Literal {datatype: Datatype::Number(3)}),
        expr2: Box::new(Ast::Literal {datatype: Datatype::Number(6)})
    };
    assert_eq!(Datatype::Number(-3), evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn multiplication_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Multiply,
        expr1: Box::new(Ast::Literal {datatype: Datatype::Number( 6)}),
        expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 3)})
    };
    assert_eq!(Datatype::Number(18), evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn division_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Divide,
        expr1: Box::new(Ast::Literal {datatype: Datatype::Number( 6)}),
        expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 3)})
    };
    assert_eq!(Datatype::Number(2), evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn integer_division_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Divide,
        expr1: Box::new(Ast::Literal {datatype: Datatype::Number(5)}),
        expr2: Box::new(Ast::Literal {datatype: Datatype::Number(3)})
    };
    assert_eq!(Datatype::Number(1), evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn division_by_zero_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Divide,
        expr1: Box::new(Ast::Literal {datatype: Datatype::Number( 5)}),
        expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 0)})
    };
    assert_eq!(LangError::DivideByZero, evaluate_ast(ast, &mut map).err().unwrap())
}

#[test]
fn modulo_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Modulo,
        expr1: Box::new(Ast::Literal {datatype: Datatype::Number( 8)}),
        expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 3)})
    };
    assert_eq!(Datatype::Number(2), evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn equality_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Equals,
        expr1: Box::new(Ast::Literal {datatype: Datatype::Number( 3)}),
        expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 3)})
    };
    assert_eq!(Datatype::Bool(true), evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn greater_than_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::GreaterThan,
        expr1: Box::new(Ast::Literal {datatype: Datatype::Number( 4)}),
        expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 3)})
    };
    assert_eq!(Datatype::Bool(true), evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn less_than_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::LessThan,
        expr1: Box::new(Ast::Literal {datatype: Datatype::Number( 2)}),
        expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 3)})
    };
    assert_eq!(Datatype::Bool(true), evaluate_ast(ast, &mut map).unwrap())
}





/// Assign the value 6 to the identifier "a"
/// Recall that identifier and add it to 5
#[test]
fn assignment_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::VecExpression {
        expressions: vec!(
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {datatype: Datatype::Number(6)})
            },
            Ast::Expression {
                operator: BinaryOperator::Plus,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 5)})
            }
         )
    };
    assert_eq!(Datatype::Number( 11), evaluate_ast(ast, &mut map).unwrap())
}



/// Assign the value 6 to "a".
/// Copy the value in "a" to "b".
/// Recall the value in "b" and add it to 5.
#[test]
fn variable_copy_test() {

    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::VecExpression {
        expressions: vec!(
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 6)})
            },
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier {ident: "b".to_string() }),
                expr2: Box::new(Ast::ValueIdentifier {ident: "a".to_string() })
            },
            Ast::Expression {
                operator: BinaryOperator::Plus,
                expr1: Box::new(Ast::ValueIdentifier {ident: "b".to_string() }),
                expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 5)})
            }
        )
    };
    assert_eq!(Datatype::Number(11), evaluate_ast(ast, &mut map).unwrap())
}

/// Assign the value 6 to a.
/// Assign the value 3 to a.
/// Recall the value in a and add it to 5, the value of a should be 3, equalling 8.
#[test]
fn reassignment_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::VecExpression {
        expressions: vec!(
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 6)})
            },
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {datatype: Datatype::Number( 3)} )
            },
            Ast::Expression {
                operator: BinaryOperator::Plus,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {datatype: Datatype::Number(5)})
            }
        )
    };
    assert_eq!(Datatype::Number(8), evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn conditional_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Conditional {
        condition: Box::new(Ast::Literal {datatype: Datatype::Bool(true)}),
        true_expr: Box::new(Ast::Literal {datatype: Datatype::Number(7)}),
        false_expr: None
    };
    assert_eq!(Datatype::Number(7), evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn conditional_with_else_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Conditional {
        condition: Box::new(Ast::Literal {datatype: Datatype::Bool(false)}),
        true_expr: Box::new(Ast::Literal {datatype: Datatype::Number(7)}),
        false_expr: Some(Box::new(Ast::Literal {datatype: Datatype::Number(2)}))
    };
    assert_eq!(Datatype::Number(2), evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn basic_function_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::VecExpression {
        expressions: vec!(
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {
                    datatype: Datatype::Function {
                        parameters: Box::new(Ast::VecExpression { expressions: vec!()}), // empty parameters
                        body: (Box::new(Ast::Literal {datatype: Datatype::Number(32)})), // just return a number
                        output_type: Box::new(Datatype::Number(0)), // expect a number
                    }
                })
            },
            Ast::Expression {
                operator: BinaryOperator::ExecuteFn,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }), // get the identifier for a
                expr2: Box::new(Ast::VecExpression { expressions: vec!()}) // provide the function parameters
            }
        )
    };
    assert_eq!(Datatype::Number(32), evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn function_with_parameter_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::VecExpression {
        expressions: vec!(
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {
                    datatype: Datatype::Function {
                        parameters: Box::new(Ast::VecExpression { expressions: vec!(
                            Ast::Expression {
                                operator: BinaryOperator::FunctionParameterAssignment,
                                expr1: Box::new(Ast::ValueIdentifier {ident: "b".to_string()}), // the value's name is b
                                expr2: Box::new(Ast::Literal {datatype: Datatype::Number(0)}) // fn takes a number
                            }
                        )}),
                        body: (Box::new(Ast::ValueIdentifier {ident: "b".to_string()} )), // just return the number passed in.
                        output_type: Box::new(Datatype::Number(0)), // expect a number to be returned
                    }
                })
            },
            Ast::Expression {
                operator: BinaryOperator::ExecuteFn,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }), // get the identifier for a
                expr2: Box::new(Ast::VecExpression { expressions: vec!(
                    Ast::Literal {datatype: Datatype::Number(7)}
                )}) // provide the function parameters
            }
        )
    };
    assert_eq!(Datatype::Number(7), evaluate_ast(ast, &mut map).unwrap())
}



#[test]
fn function_with_two_parameters_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::VecExpression {
        expressions: vec!(
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier {ident: "add_two_numbers".to_string() }),
                expr2: Box::new(Ast::Literal {
                    datatype: Datatype::Function {
                        parameters: Box::new(Ast::VecExpression { expressions: vec!(
                            Ast::Expression {
                                operator: BinaryOperator::FunctionParameterAssignment,
                                expr1: Box::new(Ast::ValueIdentifier {ident: "b".to_string()}), // the value's name is b
                                expr2: Box::new(Ast::Literal {datatype: Datatype::Number(0)}) // fn takes a number
                            },
                            Ast::Expression {
                                operator: BinaryOperator::FunctionParameterAssignment,
                                expr1: Box::new(Ast::ValueIdentifier {ident: "c".to_string()}), // the value's name is b
                                expr2: Box::new(Ast::Literal {datatype: Datatype::Number(0)}) // fn takes a number
                            }
                        )}),
                        body: (Box::new(Ast::Expression { // the body of the function will add the two passed in values together
                            operator: BinaryOperator::Plus,
                            expr1: Box::new(Ast::ValueIdentifier {ident: "b".to_string()} ),
                            expr2: Box::new(Ast::ValueIdentifier {ident: "c".to_string()} ),
                        })),

                        output_type: Box::new(Datatype::Number(0)), // expect a number to be returned
                    }
                })
            },
            Ast::Expression {
                operator: BinaryOperator::ExecuteFn,
                expr1: Box::new(Ast::ValueIdentifier {ident: "add_two_numbers".to_string() }), // get the identifier for a
                expr2: Box::new(Ast::VecExpression { expressions: vec!(
                    Ast::Literal {datatype: Datatype::Number(7)},
                    Ast::Literal {datatype: Datatype::Number(5)}
                )}) // provide the function parameters
            }
        )
    };
    assert_eq!(Datatype::Number(12), evaluate_ast(ast, &mut map).unwrap())
}