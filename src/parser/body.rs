use nom::*;
use ast::Ast;
use parser::utilities::expression_or_literal_or_identifier_or_assignment;

named!(pub body<Ast>,
    do_parse!(
        statements : delimited!(
            ws!(char!('{')),
            many0!(ws!(expression_or_literal_or_identifier_or_assignment)), // consider making a ; terminate an expression // Also, multiple ast types are valuable here. define a matcher for those. //todo: should be many1
            ws!(tag!("}"))
        ) >>
        (Ast::VecExpression{expressions: statements})
    )
);

#[test]
fn parse_body_nocheck_test() {
    let input_string = "{ ( a + 8 ) }";
    let (_, _) = match body(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };
}

#[test]
fn parse_simple_body_test() {
    let input_string = "{ true }";
    let (_, _) = match body(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("Error in parsing: {}", e),
        IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
    };
}


#[test]
fn parse_simple_body_assignment_test() {
    let input_string = "{ let a := 8 }";
    let (_, _) = match body(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("Error in parsing: {}", e),
        IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
    };
}
