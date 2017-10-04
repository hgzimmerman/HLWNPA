
use datatype::{Datatype, TypeInfo};
use std::rc::Rc;
pub type LangResult = Result<Rc<Datatype>, LangError>;

#[derive(PartialEq, Debug)]
pub enum LangError {
    DivideByZero,
    IdentifierDoesntExist,
    ParserShouldHaveRejected, // should never happen
    UnsupportedArithimaticOperation,
    ConditionOnNonBoolean,
    InvertNonBoolean,
    NegateNotNumber,
    DecrementNonNumber,
    IncrementNonNumber,
    ExecuteNonFunction,
    ReturnTypeDoesNotMatchReturnValue,
    FunctionParametersShouldBeExpressionList,
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
    OutOfBoundsArrayAccess,
    FieldTypeNotSupplied,
    FieldIdentifierNotSupplied,
    NonAssignmentInStructDeclaration,
    StructBodyNotSupplied,
    StructNameNotSupplied,
    StructFieldDoesntExist,
    TriedToAccessNonStruct,
    NonAssignmentInStructInit,
    ExpectedIdentifier,
    ExpectedExpression,
    ExpectedIdentifierToBeStructType { found: String },
    InitState,
    CouldNotReadFile { filename: String, reason: String },
    CouldNotParseFile { filename: String, reason: String },
    TriedToGetLengthOfNonArray,
    RangeValueIsntNumber
}
