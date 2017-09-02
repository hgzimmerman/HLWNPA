use lang_result::*;
use datatype::{Datatype, TypeInfo};
use std::mem::discriminant;

use std::boxed::Box;
use std::collections::HashMap;


// Consider allowing inner type safety by switching the Ast to this style:
//
//struct FooData { ... }
//struct BarData { ... }
//enum Baz {
//Foo(FooData),
//Bar(BarData)
//}
// Good candidates for a first pass are Type and Literal


#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Ast {
    Expression {
        operator: BinaryOperator,
        expr1: Box<Ast>,
        expr2: Box<Ast>,
    },
    UnaryExpression {
        operator: UnaryOperator,
        expr: Box<Ast>,
    },
    VecExpression { expressions: Vec<Ast> }, // uesd for structuring execution of the AST.
    Conditional {
        condition: Box<Ast>,
        true_expr: Box<Ast>,
        false_expr: Option<Box<Ast>>,
    },
    Literal { datatype: Datatype }, // consider making the Literal another enum with supported default datatypes.
    Type { datatype: TypeInfo }, // value in the datatype is useless, just use this to determine parameter type.
    ValueIdentifier { ident: String }, // gets the value mapped to a hashmap
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
    FunctionParameterAssignment,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum UnaryOperator {
    Print,
    Invert,
    Increment,
    Decrement,
}



impl Ast {
    pub fn evaluate(self, map: &mut HashMap<String, Datatype>) -> LangResult {
        match self {
            Ast::Expression {
                operator,
                expr1,
                expr2,
            } => {
                match operator {
                    BinaryOperator::Plus => expr1.evaluate(map)? + expr2.evaluate(map)?,
                    BinaryOperator::Minus => expr1.evaluate(map)? - expr2.evaluate(map)?,
                    BinaryOperator::Multiply => expr1.evaluate(map)? * expr2.evaluate(map)?,
                    BinaryOperator::Divide => expr1.evaluate(map)? / expr2.evaluate(map)?,
                    BinaryOperator::Modulo => expr1.evaluate(map)? % expr2.evaluate(map)?,
                    BinaryOperator::Equals => {
                        if expr1.evaluate(map)? == expr2.evaluate(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    BinaryOperator::GreaterThan => {
                        if expr1.evaluate(map)? >= expr2.evaluate(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    BinaryOperator::LessThan => {
                        if expr1.evaluate(map)? <= expr2.evaluate(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    BinaryOperator::Assignment => {
                        if let Ast::ValueIdentifier { ident } = *expr1 {
                            let mut cloned_map = map.clone(); // since this is a clone, the required righthand expressions will be evaluated in their own 'stack', this modified hashmap will be cleaned up post assignment.
                            let evaluated_right_hand_side = expr2.evaluate(&mut cloned_map)?;
                            let cloned_evaluated_rhs = evaluated_right_hand_side.clone();
                            map.insert(ident, evaluated_right_hand_side);
                            return Ok(cloned_evaluated_rhs);
                        } else {
                            Err(LangError::IdentifierDoesntExist)
                        }
                    }
                    BinaryOperator::FunctionParameterAssignment => {
                        // does the same thing as assignment, but I want a separate type for this.
                        if let Ast::ValueIdentifier { ident } = *expr1 {
                            let mut cloned_map = map.clone(); // since this is a clone, the required righthand expressions will be evaluated in their own 'stack', this modified hashmap will be cleaned up post assignment.
                            let evaluated_right_hand_side = expr2.evaluate(&mut cloned_map)?;
                            let cloned_evaluated_rhs = evaluated_right_hand_side.clone();
                            map.insert(ident, evaluated_right_hand_side);
                            return Ok(cloned_evaluated_rhs);
                        } else {
                            Err(LangError::IdentifierDoesntExist)
                        }
                    }
                    BinaryOperator::ExecuteFn => {
                        let mut cloned_map = map.clone(); // clone the map, to create a temporary new "stack" for the life of the function

                        // evaluate the parameters
                        let evaluated_parameters: Vec<Datatype> = match *expr2 {
                            Ast::VecExpression { expressions } => {
                                let mut evaluated_expressions: Vec<Datatype> = vec![];
                                for e in expressions {
                                    match e.evaluate(&mut cloned_map) {
                                        Ok(dt) => evaluated_expressions.push(dt),
                                        Err(err) => return Err(err),
                                    }
                                }
                                evaluated_expressions
                            }
                            _ => return Err(LangError::FunctionParametersShouldBeVecExpression),
                        };


                        // Take an existing function by (by grabbing the function using an identifier, which should resolve to a function)
                        match expr1.evaluate(&mut cloned_map)? {
                            Datatype::Function {
                                parameters,
                                body,
                                return_type,
                            } => {
                                match *parameters {
                                    // The parameters should be in the form: VecExpression(expression_with_fn_assignment, expression_with_fn_assignment, ...) This way, functions should be able to support arbitrary numbers of parameters.
                                    Ast::VecExpression { expressions } => {
                                        // zip the values of the evaluated parameters into the expected parameters for the function
                                        if evaluated_parameters.len() == expressions.len() {
                                            // Replace the right hand side of the expression (which should be an Ast::Type with a computed literal.
                                            let rhs_replaced_with_evaluated_parameters_results: Vec<Result<Ast, LangError>> = expressions
                                                .iter()
                                                .zip(evaluated_parameters)
                                                .map(|expressions_with_parameters: (&Ast, Datatype)| {
                                                    let (e, d) = expressions_with_parameters; // assign out of tuple.
                                                    if let Ast::Expression {
                                                        ref operator,
                                                        ref expr1,
                                                        ref expr2,
                                                    } = *e
                                                    {
                                                        let operator = operator.clone();
                                                        let expr1 = expr1.clone();

                                                        //do run-time type-checking, the supplied value should be of the same type as the specified value
                                                        let expected_type: &TypeInfo = match **expr2 {
                                                            Ast::Type { ref datatype } => datatype,
                                                            _ => return Err(LangError::ExpectedDataTypeInfo),
                                                        };
                                                        if expected_type != &TypeInfo::from(d.clone()) {
                                                            return Err(LangError::TypeError {
                                                                expected: expected_type.clone(),
                                                                found: TypeInfo::from(d),
                                                            });
                                                        }

                                                        if operator == BinaryOperator::FunctionParameterAssignment {
                                                            return Ok(Ast::Expression {
                                                                operator: operator,
                                                                expr1: expr1,
                                                                expr2: Box::new(Ast::Literal { datatype: d }),
                                                            }); // return a new FunctionParameterAssignment Expression with a replaced expr2.
                                                        } else {
                                                            return Err(LangError::InvalidFunctionPrototypeFormatting);
                                                        }
                                                    } else {
                                                        return Err(LangError::InvalidFunctionPrototypeFormatting);
                                                    }
                                                })
                                                .collect();

                                            // These functions _should_ all be assignments
                                            // So after replacing the Types with Literals that have been derived from the expressions passed in,
                                            // they can be associated with the identifiers, and the identifiers can be used in the function body later.
                                            for rhs in rhs_replaced_with_evaluated_parameters_results {
                                                let rhs = rhs?; // return the error if present
                                                rhs.evaluate(&mut cloned_map)?; // create the assignment
                                            }
                                        } else {
                                            return Err(LangError::ParameterLengthMismatch);
                                        }

                                        // Evaluate the body of the function
                                        let output: Datatype = body.evaluate(&mut cloned_map)?;
                                        if TypeInfo::from(output.clone()) == *return_type {
                                            return Ok(output);
                                        } else {
                                            return Err(LangError::ReturnTypeDoesNotMatchReturnValue);
                                        }
                                    }
                                    _ => return Err(LangError::ParserShouldHaveRejected), // The parser should have put the parameters in the form VecExpression(expression_with_assignment, expression_with_assignment, ...)
                                }
                            }
                            _ => Err(LangError::ExecuteNonFunction),
                        }
                    }
                }
            }
            Ast::UnaryExpression { operator, expr } => {
                match operator {
                    UnaryOperator::Print => {
                        print!("{:?}", expr.evaluate(map)?); // todo use: std::fmt::Display::fmt instead
                        Ok(Datatype::None)
                    }
                    UnaryOperator::Invert => {
                        match expr.evaluate(map)? {
                            Datatype::Bool(bool) => Ok(Datatype::Bool(!bool)),
                            _ => Err(LangError::InvertNonBoolean),
                        }
                    }
                    UnaryOperator::Increment => {
                        match expr.evaluate(map)? {
                            Datatype::Number(number) => Ok(Datatype::Number(number + 1)),
                            _ => Err(LangError::IncrementNonNumber),
                        }
                    }
                    UnaryOperator::Decrement => {
                        match expr.evaluate(map)? {
                            Datatype::Number(number) => Ok(Datatype::Number(number - 1)),
                            _ => Err(LangError::DecrementNonNumber),
                        }
                    }
                }
            }
            //Evaluate multiple expressions and return the result of the last one.
            Ast::VecExpression { expressions } => {
                let mut val: Datatype = Datatype::None;
                for e in expressions {
                    val = e.evaluate(map)?;
                }
                Ok(val) // return the last evaluated expression;
            }
            Ast::Conditional {
                condition,
                true_expr,
                false_expr,
            } => {
                match condition.evaluate(map)? {
                    Datatype::Bool(bool) => {
                        match bool {
                            true => Ok(true_expr.evaluate(map)?),
                            false => {
                                match false_expr {
                                    Some(e) => Ok(e.evaluate(map)?),
                                    _ => Ok(Datatype::None),
                                }
                            }
                        }
                    }
                    _ => Err(LangError::ConditionOnNonBoolean),
                }
            }
            Ast::Literal { datatype } => Ok(datatype),
            Ast::Type { datatype } => Ok(Datatype::None), // you shouldn't try to evaluate the datatype, // todo consider making this an error
            Ast::ValueIdentifier { ident } => {
                match map.get(&ident) {
                    Some(value) => Ok(value.clone()),
                    None => Err(LangError::VariableDoesntExist( format!("Variable `{}` hasn't been assigned yet", ident)) ), // identifier hasn't been assigned yet.
                }
            }
        }
    }
}




#[test]
fn plus_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Plus,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(6) }),
    };
    assert_eq!(Datatype::Number(9), ast.evaluate(&mut map).unwrap())
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
        ast.evaluate(&mut map).unwrap()
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
    assert_eq!(Datatype::Number(3), ast.evaluate(&mut map).unwrap())
}

#[test]
fn minus_negative_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Minus,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(6) }),
    };
    assert_eq!(Datatype::Number(-3), ast.evaluate(&mut map).unwrap())
}

#[test]
fn multiplication_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Multiply,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(6) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
    };
    assert_eq!(Datatype::Number(18), ast.evaluate(&mut map).unwrap())
}

#[test]
fn division_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Divide,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(6) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
    };
    assert_eq!(Datatype::Number(2), ast.evaluate(&mut map).unwrap())
}

#[test]
fn integer_division_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Divide,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(5) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
    };
    assert_eq!(Datatype::Number(1), ast.evaluate(&mut map).unwrap())
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
        ast.evaluate(&mut map).err().unwrap()
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
    assert_eq!(Datatype::Number(2), ast.evaluate(&mut map).unwrap())
}

#[test]
fn equality_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::Equals,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
    };
    assert_eq!(Datatype::Bool(true), ast.evaluate(&mut map).unwrap())
}

#[test]
fn greater_than_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::GreaterThan,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(4) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
    };
    assert_eq!(Datatype::Bool(true), ast.evaluate(&mut map).unwrap())
}

#[test]
fn less_than_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Expression {
        operator: BinaryOperator::LessThan,
        expr1: Box::new(Ast::Literal { datatype: Datatype::Number(2) }),
        expr2: Box::new(Ast::Literal { datatype: Datatype::Number(3) }),
    };
    assert_eq!(Datatype::Bool(true), ast.evaluate(&mut map).unwrap())
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
    assert_eq!(Datatype::Number(11), ast.evaluate(&mut map).unwrap())
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
    assert_eq!(Datatype::Number(11), ast.evaluate(&mut map).unwrap())
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
    assert_eq!(Datatype::Number(8), ast.evaluate(&mut map).unwrap())
}

#[test]
fn conditional_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Conditional {
        condition: Box::new(Ast::Literal { datatype: Datatype::Bool(true) }),
        true_expr: Box::new(Ast::Literal { datatype: Datatype::Number(7) }),
        false_expr: None,
    };
    assert_eq!(Datatype::Number(7), ast.evaluate(&mut map).unwrap())
}

#[test]
fn conditional_with_else_test() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    let ast = Ast::Conditional {
        condition: Box::new(Ast::Literal { datatype: Datatype::Bool(false) }),
        true_expr: Box::new(Ast::Literal { datatype: Datatype::Number(7) }),
        false_expr: Some(Box::new(Ast::Literal { datatype: Datatype::Number(2) })),
    };
    assert_eq!(Datatype::Number(2), ast.evaluate(&mut map).unwrap())
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
    assert_eq!(Datatype::Number(32), ast.evaluate(&mut map).unwrap())
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
                                    expr1: Box::new(Ast::ValueIdentifier { ident: "b".to_string() }), // the value's name is b
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
    assert_eq!(Datatype::Number(7), ast.evaluate(&mut map).unwrap())
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
                                    expr1: Box::new(Ast::ValueIdentifier { ident: "b".to_string() }), // the value's name is b
                                    expr2: Box::new(Ast::Type { datatype: TypeInfo::Number }), // fn takes a number
                                },
                                Ast::Expression {
                                    operator: BinaryOperator::FunctionParameterAssignment,
                                    expr1: Box::new(Ast::ValueIdentifier { ident: "c".to_string() }), // the value's name is b
                                    expr2: Box::new(Ast::Type { datatype: TypeInfo::Number }), // fn takes a number
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
    assert_eq!(Datatype::Number(12), ast.evaluate(&mut map).unwrap())
}
