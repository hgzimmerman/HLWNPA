/// There are more Operators used than these (S Expressions use more for control flow),
/// but these are the ones that directly map to arithmetic symbols.
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Operator {
    ArrayAccess,
    StructAccess,
    ExecuteFunction,

    Increment,
    Decrement,
    Negate, // -
    Invert, // !

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

    Assignment
}

impl Into<u32> for Operator {
    fn into(self) -> u32 {
        use self::Operator::*;
        match self {
            ArrayAccess | StructAccess | ExecuteFunction => 0,
            Increment | Decrement | Negate | Invert => 1,
            Times | Divide | Modulo => 2,
            Plus | Minus => 3,
            GreaterThan | LessThan | GreaterThanOrEqual | LessThanOrEqual => 4,
            Equals | NotEquals => 5,
            LogicalAnd | LogicalOr => 6,
            Assignment => 7
        }
    }
}
