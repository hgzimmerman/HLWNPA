use nom::*;
use ast::{Ast, BinaryOperator};
use parser::identifier::identifier;
use parser::utilities::expression_or_literal_or_identifier;
use datatype::Datatype;

// TODO leave the let binding, possibly as a way to declare a const vs mutable structure
named!(pub assignment<Ast>,
    do_parse!(
        ws!(tag!("let")) >>
        id: ws!(identifier) >>
        ws!(tag!(":="))>>
        value: ws!(expression_or_literal_or_identifier) >>
        (Ast::Expression{ operator: BinaryOperator::Assignment, expr1: Box::new(id), expr2: Box::new(value) })
    )
);

#[test]
fn parse_assignment_of_literal_test() {
    let input_string = "let b := 8";
    let (_, value) = match assignment(input_string.as_bytes()) {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(
        Ast::Expression {
            operator: BinaryOperator::Assignment,
            expr1: Box::new(Ast::ValueIdentifier("b".to_string())),
            expr2: Box::new(Ast::Literal(Datatype::Number(8))),
        },
        value
    )
}
