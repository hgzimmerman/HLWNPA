use ast::Ast;
#[allow(unused_imports)]
use nom::*;
use std::str;


#[cfg(not(feature = "polite"))]
named!(
    reserved_words,
    alt!(
        tag!("let") |tag!("fn") | tag!("if") | tag!("else") |
            tag!("while") | tag!("true") | tag!("false") |
            tag!("struct") |
            tag!("new") | tag!("include")
    )
);

#[cfg(feature = "polite")]
named!(
    reserved_words,
    alt!(
        ws!(tag!("let")) | ws!(tag!("fn")) | ws!(tag!("if")) | ws!(tag!("else")) |
            ws!(tag!("while")) | ws!(tag!("true")) | ws!(tag!("false")) |
            ws!(tag!("struct")) | ws!(tag!("new")) |
            ws!(tag!("include")) | ws!(tag!("please")) |
            ws!(tag!("thankyou"))
    )
);

named!(accepted_identifier_characters<&str>,
    map_res!(
        is_not!(" \n\t\r.(){}<>[],:;+-*/%!=\"&|"),
        str::from_utf8
    )
);

named!(pub identifier<Ast>,
    do_parse!(
        not!(pair!(reserved_words, tag!(" ")))>>
        id: ws!(
            accepted_identifier_characters
        ) >>
        (Ast::ValueIdentifier ( id.to_string()))
    )
);

named!( pub new_identifier<Ast>,
    do_parse!(
        id_vec: many1!(
            none_of!(" \n\t\r.(){}<>[],:;+-*/%!=\"&|")
        ) >>
        (Ast::ValueIdentifier(id_vec.into_iter().collect::<String>()))
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

//#[test]
//fn parse_identifier_with_leading_reserved_word() {
//    let (_, value) = match identifier(b"a_struct_thing") {
//        IResult::Done(r, v) => (r, v),
//        IResult::Error(e) => panic!("{:?}", e),
//        _ => panic!(),
//    };
//    assert_eq!(Ast::ValueIdentifier ( "a_struct_thing".to_string()), value);
//
//
//    let (_, value) = match identifier(b"struct_thing") {
//        IResult::Done(r, v) => (r, v),
//        IResult::Error(e) => panic!("{:?}", e),
//        _ => panic!(),
//    };
//    assert_eq!(Ast::ValueIdentifier ( "struct_thing".to_string()), value)
//}

#[test]
fn parse_new_id() {

    let (_, value) = match identifier(b"a") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::ValueIdentifier ( "a".to_string()), value);
}

#[test]
fn parse_fail_new_id_reserved_character() {

    let e = match identifier(b"+") {
        IResult::Done(_, v) => panic!("parse succeeded with value: {:?}", v),
        IResult::Error(e) => e,
        _ => panic!(),
    };

}

#[test]
fn parse_fail_new_id_contains_reserved_character() {

    let value = match identifier(b"hello+world") {
        IResult::Done(_, v) => v,
        IResult::Error(e) =>  panic!("{}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::ValueIdentifier ( "hello".to_string()), value)
}

#[test]
fn parse_fail_new_id_reserved_word() {

    let e = match identifier(b"struct ") {
        IResult::Done(_, v) => panic!("parse succeeded with value: {:?}", v),
        IResult::Error(e) => e,
        _ => panic!(),
    };
}

#[test]
fn parse_succeed_new_id_reserved_word() {

    let value = match identifier(b"struct_thing") {
        IResult::Done(_, v) =>  v,
            IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };

    assert_eq!(Ast::ValueIdentifier ( "struct_thing".to_string()), value)
}
