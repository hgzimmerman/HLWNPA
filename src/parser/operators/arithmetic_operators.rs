use ast::ArithmeticOperator;
#[allow(unused_imports)]
use nom::*;

named!(plus<ArithmeticOperator>,
    value!(
        ArithmeticOperator::Plus,
        tag!("+")
    )
);
named!(minus<ArithmeticOperator>,
    value!(
        ArithmeticOperator::Minus,
        tag!("-")
    )
);

named!(multiply<ArithmeticOperator>,
     value!(
        ArithmeticOperator::Times,
        tag!("*")
    )
);
named!(divide<ArithmeticOperator>,
    value!(
        ArithmeticOperator::Divide,
        tag!("/")
    )
);
named!(modulo<ArithmeticOperator>,
    value!(
        ArithmeticOperator::Modulo,
        tag!("%")
    )
);
named!(equals<ArithmeticOperator>,
    value!(
        ArithmeticOperator::Equals,
        tag!("==")
    )
);
named!(not_equals<ArithmeticOperator>,
    value!(
        ArithmeticOperator::NotEquals,
        tag!("!=")
    )
);
named!(greater_than<ArithmeticOperator>,
    value!(
        ArithmeticOperator::GreaterThan,
        tag!(">")
    )
);
named!(less_than<ArithmeticOperator>,
    value!(
        ArithmeticOperator::LessThan,
        tag!("<")
    )
);
named!(greater_than_or_eq<ArithmeticOperator>,
    value!(
        ArithmeticOperator::GreaterThanOrEqual,
        tag!(">=")
    )
);
named!(less_than_or_eq<ArithmeticOperator>,
    value!(
        ArithmeticOperator::LessThanOrEqual,
        tag!("<=")
    )
);

named!(pub negate<ArithmeticOperator>,
    value!(
        ArithmeticOperator::Negate,
        tag!("!")
    )
);
named!(increment<ArithmeticOperator>,
    value!(
        ArithmeticOperator::Increment,
        tag!("++")

    )
);
named!(decrement<ArithmeticOperator>,
    value!(
        ArithmeticOperator::Decrement,
        tag!("--")
    )
);


named!( pub arithmetic_binary_operator<ArithmeticOperator>,
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

named!( pub arithmetic_unary_operator<ArithmeticOperator>,
    ws!(alt!(
        increment |
        decrement
    ))
);
