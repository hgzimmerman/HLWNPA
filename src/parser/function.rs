#[allow(unused_imports)]
use nom::*;
use ast::{Ast, SExpression};
use parser::identifier::identifier;
use parser::body::body;
use parser::type_signature::type_signature;
use datatype::{Datatype, TypeInfo};
use parser::assignment::type_assignment;


/// Either a Type or an identifier that can be resolved to a Struct's Type
named!(function_return_type<Ast>,
    do_parse!(
        ws!(tag!("->")) >>
        return_type: alt!( complete!(type_signature) | complete!(identifier) ) >>
        // Extract the datatype from the Ast::Type provided by the type_signature function
        (return_type)
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
            many0!(ws!(type_assignment)),
            ws!(char!(')'))
        ) >>
        return_type: function_return_type >>
        body_expressions: body >>
        (Ast::SExpr(Box::new(SExpression::CreateFunction {
            identifier: Box::new(function_name),
            fn_parameters_body_and_return_type: Box::new(Ast::Literal (
                Datatype::Function {
                    parameters: Box::new(Ast::VecExpression{expressions: arguments}),
                    body: Box::new(body_expressions),
                    return_type: Box::new(return_type)
                }
            ) )
        })))
    )
);







#[test]
fn parse_whole_function_number_input_returns_number_test() {
    let input_string = "fn test_function ( a : Number ) -> Number { a + 8 }";
    let (_, value) = match function(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };

    let expected_fn: Ast = Ast::SExpr(Box::new(SExpression::CreateFunction {
        identifier: Box::new(Ast::ValueIdentifier("test_function".to_string())),
        fn_parameters_body_and_return_type: Box::new(Ast::Literal(Datatype::Function {
            parameters: Box::new(Ast::VecExpression {
                expressions: vec![Ast::SExpr(Box::new(SExpression::TypeAssignment{
                        identifier: Box::new(Ast::ValueIdentifier ( "a".to_string() )),
                        typeInfo: Box::new(Ast::Type ( TypeInfo::Number ))
                    }))],
            }),
            body: Box::new(Ast::VecExpression {
                expressions: vec![
                    Ast::SExpr(Box::new(
                    SExpression::Add(
                            Box::new(Ast::ValueIdentifier ( "a".to_string() )),
                            Box::new(Ast::Literal ( Datatype::Number(8))),
                        )
                    ))
                ],
            }),
            return_type: Box::new(Ast::Type(TypeInfo::Number)),
        })),
    }));
    assert_eq!(expected_fn, value)
}


#[test]
fn parse_whole_function_identifier_input_returns_number_test() {
    let input_string = "fn test_function ( a : Identifier ) -> Number { a + 8 }";
    let (_, value) = match function(input_string.as_bytes()) {
        IResult::Done(rest, v) => (rest, v),
        IResult::Error(e) => panic!("{}", e),
        _ => panic!(),
    };

    let expected_fn: Ast = Ast::SExpr(Box::new(SExpression::CreateFunction {
        identifier: Box::new(Ast::ValueIdentifier("test_function".to_string())),
        fn_parameters_body_and_return_type: Box::new(Ast::Literal(Datatype::Function {
            parameters: Box::new(Ast::VecExpression {
                expressions: vec![
                    Ast::SExpr(Box::new(SExpression::TypeAssignment{
                        identifier: Box::new(Ast::ValueIdentifier ( "a".to_string() )),
                        typeInfo: Box::new(Ast::ValueIdentifier("Identifier".to_string()))
                    }))
                ],
            }),
            body: Box::new(Ast::VecExpression {
                expressions: vec![
                    Ast::SExpr(Box::new(
                        SExpression::Add(
                            Box::new(Ast::ValueIdentifier ( "a".to_string() )),
                            Box::new(Ast::Literal ( Datatype::Number(8))),
                        )
                    ))
                ],
            }),
            return_type: Box::new(Ast::Type(TypeInfo::Number)),
        })),
    }));
    assert_eq!(expected_fn, value)
}
