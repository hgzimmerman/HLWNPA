use nom::*;
use ast::{Ast, BinaryOperator};
use parser::body::body;
use parser::utilities::expression_or_literal_or_identifier;
use std::boxed::Box;
use datatype::Datatype;


named!(pub while_loop<Ast>,
    do_parse!(
        ws!(tag!("while")) >>
        while_conditional: ws!(expression_or_literal_or_identifier) >>
        while_body: ws!(body) >>

        (Ast::Expression {
            operator: BinaryOperator::Loop,
            expr1: Box::new(while_conditional),
            expr2: Box::new(while_body)
        })
    )
);


#[test]
fn parse_while_loop_test() {
    let input_string = "while true { true }";
    let (_, value) = match while_loop(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("Error in parsing: {}", e),
        IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
    };

    assert_eq!(
        Ast::Expression {
            operator: BinaryOperator::Loop,
            expr1: Box::new(Ast::Literal(Datatype::Bool(true))),
            expr2: Box::new(Ast::VecExpression {
                expressions: vec![Ast::Literal(Datatype::Bool(true))],
            }),
        },
        value
    )
}
