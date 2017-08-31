
use datatype::Datatype;
pub type LangResult = Result<Datatype, LangError>;

#[derive(PartialEq, Debug)]
pub enum LangError {
    DivideByZero,
    InvalidEvaluationOfNone, // should never happen
    IdentifierDoesntExist,
    ParserShouldHaveRejected,// should never happen
    UnsupportedArithimaticOperation
}