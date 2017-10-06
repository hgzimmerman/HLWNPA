use operator::Operator;
#[allow(unused_imports)]
use nom::*;

named!(plus<Operator>,
    value!(
        Operator::Plus,
        tag!("+")
    )
);
named!(minus<Operator>,
    value!(
        Operator::Minus,
        tag!("-")
    )
);

named!(multiply<Operator>,
     value!(
        Operator::Times,
        tag!("*")
    )
);
named!(divide<Operator>,
    value!(
        Operator::Divide,
        tag!("/")
    )
);
named!(modulo<Operator>,
    value!(
        Operator::Modulo,
        tag!("%")
    )
);
named!(equals<Operator>,
    value!(
        Operator::Equals,
        tag!("==")
    )
);
named!(not_equals<Operator>,
    value!(
        Operator::NotEquals,
        tag!("!=")
    )
);
named!(greater_than<Operator>,
    value!(
        Operator::GreaterThan,
        tag!(">")
    )
);
named!(less_than<Operator>,
    value!(
        Operator::LessThan,
        tag!("<")
    )
);
named!(greater_than_or_eq<Operator>,
    value!(
        Operator::GreaterThanOrEqual,
        tag!(">=")
    )
);
named!(less_than_or_eq<Operator>,
    value!(
        Operator::LessThanOrEqual,
        tag!("<=")
    )
);
named!(logical_and<Operator>,
    value!(
        Operator::LogicalAnd,
        tag!("&&")
    )
);
named!(logical_or<Operator>,
    value!(
        Operator::LogicalOr,
        tag!("||")
    )
);


named!(pub invert<Operator>,
    value!(
        Operator::Invert,
        tag!("!")
    )
);

named!(pub negate<Operator>,
    value!(
        Operator::Negate,
        tag!("-")
    )
);

named!(increment<Operator>,
    value!(
        Operator::Increment,
        tag!("++")

    )
);
named!(decrement<Operator>,
    value!(
        Operator::Decrement,
        tag!("--")
    )
);

named!(assignment<Operator>,
    value!(
        Operator::Assignment,
        tag!(":=")
    )
);


named!( pub arithmetic_binary_operator<Operator>,
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
        logical_or |

        assignment
    ))
);

named!( pub arithmetic_unary_operator<Operator>,
    ws!(alt!(
        increment |
        decrement
    ))
);
