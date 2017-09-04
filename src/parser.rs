use datatype::{Datatype, TypeInfo};
use ast::{Ast, BinaryOperator, UnaryOperator};
use nom::*;
use nom::IResult;
use std::str::FromStr;
use std::str;
use std::boxed::Box;


// ____  _                           ___                       _
//| __ )(_)_ __   __ _ _ __ _   _   / _ \ _ __   ___ _ __ __ _| |_ ___  _ __ ___
//|  _ \| | '_ \ / _` | '__| | | | | | | | '_ \ / _ \ '__/ _` | __/ _ \| '__/ __|
//| |_) | | | | | (_| | |  | |_| | | |_| | |_) |  __/ | | (_| | || (_) | |  \__ \
//|____/|_|_| |_|\__,_|_|   \__, |  \___/| .__/ \___|_|  \__,_|\__\___/|_|  |___/
//                          |___/        |_|

named!(plus<BinaryOperator>,
    value!(
        BinaryOperator::Plus,
        tag!("+")
    )
);
named!(minus<BinaryOperator>,
    value!(
        BinaryOperator::Minus,
        tag!("-")
    )
);

named!(multiply<BinaryOperator>,
     value!(
        BinaryOperator::Multiply,
        tag!("*")
    )
);
named!(divide<BinaryOperator>,
    value!(
        BinaryOperator::Divide,
        tag!("/")
    )
);
named!(modulo<BinaryOperator>,
    value!(
        BinaryOperator::Modulo,
        tag!("%")
    )
);
named!(equals<BinaryOperator>,
    value!(
        BinaryOperator::Equals,
        tag!("==")
    )
);
named!(not_equals<BinaryOperator>,
    value!(
        BinaryOperator::NotEquals,
        tag!("!=")
    )
);
named!(greater_than<BinaryOperator>,
    value!(
        BinaryOperator::GreaterThan,
        tag!(">")
    )
);
named!(less_than<BinaryOperator>,
    value!(
        BinaryOperator::LessThan,
        tag!("<")
    )
);
named!(greater_than_or_eq<BinaryOperator>,
    value!(
        BinaryOperator::GreaterThanOrEqual,
        tag!(">=")
    )
);
named!(less_than_or_eq<BinaryOperator>,
    value!(
        BinaryOperator::LessThanOrEqual,
        tag!("<=")
    )
);


named!(binary_operator<BinaryOperator>,
    ws!(alt!(
        plus |
        minus |
        multiply |
        divide |
        modulo |
        equals |
        not_equals |
        greater_than_or_eq | // try to match these before the normal greater_than or less_than operators, because those parsers will preemptivly match input like "<=" leaving the "=" as the remainder of input, causing the next parser to fail.
        less_than_or_eq |
        greater_than |
        less_than

    ))
);

// _   _                           ___                       _
//| | | |_ __   __ _ _ __ _   _   / _ \ _ __   ___ _ __ __ _| |_ ___  _ __ ___
//| | | | '_ \ / _` | '__| | | | | | | | '_ \ / _ \ '__/ _` | __/ _ \| '__/ __|
//| |_| | | | | (_| | |  | |_| | | |_| | |_) |  __/ | | (_| | || (_) | |  \__ \
// \___/|_| |_|\__,_|_|   \__, |  \___/| .__/ \___|_|  \__,_|\__\___/|_|  |___/
//                        |___/        |_|

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

named!(unary_operator<UnaryOperator>,
    ws!(alt!(invert | increment | decrement))
);


named!(number<i32>,
    do_parse!(
        number: map_res!(
            map_res!(
                recognize!(
                    digit
                ),
                str::from_utf8
            ),
            FromStr::from_str
        ) >>
        (number)
    )
);
named!(number_literal<Ast>,
    do_parse!(
       num: ws!(number) >>
        (Ast::Literal ( Datatype::Number(num)))
    )
);

named!(string<String>,
    do_parse!(
       str: map_res!(
            delimited!(
                tag!("\""),
                take_until!("\""),
                tag!("\"")
            ),
            str::from_utf8
        ) >>
        (str.to_string())
    )
);

named!(string_literal<Ast>,
    do_parse!(
        str: ws!(string) >>
        (Ast::Literal (Datatype::String(str)))
    )
);

named!(bool_false<bool>,
    value!(
        false,
        tag!("false")
    )
);
named!(bool_true<bool>,
    value!(
        true,
        tag!("true")
    )
);
named!(bool_literal<Ast>,
    do_parse!(
        boolean_value: alt!(bool_true | bool_false) >>
        (Ast::Literal (Datatype::Bool(boolean_value)))
    )
);

/// put all literal types here
named!(literal<Ast>,
    alt!(number_literal | string_literal | bool_literal)
);

named!(literal_or_identifier<Ast>,
    alt!(literal | identifier)
);

named!(binary_expr<Ast>,
    do_parse!(
       l1: expression_or_literal_or_identifier >>
       op: binary_operator >>
       l2: expression_or_literal_or_identifier >>
       (Ast::Expression{ operator: op, expr1: Box::new(l1), expr2: Box::new(l2)})
    )
);
named!(binary_expr_parens<Ast>,
    delimited!(char!('('), binary_expr, char!(')'))
);


named!(unary_expr<Ast>,
    do_parse!(
        op: unary_operator >>
        l: expression_or_literal_or_identifier >>
         (Ast::UnaryExpression{ operator: op, expr: Box::new(l)})
    )
);
named!(unary_expr_parens<Ast>,
    delimited!(char!('('), unary_expr, char!(')') )
);

named!(any_expression_parens<Ast>,
    alt!(binary_expr_parens | unary_expr_parens)
);


named!(identifier<Ast>,
    do_parse!(
        not!(reserved_words)>>
        id: ws!(
            accepted_identifier_characters
        ) >>
        (Ast::ValueIdentifier ( id.to_string()))
    )
);

named!(reserved_words,
    alt!(
        ws!(tag!("let")) |
        ws!(tag!("fn")) |
        ws!(tag!("if")) |
        ws!(tag!("else")) |
        ws!(tag!("while")) |
        ws!(tag!("true")) |
        ws!(tag!("false"))
    )
);

named!(accepted_identifier_characters<&str>,
    map_res!(
        is_not!(" \n\t\r.(){}<>[],:;+-*/%!=\""),
        str::from_utf8
    )
);

// TODO leave the let binding, possibly as a way to declare a const vs mutable structure
named!(assignment<Ast>,
    do_parse!(
        ws!(tag!("let")) >>
        id: ws!(identifier) >>
        ws!(tag!(":="))>>
        value: ws!(expression_or_literal_or_identifier) >>
        (Ast::Expression{ operator: BinaryOperator::Assignment, expr1: Box::new(id), expr2: Box::new(value) })
    )
);

// TODO: Consider having this return a TypeInfo and let a higher up parser assign this into the proper AST form.
// ^^ eeeh, probably not.
/// _ts indicates that the parser combinator is a getting a type signature
named!(type_signature<Ast>,
   ws!(alt!(number_ts | string_ts | bool_ts ))
);

named!(number_ts<Ast>,
    value!(
       Ast::Type( TypeInfo::Number),
       tag!("Number")
    )
);
named!(string_ts<Ast>,
    value!(
        Ast::Type( TypeInfo::String),
        tag!("String")
    )
);
named!(bool_ts<Ast>,
    value!(
        Ast::Type(TypeInfo::Bool),
        tag!("Bool")
    )
);


/// Used for assigning identifiers to types
named!(function_parameter_assignment<Ast>,
    do_parse!(
        id: identifier >>
        tag!(":") >>
        type_info: type_signature >>
        (Ast::Expression{ operator: BinaryOperator::FunctionParameterAssignment, expr1: Box::new(id), expr2: Box::new(type_info) })
    )
);

named!(body<Ast>,
    do_parse!(
        statements : delimited!(
            ws!(char!('{')),
            many0!(ws!(expression_or_literal_or_identifier_or_assignment)), // consider making a ; terminate an expression // Also, multiple ast types are valuable here. define a matcher for those. //todo: should be many1
            ws!(tag!("}"))
        ) >>
        (Ast::VecExpression{expressions: statements})
    )
);

named!(function_return_type<TypeInfo>,
    do_parse!(
        ws!(tag!("->")) >>
        return_type: type_signature >>
        // Extract the datatype from the Ast::Type provided by the type_signature function
        (match return_type {
            Ast::Type (datatype) => datatype,
            _ => unreachable!() // this branch should never be encountered. //TODO create an error
        })
    )
);

/// The function definition syntax should look like: fn fn_name(id: datatype, ...) -> return_type { expressions ...}
/// Where the id: datatype is optional
named!(pub function<Ast>,
    do_parse!(
        ws!(tag!("fn")) >>
        function_name: identifier >>
        arguments: delimited!(
            ws!(char!('(')),
            many0!(ws!(function_parameter_assignment)),
            ws!(char!(')'))
        ) >>
        return_type: function_return_type >>
        body_expressions: body >>
        (Ast::Expression{
            operator: BinaryOperator::Assignment,
            expr1: Box::new(function_name),
            expr2: Box::new(Ast::Literal (
                Datatype::Function {
                    parameters: Box::new(Ast::VecExpression{expressions: arguments}),
                    body: Box::new(body_expressions),
                    return_type: Box::new(return_type)
                }
            ) )
        })
    )
);


named!(if_expression<Ast>,
    do_parse!(
        ws!(tag!("if")) >>
        if_conditional: ws!(expression_or_literal_or_identifier) >>
        if_body: ws!(body) >>
        else_body: opt!(
            complete!(
                // nest another do_parse to get the else keyword and its associated block
                do_parse!(
                    ws!(tag!("else")) >>
                    e: map!(
                        ws!(body),
                        Box::new
                    ) >>
                    (e)
                )

            )
        ) >>
        (
        Ast::Conditional {
            condition: Box::new(if_conditional),
            true_expr: Box::new(if_body),
            false_expr: else_body
        })
    )
);


named!(while_loop<Ast>,
    do_parse!(
        ws!(tag!("while")) >>
        while_conditional: ws!(expression_or_literal_or_identifier) >>
        while_body: ws!(body) >>

        (Ast::Expression {
            operator: BinaryOperator::Loop,
            expr1: Box::new(while_conditional),
            expr2: Box::new(while_body)
        })
    )
);


///Anything that generates an AST node.
named!(any_ast<Ast>,
    alt!(
        complete!(function_execution) | // the complete! is necessary, as it causes the function execution parser to return an error instead of an incomplete, allowing the next values to evaluate.
        complete!(assignment) |
        complete!(if_expression) |
        complete!(while_loop) |
        identifier |
        function |
        any_expression_parens
    ) // Order is very important here
);

named!(expression_or_literal_or_identifier<Ast>,
    alt!(any_expression_parens | literal | identifier)
);

named!(expression_or_literal_or_identifier_or_assignment<Ast>,
    alt!(any_expression_parens | literal | identifier | assignment)
);

named!(pub program<Ast>,
    do_parse!(
        e: many1!(ws!(any_ast)) >>
        (Ast::VecExpression{expressions: e})
    )
);

named!(function_execution<Ast>,
    do_parse!(
        function_name: identifier >>
        arguments: delimited!(
            ws!(char!('(')),
            many0!(ws!(expression_or_literal_or_identifier)),
            ws!(char!(')'))
        )
        >>
        (Ast::Expression {
            operator: BinaryOperator::ExecuteFn,
            expr1: Box::new(function_name), // and identifier
            expr2: Box::new(Ast::VecExpression{expressions: arguments})
        })
    )
);


#[test]
fn parse_addition_test() {
    let (_, value) = match binary_expr(b"3 + 4") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Expression {operator: BinaryOperator::Plus, expr1: Box::new(Ast::Literal ( Datatype::Number(3))), expr2:  Box::new(Ast::Literal ( Datatype::Number(4))) }, value);
}

#[test]
fn parse_addition_parens_test() {
    let (_, value) = match binary_expr_parens(b"(3 + 4)") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Expression {operator: BinaryOperator::Plus, expr1: Box::new(Ast::Literal ( Datatype::Number(3))), expr2:  Box::new(Ast::Literal ( Datatype::Number(4))) }, value);
}

#[test]
fn parse_nested_addition_parens_test() {
    let (_, value) = match binary_expr_parens(b"((3 + 4) + 7)") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(
        Ast::Expression {
            operator: BinaryOperator::Plus,
            expr1: Box::new(
                Ast::Expression{
                    operator: BinaryOperator::Plus,
                    expr1: Box::new(Ast::Literal ( Datatype::Number(3))),
                    expr2:  Box::new(Ast::Literal ( Datatype::Number(4)))
                }
            ),
            expr2: Box::new(Ast::Literal(Datatype::Number(7)))
        }, value
    );
}

#[test]
fn parse_plus_test() {
    let (_, value) = match plus(b"+") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(BinaryOperator::Plus, value)
}

#[test]
fn parse_operator_test() {
    let (_, value) = match binary_operator(b"%") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(BinaryOperator::Modulo, value)
}

#[test]
fn parse_identifier_alphanumeric_test() {
    let (_, value) = match identifier(b"variableName") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::ValueIdentifier ( "variableName".to_string()), value)
}

#[test]
fn parse_identifier_underscore_test() {
    let (_, value) = match identifier(b"variable_name ") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::ValueIdentifier ( "variable_name".to_string()), value)
}

#[test]
fn parse_number_test() {
    let (_, value) = match number(b"42") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(42, value)
}

#[test]
fn parse_number_literal_test() {
    let (_, value) = match number_literal(b"42") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Literal ( Datatype::Number(42)), value)
}

#[test]
fn parse_bool_literal_test() {
    let (_, value) = match bool_literal(b"true") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Literal ( Datatype::Bool(true)), value)
}


#[test]
fn parse_string_test() {
    let input_string = "\"Hello World\"";
    let (_, value) = match string(input_string.as_bytes()) {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!("Hello World".to_string(), value)
}

#[test]
fn parse_string_literal_test() {
    let input_string = " \"Hello World\"  ";
    let (_, value) = match string_literal(input_string.as_bytes()) {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Literal ( Datatype::String("Hello World".to_string())), value)
}

#[test]
fn parse_string_and_number_addition_test() {
    let (_, value) = match binary_expr_parens(b"(3 + \"Hi\")") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Expression {operator: BinaryOperator::Plus, expr1: Box::new(Ast::Literal ( Datatype::Number(3))), expr2: Box::new(Ast::Literal ( Datatype::String("Hi".to_string()))) }, value);
}

#[test]
fn parse_assignment_of_literal_test() {
    let input_string = "let b := 8";
    let (_, value) = match assignment(input_string.as_bytes()) {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Expression {operator: BinaryOperator::Assignment, expr1: Box::new(Ast::ValueIdentifier ( "b".to_string())), expr2: Box::new(Ast::Literal ( Datatype::Number(8))) }, value)
}

#[test]
fn parse_function_parameter_assignment_of_type_number_test() {
    let input_string = "b : Number";
    let (_, value) = match function_parameter_assignment(input_string.as_bytes()) {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Expression {operator: BinaryOperator::FunctionParameterAssignment, expr1: Box::new(Ast::ValueIdentifier ( "b".to_string())), expr2: Box::new(Ast::Type ( TypeInfo::Number)) }, value)
}

#[test]
fn parse_body_nocheck_test() {
    let input_string = "{ ( a + 8 ) }";
    let (_, _) = match body(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };
}

#[test]
fn parse_identifier_characters_test() {
    let input_string = "name ";
    let (_, _) = match accepted_identifier_characters(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };
}


#[test]
fn parse_whole_function_number_input_returns_number_test() {
    let input_string = "fn test_function ( a : Number ) -> Number { ( a + 8 ) }";
    let (_, value) = match function(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };

    let expected_fn: Ast = Ast::Expression {
        operator: BinaryOperator::Assignment,
        expr1: Box::new(Ast::ValueIdentifier ("test_function".to_string() )),
        expr2: Box::new(Ast::Literal (
            Datatype::Function {
                parameters: Box::new(Ast::VecExpression {
                    expressions: vec![Ast::Expression {
                    operator: BinaryOperator::FunctionParameterAssignment,
                    expr1: Box::new(Ast::ValueIdentifier ( "a".to_string() )),
                    expr2: Box::new(Ast::Type ( TypeInfo::Number ))
                }],
                }),
                body: Box::new(Ast::VecExpression {
                    expressions: vec![
                    Ast::Expression {
                        operator: BinaryOperator::Plus,
                        expr1: Box::new(Ast::ValueIdentifier ( "a".to_string() )),
                        expr2: Box::new(Ast::Literal ( Datatype::Number(8))),
                    }],
                }),
                return_type: Box::new(TypeInfo::Number),
            },
        )),
    };
    assert_eq!(expected_fn, value)
}

#[test]
fn just_parse_program_test() {
    let input_string = "( 1 + 2)
     let x := 7
     fn test_function ( a : Number ) -> Number { ( a + 8 ) }
     test_function(8)";
    let (_, value) = match program(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };
}


/// assign the value 7 to x
/// create a function that takes a number
/// call the function with x
#[test]
fn parse_program_and_validate_ast_test() {
    let input_string = "
     let x := 7
     fn test_function ( a : Number ) -> Number { ( a + 8 ) }
     test_function(x)";
    let (_, value) = match program(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };

    let expected_assignment: Ast = Ast::Expression {
        operator: BinaryOperator::Assignment,
        expr1: Box::new(Ast::ValueIdentifier ( "x".to_string() )),
        expr2: Box::new(Ast::Literal ( Datatype::Number(7) )),
    };

    let expected_fn: Ast = Ast::Expression {
        operator: BinaryOperator::Assignment,
        expr1: Box::new(Ast::ValueIdentifier ( "test_function".to_string() )),
        expr2: Box::new(Ast::Literal (
            Datatype::Function {
                parameters: Box::new(Ast::VecExpression {
                    expressions: vec![Ast::Expression {
                        operator: BinaryOperator::FunctionParameterAssignment,
                        expr1: Box::new(Ast::ValueIdentifier ( "a".to_string() )),
                        expr2: Box::new(Ast::Type ( TypeInfo::Number ))
                    }],
                }),
                body: Box::new(Ast::VecExpression {
                    expressions: vec![
                        Ast::Expression {
                            operator: BinaryOperator::Plus,
                            expr1: Box::new(Ast::ValueIdentifier ( "a".to_string() )),
                            expr2: Box::new(Ast::Literal ( Datatype::Number(8))),
                        }],
                }),
                return_type: Box::new(TypeInfo::Number),
            },
        )),
    };
    let expected_fn_call: Ast = Ast::Expression {
        operator: BinaryOperator::ExecuteFn,
        expr1: Box::new(Ast::ValueIdentifier ( "test_function".to_string() )),
        expr2: Box::new(Ast::VecExpression {
            expressions: vec![Ast::ValueIdentifier ( "x".to_string() )],
        }),
    };

    let expected_program_ast: Ast = Ast::VecExpression {
        expressions: vec![
            expected_assignment,
            expected_fn,
            expected_fn_call
        ],
    };

    assert_eq!(expected_program_ast, value)

}

#[test]
fn parse_program_with_only_identifier_test() {
    let input_string = "x";
    let (_, value) = match program(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("Error in parsing: {}", e),
        IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
    };

    assert_eq!(Ast::VecExpression {expressions: vec![Ast::ValueIdentifier ( "x".to_string())]}, value)
}

#[test]
fn parse_simple_body_test() {
    let input_string = "{ true }";
    let (_, _) = match body(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("Error in parsing: {}", e),
        IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
    };
}


#[test]
fn parse_simple_body_assignment_test() {
    let input_string = "{ let a := 8 }";
    let (_, _) = match body(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("Error in parsing: {}", e),
        IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
    };
}

#[test]
fn parse_if_statement_test() {
    let input_string = "if true { true }";
    let (_, value) = match if_expression(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("Error in parsing: {}", e),
        IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
    };
    assert_eq!(Ast::Conditional {
        condition: Box::new(Ast::Literal ( Datatype::Bool(true) )),
        true_expr: Box::new(Ast::VecExpression{ expressions: vec![Ast::Literal ( Datatype::Bool(true))]}),
        false_expr: None
    }, value)
}

#[test]
fn parse_if_else_statement_test() {
    let input_string = "if true { true } else { true }";
    let (_, _) = match if_expression(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("Error in parsing: {}", e),
        IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
    };
}

#[test]
fn parse_program_with_if_test() {
    let input_string = "if true { true } else { true }";
    let (_, value) = match program(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("Error in parsing: {}", e),
        IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
    };

    assert_eq!(Ast::VecExpression {
        expressions: vec![Ast::Conditional {
            condition: Box::new(Ast::Literal ( Datatype::Bool(true))),
            true_expr: Box::new(Ast::VecExpression{ expressions: vec![Ast::Literal ( Datatype::Bool(true))]}),
            false_expr: Some(Box::new(Ast::VecExpression{ expressions: vec![Ast::Literal ( Datatype::Bool(true))]}))
        }]
    }, value)
}


#[test]
fn parse_while_loop_test() {
    let input_string = "while true { true }";
    let (_, value) = match while_loop(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("Error in parsing: {}", e),
        IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
    };

    assert_eq!(
        Ast::Expression  {
            operator: BinaryOperator::Loop,
            expr1: Box::new(Ast::Literal( Datatype::Bool(true))),
            expr2: Box::new(Ast::VecExpression{ expressions: vec![Ast::Literal ( Datatype::Bool(true))]})
    }, value)
}
