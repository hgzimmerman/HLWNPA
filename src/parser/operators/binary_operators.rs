use ast::{Ast, BinaryOperator};
use nom::*;
use nom::IResult;

named!(plus<BinaryOperator>,
    value!(
        BinaryOperator::Plus,
        tag!("+")
    )
);
named!(minus<BinaryOperator>,
    value!(
        BinaryOperator::Minus,
        tag!("-")
    )
);

named!(multiply<BinaryOperator>,
     value!(
        BinaryOperator::Multiply,
        tag!("*")
    )
);
named!(divide<BinaryOperator>,
    value!(
        BinaryOperator::Divide,
        tag!("/")
    )
);
named!(modulo<BinaryOperator>,
    value!(
        BinaryOperator::Modulo,
        tag!("%")
    )
);
named!(equals<BinaryOperator>,
    value!(
        BinaryOperator::Equals,
        tag!("==")
    )
);
named!(not_equals<BinaryOperator>,
    value!(
        BinaryOperator::NotEquals,
        tag!("!=")
    )
);
named!(greater_than<BinaryOperator>,
    value!(
        BinaryOperator::GreaterThan,
        tag!(">")
    )
);
named!(less_than<BinaryOperator>,
    value!(
        BinaryOperator::LessThan,
        tag!("<")
    )
);
named!(greater_than_or_eq<BinaryOperator>,
    value!(
        BinaryOperator::GreaterThanOrEqual,
        tag!(">=")
    )
);
named!(less_than_or_eq<BinaryOperator>,
    value!(
        BinaryOperator::LessThanOrEqual,
        tag!("<=")
    )
);


named!( pub binary_operator<BinaryOperator>,
    ws!(alt!(
        plus |
        minus |
        multiply |
        divide |
        modulo |
        equals |
        not_equals |
        greater_than_or_eq | // try to match these before the normal greater_than or less_than operators, because those parsers will preemptivly match input like "<=" leaving the "=" as the remainder of input, causing the next parser to fail.
        less_than_or_eq |
        greater_than |
        less_than
    ))
);


#[test]
fn parse_plus_test() {
    let (_, value) = match plus(b"+") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(BinaryOperator::Plus, value)
}

#[test]
fn parse_binary_operator_test() {
    let (_, value) = match binary_operator(b"%") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(BinaryOperator::Modulo, value)
}
