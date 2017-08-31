
use datatype::Datatype;
pub type LangResult = Result<Datatype, LangError>;

#[derive(PartialEq, Debug)]
pub enum LangError {
    DivideByZero,
    IdentifierDoesntExist,
    ParserShouldHaveRejected,// should never happen
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

}