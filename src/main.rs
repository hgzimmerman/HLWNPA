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

#[derive(PartialEq, Debug)]
enum Ast {
    Expression { operator: BinaryOperator, expr1: Box<Ast>, expr2: Box<Ast>  },
    UnaryExpression{ operator: UnaryOperator, expr: Box<Ast>},
    VecExpression { expressions: Vec<Ast>}, // uesd for structuring execution of the AST.
    Literal { datatype: Datatype }, // consider making the Literal another enum with supported default datatypes.
    ValueIdentifier { ident: String}, // gets the value mapped to a hashmap
}


#[derive(PartialEq, Debug)]
enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Assignment,
}

#[derive(PartialEq, Debug)]
enum UnaryOperator {
    Print,
    Invert,
    Increment,
    Decrement
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
                }
                BinaryOperator::Multiply => {
                    evaluate_ast(*expr1, map)? * evaluate_ast(*expr2, map)?
                }
                BinaryOperator::Divide => {
                    evaluate_ast(*expr1, map)? / evaluate_ast(*expr2, map)?
                }
                BinaryOperator::Modulo => {
                    evaluate_ast(*expr1, map)? % evaluate_ast(*expr2, map)?
                }
                BinaryOperator::Assignment => {
                    if let Ast::ValueIdentifier{ident} = *expr1 {
                        let mut cloned_map = map.clone(); // since this is a clone, the required righthand expressions will be evaluated in their own 'stack', this modified hashmap will be cleaned up post assignment.
                        let evaluated_right_hand_side = evaluate_ast(*expr2, &mut cloned_map)?;
                        let cloned_evaluated_rhs = evaluated_right_hand_side.clone();
                        map.insert(ident, evaluated_right_hand_side);
                        return Ok(cloned_evaluated_rhs);
                    }
                    else { Err(LangError::IdentifierDoesntExist) }
                }
            }
        },
        Ast::UnaryExpression {operator, expr} => {
          match operator {
              UnaryOperator::Print => {
                  print!("{:?}", evaluate_ast(*expr, map)?); // todo use: std::fmt::Display::fmt instead
                  Ok(Datatype::None)
              }
              _ => Err(LangError::UnsupportedArithimaticOperation)
          }
        },
        Ast::VecExpression {expressions} => {
            let mut val: Datatype = Datatype::None;
            for e in expressions {
                val = evaluate_ast(e, map)?;
            };
            Ok(val) // return the last evaluated expression;
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



/// Assign the value 6 to the identifier "a"
/// Recall that identifier and add it to 5
#[test]
fn assignment_test(){
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