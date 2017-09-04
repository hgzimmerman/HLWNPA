use std::str::FromStr;
use std::str;
use nom::*;
use ast::Ast;
use datatype::Datatype;

named!(string<String>,
    do_parse!(
       str: map_res!(
            delimited!(
                tag!("\""),
                take_until!("\""),
                tag!("\"")
            ),
            str::from_utf8
        ) >>
        (str.to_string())
    )
);

named!(pub string_literal<Ast>,
    do_parse!(
        str: ws!(string) >>
        (Ast::Literal (Datatype::String(str)))
    )
);

#[test]
fn parse_string_test() {
    let input_string = "\"Hello World\"";
    let (_, value) = match string(input_string.as_bytes()) {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!("Hello World".to_string(), value)
}

#[test]
fn parse_string_literal_test() {
    let input_string = " \"Hello World\"  ";
    let (_, value) = match string_literal(input_string.as_bytes()) {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Literal ( Datatype::String("Hello World".to_string())), value)
}
