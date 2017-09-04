
use ast::{Ast };
use nom::*;
use std::str;

named!(reserved_words,
    alt!(
        ws!(tag!("let")) |
        ws!(tag!("fn")) |
        ws!(tag!("if")) |
        ws!(tag!("else")) |
        ws!(tag!("while")) |
        ws!(tag!("true")) |
        ws!(tag!("false"))
    )
);

named!(accepted_identifier_characters<&str>,
    map_res!(
        is_not!(" \n\t\r.(){}<>[],:;+-*/%!=\""),
        str::from_utf8
    )
);

named!(pub identifier<Ast>,
    do_parse!(
        not!(reserved_words)>>
        id: ws!(
            accepted_identifier_characters
        ) >>
        (Ast::ValueIdentifier ( id.to_string()))
    )
);

#[test]
fn parse_identifier_characters_test() {
    let input_string = "name ";
    let (_, _) = match accepted_identifier_characters(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };
}

#[test]
fn parse_identifier_alphanumeric_test() {
    let (_, value) = match identifier(b"variableName") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::ValueIdentifier ( "variableName".to_string()), value)
}

#[test]
fn parse_identifier_underscore_test() {
    let (_, value) = match identifier(b"variable_name ") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::ValueIdentifier ( "variable_name".to_string()), value)
}