
use datatype::{Datatype, TypeInfo};
use std::collections::HashMap;
use ast::{Ast, UnaryOperator, BinaryOperator};

pub fn add_std_functions(map: &mut HashMap<String, Datatype>) {
    add_print_function(map)
}

fn add_print_function(map: &mut HashMap<String, Datatype>) {
    let ast: Ast = Ast::Expression {
        operator: BinaryOperator::Assignment,
        expr1: Box::new(Ast::ValueIdentifier("print".to_string())),
        expr2: Box::new(Ast::Literal(Datatype::Function {
            parameters: Box::new(Ast::VecExpression {
                expressions: vec![Ast::Expression {
                    operator: BinaryOperator::TypeAssignment,
                    expr1: Box::new(Ast::ValueIdentifier("to_print".to_string())),
                    expr2: Box::new(Ast::ValueIdentifier("Identifier".to_string()))
                }],
            }),
            body: Box::new(Ast::VecExpression {
                expressions: vec![
                    Ast::UnaryExpression {
                        operator: UnaryOperator::Print,
                        expr: Box::new(Ast::ValueIdentifier("to_print".to_string()))
                    }],
            }),
            return_type: Box::new(Ast::Type(TypeInfo::None)),
        })),

    };
    ast.evaluate(map);
}


#[test]
fn expect_print_function_to_be_added_to_global_map() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    add_std_functions(&mut map);
    let mut expected_map: HashMap<String, Datatype> = HashMap::new();
    assert_eq!(expected_map, map);
}