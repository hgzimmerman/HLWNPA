
use datatype::{Datatype, TypeInfo};
pub type LangResult = Result<Datatype, LangError>;

#[derive(PartialEq, Debug)]
pub enum LangError {
    DivideByZero,
    IdentifierDoesntExist,
    ParserShouldHaveRejected, // should never happen
    UnsupportedArithimaticOperation,
    ConditionOnNonBoolean,
    InvertNonBoolean,
    DecrementNonNumber,
    IncrementNonNumber,
    ExecuteNonFunction,
    ReturnTypeDoesNotMatchReturnValue,
    FunctionParametersShouldBeVecExpression,
    ParameterLengthMismatch,
    InvalidFunctionPrototypeFormatting,
    TypeError { expected: TypeInfo, found: TypeInfo },
    ExpectedDataTypeInfo,
    InvalidSyntax,
    InvalidSyntaxFailedToParse,
    VariableDoesntExist(String),
    TriedToEvaluateTypeInfo(TypeInfo),
    ConditionalNotBoolean(TypeInfo),
    ArrayAccessOnNonArry(TypeInfo),
    InvalidIndexType(Datatype),
    NegativeIndex(i32),
    OutOfBoundsArrayAccess
}
