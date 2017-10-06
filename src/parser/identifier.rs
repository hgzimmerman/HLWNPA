use ast::Ast;
#[allow(unused_imports)]
use nom::*;
use std::str;


named!(normal_reserved_words,
    alt!(
        tag!("let") | tag!("const") | tag!("fn") | tag!("if") | tag!("else") |
        tag!("for") |
        tag!("while") | tag!("true") | tag!("false") |
        tag!("struct") |
        tag!("new") | tag!("include")
    )
);
#[cfg(not(feature = "polite"))]
named!( reserved_words,
    alt!(
        normal_reserved_words
    )
);

#[cfg(feature = "polite")]
named!( reserved_words,
    alt!(
        normal_reserved_words |
        tag!("please") |
        tag!("thankyou")
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
        // fail the identifier parser if it starts with a reserved word and then a whitespace
        not!(pair!(reserved_words, multispace))>> // TODO consider making the next character a not!(alphanumeric)
        id: ws!(
            accepted_identifier_characters
        ) >>
        (Ast::ValueIdentifier ( id.to_string()))
    )
);

#[cfg(test)]
mod test {
    use super::*;

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
        assert_eq!(Ast::ValueIdentifier("variableName".to_string()), value)
    }

    #[test]
    fn parse_identifier_underscore_test() {
        let (_, value) = match identifier(b"variable_name ") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(Ast::ValueIdentifier("variable_name".to_string()), value)
    }

    #[test]
    fn parse_new_id() {
        let (_, value) = match identifier(b"a") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(Ast::ValueIdentifier("a".to_string()), value);
    }

    #[test]
    fn parse_fail_new_id_reserved_character() {
        let _ = match identifier(b"+") {
            IResult::Done(_, v) => panic!("parse succeeded with value: {:?}", v),
            IResult::Error(e) => e,
            _ => panic!(),
        };
    }

    #[test]
    fn parse_fail_new_id_contains_reserved_character() {
        let value = match identifier(b"hello+world") {
            IResult::Done(_, v) => v,
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };
        assert_eq!(Ast::ValueIdentifier("hello".to_string()), value)
    }

    #[test]
    fn parse_fail_new_id_reserved_word() {
        let _ = match identifier(b"struct ") {
            IResult::Done(_, v) => panic!("parse succeeded with value: {:?}", v),
            IResult::Error(e) => e,
            _ => panic!(),
        };
    }

    #[test]
    fn parse_succeed_new_id_reserved_word() {
        let value = match identifier(b"struct_thing") {
            IResult::Done(_, v) => v,
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Ast::ValueIdentifier("struct_thing".to_string()), value)
    }
}
