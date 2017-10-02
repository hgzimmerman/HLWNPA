use ast::Ast;
use std::boxed::Box;

/// Operators that store their operands.
/// The Ast's evaluate() method will preform different logic on the operands depending on the operator.
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum SExpression {
    //BinaryOperators
    Add(Box<Ast>, Box<Ast>),
    Subtract(Box<Ast>, Box<Ast>),
    Multiply(Box<Ast>, Box<Ast>),
    Divide(Box<Ast>, Box<Ast>),
    Modulo(Box<Ast>, Box<Ast>),
    Equals(Box<Ast>, Box<Ast>),
    NotEquals(Box<Ast>, Box<Ast>),
    GreaterThan(Box<Ast>, Box<Ast>),
    LessThan(Box<Ast>, Box<Ast>),
    GreaterThanOrEqual(Box<Ast>, Box<Ast>),
    LessThanOrEqual(Box<Ast>, Box<Ast>),
    LogicalAnd(Box<Ast>, Box<Ast>),
    LogicalOr(Box<Ast>, Box<Ast>),
    //Unary Operators
    Print(Box<Ast>),
    Include(Box<Ast>),
    Invert(Box<Ast>),
    Negate(Box<Ast>),
    Increment(Box<Ast>),
    Decrement(Box<Ast>),
    // Language Features
    Assignment { identifier: Box<Ast>, ast: Box<Ast> },
    TypeAssignment {
        identifier: Box<Ast>,
        type_info: Box<Ast>,
    },
    FieldAssignment { identifier: Box<Ast>, ast: Box<Ast> },
    CreateFunction {
        identifier: Box<Ast>,
        function_datatype: Box<Ast>,
    },
    CreateStruct {
        identifier: Box<Ast>,
        struct_datatype: Box<Ast>,
    },
    Loop {
        conditional: Box<Ast>,
        body: Box<Ast>,
    },
    AccessArray {
        identifier: Box<Ast>,
        index: Box<Ast>,
    },
    GetArrayLength ( Box<Ast> ),
    Range{
        start: Box<Ast>,
        end: Box<Ast>
    },
    StructDeclaration {
        identifier: Box<Ast>,
        struct_type_info: Box<Ast>,
    },
    AccessStructField {
        identifier: Box<Ast>,
        field_identifier: Box<Ast>,
    },
    ExecuteFn {
        identifier: Box<Ast>,
        parameters: Box<Ast>,
    },
}