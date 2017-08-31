use lang_result::*;
use datatype::Datatype;
use std::mem::discriminant;

use std::boxed::Box;
use std::collections::HashMap;

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
    pub fn evaluate_ast(self, map: &mut HashMap<String, Datatype>) -> LangResult {
        match self {
            Ast::Expression {
                operator,
                expr1,
                expr2,
            } => {
                match operator {
                    BinaryOperator::Plus => expr1.evaluate_ast(map)? + expr2.evaluate_ast(map)?,
                    BinaryOperator::Minus => expr1.evaluate_ast(map)? - expr2.evaluate_ast(map)?,
                    BinaryOperator::Multiply => expr1.evaluate_ast(map)? * expr2.evaluate_ast(map)?,
                    BinaryOperator::Divide => expr1.evaluate_ast(map)? / expr2.evaluate_ast(map)?,
                    BinaryOperator::Modulo => expr1.evaluate_ast(map)? % expr2.evaluate_ast(map)?,
                    BinaryOperator::Equals => {
                        if expr1.evaluate_ast(map)? == expr2.evaluate_ast(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    BinaryOperator::GreaterThan => {
                        if expr1.evaluate_ast(map)? >= expr2.evaluate_ast(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    BinaryOperator::LessThan => {
                        if expr1.evaluate_ast(map)? <= expr2.evaluate_ast(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    BinaryOperator::Assignment => {
                        if let Ast::ValueIdentifier { ident } = *expr1 {
                            let mut cloned_map = map.clone(); // since this is a clone, the required righthand expressions will be evaluated in their own 'stack', this modified hashmap will be cleaned up post assignment.
                            let evaluated_right_hand_side = expr2.evaluate_ast(&mut cloned_map)?;
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
                            let evaluated_right_hand_side = expr2.evaluate_ast(&mut cloned_map)?;
                            let cloned_evaluated_rhs = evaluated_right_hand_side.clone();
                            map.insert(ident, evaluated_right_hand_side);
                            return Ok(cloned_evaluated_rhs);
                        } else {
                            Err(LangError::IdentifierDoesntExist)
                        }
                    }
                    BinaryOperator::ExecuteFn => {
                        // evaluate the parameters

                        let mut cloned_map = map.clone(); // clone the map, to create a temporary new "stack" for the life of the function

                        let evaluated_parameters: Vec<Datatype> = match *expr2 {
                            Ast::VecExpression { expressions } => {
                                let mut evaluated_expressions: Vec<Datatype> = vec!();
                                for e in expressions {
                                    match e.evaluate_ast(&mut cloned_map) {
                                        Ok(dt) => evaluated_expressions.push(dt),
                                        Err(err) => return Err(err),
                                    }
                                }
                                evaluated_expressions
                            }
                            _ => return Err(LangError::FunctionParametersShouldBeVecExpression),
                        };


                        match expr1.evaluate_ast(&mut cloned_map)? {
                            Datatype::Function {
                                parameters,
                                body,
                                output_type,
                            } => {
                                match *parameters {
                                    // The parameters should be in the form VecExpression(expression_with_fn_assignment, expression_with_fn_assignment, ...) This way, functions should be able to support arbitrary numbers of parameters.
                                    Ast::VecExpression { expressions } => {
                                        // zip the values of the evaluated parameters into the expected parameters for the function
                                        if evaluated_parameters.len() == expressions.len() {
                                            // create an ast::VecExpression that
                                            let rhs_replaced_with_evaluated_parameters_results: Vec<Result<Ast, LangError>> = expressions.iter().zip(evaluated_parameters).map(|expressions_with_parameters: (&Ast, Datatype)| {
                                                let (e, d) = expressions_with_parameters; // assign out of tuple.
                                                if let Ast::Expression { ref operator, ref expr1, .. } = *e {
                                                    let operator = operator.clone();
                                                    let expr1 = expr1.clone();

                                                    if operator == BinaryOperator::FunctionParameterAssignment {
                                                        return Ok(Ast::Expression { operator: operator, expr1: expr1, expr2: Box::new(Ast::Literal { datatype: d }) }); // return a new FunctionParameterAssignment Expression with a replaced expr2.
                                                    } else {
                                                        return Err(LangError::InvalidFunctionPrototypeFormatting)
                                                    }
                                                } else {
                                                    return Err(LangError::InvalidFunctionPrototypeFormatting)
                                                }
                                            }
                                            ).collect();


                                            for rhs in
                                                rhs_replaced_with_evaluated_parameters_results
                                            {
                                                let rhs = rhs?; // return the error if present
                                                rhs.evaluate_ast(&mut cloned_map)?; // create the assignment
                                            }
                                        } else {
                                            return Err(LangError::ParameterLengthMismatch);
                                        }

                                        // Evaluate the body of the
                                        let output = body.evaluate_ast(&mut cloned_map)?;
                                        if discriminant(&output) == discriminant(&output_type) {
                                            return Ok(output);
                                        } else {
                                            return Err(
                                                LangError::ReturnTypeDoesNotMatchReturnValue,
                                            );
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
                        print!("{:?}", expr.evaluate_ast(map)?); // todo use: std::fmt::Display::fmt instead
                        Ok(Datatype::None)
                    }
                    UnaryOperator::Invert => {
                        match expr.evaluate_ast(map)? {
                            Datatype::Bool(bool) => Ok(Datatype::Bool(!bool)),
                            _ => Err(LangError::InvertNonBoolean),
                        }
                    }
                    UnaryOperator::Increment => {
                        match expr.evaluate_ast(map)? {
                            Datatype::Number(number) => Ok(Datatype::Number(number + 1)),
                            _ => Err(LangError::IncrementNonNumber),
                        }
                    }
                    UnaryOperator::Decrement => {
                        match expr.evaluate_ast(map)? {
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
                    val = e.evaluate_ast(map)?;
                }
                Ok(val) // return the last evaluated expression;
            }
            Ast::Conditional {
                condition,
                true_expr,
                false_expr,
            } => {
                match condition.evaluate_ast(map)? {
                    Datatype::Bool(bool) => {
                        match bool {
                            true => Ok(true_expr.evaluate_ast(map)?),
                            false => {
                                match false_expr {
                                    Some(e) => Ok(e.evaluate_ast(map)?),
                                    _ => Ok(Datatype::None),
                                }
                            }
                        }
                    }
                    _ => Err(LangError::ConditionOnNonBoolean),
                }
            }
            Ast::Literal { datatype } => Ok(datatype),
            Ast::ValueIdentifier { ident } => {
                match map.get(&ident) {
                    Some(value) => Ok(value.clone()),
                    None => panic!("Variable {} hasn't been assigned yet", ident), // identifier hasn't been assigned yet.
                }
            }
        }
    }
}
