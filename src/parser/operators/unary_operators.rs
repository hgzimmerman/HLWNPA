use ast::{UnaryOperator};
#[allow(unused_imports)]
use nom::*;

named!(invert<UnaryOperator>,
    value!(
        UnaryOperator::Invert,
        tag!("!")
    )
);
named!(increment<UnaryOperator>,
    value!(
        UnaryOperator::Increment,
        tag!("++")

    )
);
named!(decrement<UnaryOperator>,
    value!(
        UnaryOperator::Decrement,
        tag!("--")
    )
);

named!( pub unary_operator<UnaryOperator>,
    ws!(alt!(invert | increment | decrement))
);
