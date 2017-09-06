#[allow(unused_imports)]
use nom::*;
use ast::Ast;
use parser::utilities::expression_or_literal_or_identifier_or_assignment;
use parser::assignment::type_assignment;

#[cfg(not(feature = "polite"))]
named!(pub body<Ast>,
    do_parse!(
        statements : delimited!(
            ws!(char!('{')),
            many0!(ws!(expression_or_literal_or_identifier_or_assignment)), // consider making a ; terminate an expression // Also, multiple ast types are valuable here. define a matcher for those. //todo: should be many1
            ws!(char!('}'))
        ) >>
        (Ast::VecExpression{expressions: statements})
    )
);

// easter egg
#[cfg(feature = "polite")]
named!(pub body<Ast>,
    do_parse!(
        statements : delimited!(
            ws!(tag!("please")),
            many0!(ws!(expression_or_literal_or_identifier_or_assignment)), // consider making a ; terminate an expression // Also, multiple ast types are valuable here. define a matcher for those. //todo: should be many1
            ws!(tag!("thankyou"))
        ) >>

        (Ast::VecExpression{expressions: statements})
    )
);


//Body that only accepts assignments in the form: a : 4
named!(pub type_assignment_body<Ast>,
    do_parse!(
        statements : delimited!(
            ws!(char!('{')),
            many0!(ws!(type_assignment)),
            ws!(char!('}'))
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
