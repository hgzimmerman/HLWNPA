#[allow(unused_imports)]
use nom::*;
use ast::Ast;
use datatype::Datatype;

named!(bool_false<bool>,
    value!(
        false,
        tag!("false")
    )
);
named!(bool_true<bool>,
    value!(
        true,
        tag!("true")
    )
);
named!(pub bool_literal<Ast>,
    do_parse!(
        boolean_value: alt!(bool_true | bool_false) >>
        (Ast::Literal (Datatype::Bool(boolean_value)))
    )
);

#[test]
fn parse_bool_literal_test() {
    let (_, value) = match bool_literal(b"true") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Literal ( Datatype::Bool(true)), value)
}
