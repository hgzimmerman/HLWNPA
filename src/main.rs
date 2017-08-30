use std::boxed::Box;
use std::collections::HashMap;

fn main() {

    let mut identifier_map: HashMap<String, i32> = HashMap::new();

//    let ast = Ast::Expression {
//        operator: Operator::Plus,
//        expr1: Box::new(Ast::Literal {number:74}),
//        expr2: Box::new(Ast::Literal {number:5})
//    };

    let ast = Ast::Expression {
        operator: Operator::Print,
        expr1: Box::new(Ast::VecExpression {
            expressions: vec!(
                Ast::Expression {
                    operator: Operator::Assignment,
                    expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                    expr2: Box::new(Ast::Literal {number:6})
                },
                Ast::Expression {
                    operator: Operator::Plus,
                    expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                    expr2: Box::new(Ast::Literal {number:5})
                }
             )
        }),
        expr2: Box::new(Ast::None)
    };


    let _ = evaluate_ast(ast, &mut identifier_map);
}

#[derive(PartialEq, Debug)]
enum Ast {
    Expression { operator: Operator, expr1: Box<Ast>, expr2: Box<Ast>  },
    VecExpression { expressions: Vec<Ast>}, // uesd for structuring execution of the AST.
    Literal { number: i32 }, // consider making the Literal another enum with supported default datatypes.
    None, // used for representing a no-op node.
    ValueIdentifier { ident: String}, // gets the value mapped to a hashmap
}

#[derive(PartialEq, Debug)]
enum Datatype {
    Number { value: i32 },
    String { value: String},
    Array {value: Vec<Datatype>, type_: Box<Datatype>}, // I'm sort of losing type safety here.
    Bool { value: bool},
    //Object { value: Vec<Datatype>, v_table: Vec<Ast>}
}

#[derive(PartialEq, Debug)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Assignment,
    Print

}

fn evaluate_ast(ast: Ast, map: &mut HashMap<String, i32>) -> LangResult {
    match ast {
        Ast::Expression {operator, expr1, expr2 } => {
            match operator {
                Operator::Plus => {
                    Ok(evaluate_ast(*expr1, map)? + evaluate_ast(*expr2, map)?)
                },
                Operator::Minus => {
                    Ok(evaluate_ast(*expr1, map)? - evaluate_ast(*expr2, map)?)
                }
                Operator::Multiply => {
                    Ok(evaluate_ast(*expr1, map)? * evaluate_ast(*expr2, map)?)
                }
                Operator::Divide => {
                    let evaluated_right_hand_side = evaluate_ast(*expr2, map)?;
                    if evaluated_right_hand_side == 0 {
                        return Err(LangError::DivideByZero)
                    }
                    Ok(evaluate_ast(*expr1, map)? / evaluated_right_hand_side)
                }
                Operator::Modulo => {
                    Ok(evaluate_ast(*expr1, map)? % evaluate_ast(*expr2, map)?)
                }
                Operator::Assignment => {
                    if let Ast::ValueIdentifier{ident} = *expr1 {
                        let mut cloned_map = map.clone(); // since this is a clone, the required righthand expressions will be evaluated in their own 'stack', this modified hashmap will be cleaned up post assignment.
                        let evaluated_right_hand_side = evaluate_ast(*expr2, &mut cloned_map)?;
                        map.insert(ident, evaluated_right_hand_side);
                        return Ok(evaluated_right_hand_side);
                    }
                    else { Err(LangError::IdentifierDoesntExist) }
                }
                Operator::Print => {
                    if *expr2 != Ast::None {
                        return Err(LangError::ParserShouldHaveRejected)
                    }
                    print!("{}", evaluate_ast(*expr1, map)?);
                    Ok(0)
                }
            }
        },
        Ast::VecExpression {expressions} => {
            let mut val: i32 = 0;
            for e in expressions {
                val = evaluate_ast(e, map)?;
            };
            Ok(val) // return the last evaluated expression;
        },
        Ast::Literal {number} => { Ok(number) },
        Ast::ValueIdentifier {ident} => {
            match map.get(&ident) {
                Some(value) => Ok(*value),
                None => panic!("Variable {} hasn't been assigned yet", ident) // identifier hasn't been assigned yet.
            }
        },
        Ast::None => {
            Err(LangError::InvalidEvaluationOfNone)
        }

    }
}


type LangResult = Result<i32, LangError>;

#[derive(PartialEq, Debug)]
enum LangError {
    DivideByZero,
    InvalidEvaluationOfNone, // should never happen
    IdentifierDoesntExist,
    ParserShouldHaveRejected // should never happen
}






#[test]
fn plus_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    let ast = Ast::Expression {
                operator: Operator::Plus,
                expr1: Box::new(Ast::Literal {number:3}),
                expr2: Box::new(Ast::Literal {number:6})
            };
    assert_eq!(9, evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn minus_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    let ast = Ast::Expression {
        operator: Operator::Minus,
        expr1: Box::new(Ast::Literal {number:6}),
        expr2: Box::new(Ast::Literal {number:3})
    };
    assert_eq!(3, evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn minus_negative_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    let ast = Ast::Expression {
        operator: Operator::Minus,
        expr1: Box::new(Ast::Literal {number:3}),
        expr2: Box::new(Ast::Literal {number:6})
    };
    assert_eq!(-3, evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn multiplication_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    let ast = Ast::Expression {
        operator: Operator::Multiply,
        expr1: Box::new(Ast::Literal {number:6}),
        expr2: Box::new(Ast::Literal {number:3})
    };
    assert_eq!(18, evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn division_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    let ast = Ast::Expression {
        operator: Operator::Divide,
        expr1: Box::new(Ast::Literal {number:6}),
        expr2: Box::new(Ast::Literal {number:3})
    };
    assert_eq!(2, evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn integer_division_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    let ast = Ast::Expression {
        operator: Operator::Divide,
        expr1: Box::new(Ast::Literal {number:5}),
        expr2: Box::new(Ast::Literal {number:3})
    };
    assert_eq!(1, evaluate_ast(ast, &mut map).unwrap())
}

#[test]
fn division_by_zero_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    let ast = Ast::Expression {
        operator: Operator::Divide,
        expr1: Box::new(Ast::Literal {number:5}),
        expr2: Box::new(Ast::Literal {number:0})
    };
    assert_eq!(LangError::DivideByZero, evaluate_ast(ast, &mut map).err().unwrap())
}

#[test]
fn modulo_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    let ast = Ast::Expression {
        operator: Operator::Modulo,
        expr1: Box::new(Ast::Literal {number:8}),
        expr2: Box::new(Ast::Literal {number:3})
    };
    assert_eq!(2, evaluate_ast(ast, &mut map).unwrap())
}



/// Assign the value 6 to the identifier "a"
/// Recall that identifier and add it to 5
#[test]
fn assignment_test(){
    let mut map: HashMap<String, i32> = HashMap::new();
    let ast = Ast::VecExpression {
        expressions: vec!(
            Ast::Expression {
                operator: Operator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {number:6})
            },
            Ast::Expression {
                operator: Operator::Plus,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {number:5})
            }
         )
    };
    assert_eq!(11, evaluate_ast(ast, &mut map).unwrap())
}



/// Assign the value 6 to "a".
/// Copy the value in "a" to "b".
/// Recall the value in "b" and add it to 5.
#[test]
fn variable_copy_test() {

    let mut map: HashMap<String, i32> = HashMap::new();
    let ast = Ast::VecExpression {
        expressions: vec!(
            Ast::Expression {
                operator: Operator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {number:6})
            },
            Ast::Expression {
                operator: Operator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier {ident: "b".to_string() }),
                expr2: Box::new(Ast::ValueIdentifier {ident: "a".to_string() })
            },
            Ast::Expression {
                operator: Operator::Plus,
                expr1: Box::new(Ast::ValueIdentifier {ident: "b".to_string() }),
                expr2: Box::new(Ast::Literal {number:5})
            }
        )
    };
    assert_eq!(11, evaluate_ast(ast, &mut map).unwrap())
}

/// Assign the value 6 to a.
/// Assign the value 3 to a.
/// Recall the value in a and add it to 5, the value of a should be 3, equalling 8.
#[test]
fn reassignment_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    let ast = Ast::VecExpression {
        expressions: vec!(
            Ast::Expression {
                operator: Operator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {number:6})
            },
            Ast::Expression {
                operator: Operator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {number:3} )
            },
            Ast::Expression {
                operator: Operator::Plus,
                expr1: Box::new(Ast::ValueIdentifier {ident: "a".to_string() }),
                expr2: Box::new(Ast::Literal {number:5})
            }
        )
    };
    assert_eq!(8, evaluate_ast(ast, &mut map).unwrap())
}