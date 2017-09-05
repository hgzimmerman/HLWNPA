use ast::Ast;
#[allow(unused_imports)]
use nom::*;
use datatype::Datatype;
use std::str::FromStr;
use std::str;

named!(number_raw<i32>,
    do_parse!(
        number: map_res!(
            map_res!(
                recognize!(
                    digit
                ),
                str::from_utf8
            ),
            FromStr::from_str
        ) >>
        (number)
    )
);
named!(pub number_literal<Ast>,
    do_parse!(
       num: ws!(number_raw) >>
        (Ast::Literal ( Datatype::Number(num)))
    )
);

#[test]
fn parse_number_test() {
    let (_, value) = match number_raw(b"42") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(42, value)
}

#[test]
fn parse_number_literal_test() {
    let (_, value) = match number_literal(b"42") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Literal ( Datatype::Number(42)), value)
}
