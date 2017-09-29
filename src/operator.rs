/// There are more Operators used than these (S Expressions use more for control flow),
/// but these are the ones that directly map to arithmetic symbols.
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum ArithmeticOperator {
    Increment,
    Decrement,
    Negate,

    Times,
    Divide,
    Modulo,

    Plus,
    Minus,

    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

    Equals,
    NotEquals,

    LogicalAnd,
    LogicalOr,
}

impl Into<u32> for ArithmeticOperator {
    fn into(self) -> u32 {
        use self::ArithmeticOperator::*;
        match self {
            Increment | Decrement | Negate => 0,
            Times | Divide | Modulo => 1,
            Plus | Minus => 2,
            GreaterThan | LessThan | GreaterThanOrEqual | LessThanOrEqual => 3,
            Equals | NotEquals => 4,
            LogicalAnd | LogicalOr => 5,
        }
    }
}
