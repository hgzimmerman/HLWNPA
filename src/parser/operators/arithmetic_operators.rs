use operator::ArithmeticOperator;
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
named!(logical_and<ArithmeticOperator>,
    value!(
        ArithmeticOperator::LogicalAnd,
        tag!("&&")
    )
);
named!(logical_or<ArithmeticOperator>,
    value!(
        ArithmeticOperator::LogicalOr,
        tag!("||")
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

        multiply |
        divide |
        modulo |

        plus |
        minus |

        greater_than_or_eq |
        less_than_or_eq |
        greater_than |
        less_than |

        equals |
        not_equals |

        logical_and |
        logical_or
    ))
);

//named!( pub arithmetic_binary_multiplicative_operator<ArithmeticOperator>,
//    ws!(alt!(
//        multiply |
//        divide |
//        modulo
//    ))
//);
//
//named!( pub arithmetic_binary_additive_operator<ArithmeticOperator>,
//    ws!(alt!(
//        plus |
//        minus
//    ))
//);

//named!( pub arithmetic_binary_inequality_operator<ArithmeticOperator>,
//    ws!(alt!(
//    // try to match <=, >= before the normal greater_than or less_than operators,
//    // because those parsers will preemptivly match input like "<=", taking the "<" and leaving the "="
//    // as the remainder of input, causing the next parser to fail because it didn't expect the "=".
//        greater_than_or_eq |
//        less_than_or_eq |
//        greater_than |
//        less_than
//    ))
//);

//named!( pub arithmetic_binary_equality_operator<ArithmeticOperator>,
//    ws!(alt!(
//        equals |
//        not_equals
//    ))
//);
//
//named!( pub arithmetic_binary_boolean_operator<ArithmeticOperator>,
//    ws!(alt!(
//        logical_and |
//        logical_or
//    ))
//);

named!( pub arithmetic_unary_operator<ArithmeticOperator>,
    ws!(alt!(
        increment |
        decrement
    ))
);
