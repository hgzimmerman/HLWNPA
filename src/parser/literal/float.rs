use ast::Ast;
#[allow(unused_imports)]
use nom::*;
use datatype::Datatype;
use std::str::FromStr;
use std::str;


named!(float_raw<f64>,
    do_parse!(
        float_string: float_structure >>
        (f64::from_str(float_string.as_str()).unwrap())
    )
);

named!(float_structure<String>,
    do_parse!(
        basis: digit >>
        char!('.') >>
        decimal: digit >>
        (str::from_utf8(basis).unwrap().to_string() + "." + str::from_utf8(decimal).unwrap())
    )
);

named!(pub float_literal<Ast>,
    do_parse!(
       num: ws!(float_raw) >>
        (Ast::Literal ( Datatype::Float(num)))
    )
);

#[test]
fn parse_float_test() {
    let (_, value) = match float_raw(b"42.0") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(42.0, value)
}

