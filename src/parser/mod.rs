use ast::{Ast, BinaryOperator};

#[allow(unused_imports)]
use nom::*;

use std::boxed::Box;


mod operators;

mod expressions;
use self::expressions::any_expression_parens;

mod identifier;
use self::identifier::identifier;

mod literal;

mod utilities;
use self::utilities::*;

mod assignment;
use self::assignment::assignment;

mod type_signature;

mod function;
pub use self::function::function; // todo, maybe not have this be pub in the future.

mod body;

mod control_flow;
use self::control_flow::{if_expression, while_loop};




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

#[cfg(test)]
mod test {
    use super::*;
    use testing::SIMPLE_PROGRAM_INPUT_1;
    use nom::IResult;
    use ast::{BinaryOperator, UnaryOperator};
    use test::Bencher;
    use datatype::{Datatype, TypeInfo};
    /// assign the value 7 to x
/// create a function that takes a number
/// call the function with x
    #[test]
    fn parse_program_and_validate_ast_test() {
        let (_, value) = match program(SIMPLE_PROGRAM_INPUT_1.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        let expected_assignment: Ast = Ast::Expression {
            operator: BinaryOperator::Assignment,
            expr1: Box::new(Ast::ValueIdentifier("x".to_string())),
            expr2: Box::new(Ast::Literal(Datatype::Number(7))),
        };

        let expected_fn: Ast = Ast::Expression {
            operator: BinaryOperator::Assignment,
            expr1: Box::new(Ast::ValueIdentifier("test_function".to_string())),
            expr2: Box::new(Ast::Literal(Datatype::Function {
                parameters: Box::new(Ast::VecExpression {
                    expressions: vec![Ast::Expression {
                        operator: BinaryOperator::FunctionParameterAssignment,
                        expr1: Box::new(Ast::ValueIdentifier("a".to_string())),
                        expr2: Box::new(Ast::Type(TypeInfo::Number))
                    }],
                }),
                body: Box::new(Ast::VecExpression {
                    expressions: vec![
                        Ast::Expression {
                            operator: BinaryOperator::Plus,
                            expr1: Box::new(Ast::ValueIdentifier("a".to_string())),
                            expr2: Box::new(Ast::Literal(Datatype::Number(8))),
                        }],
                }),
                return_type: Box::new(TypeInfo::Number),
            })),
        };
        let expected_fn_call: Ast = Ast::Expression {
            operator: BinaryOperator::ExecuteFn,
            expr1: Box::new(Ast::ValueIdentifier("test_function".to_string())),
            expr2: Box::new(Ast::VecExpression {
                expressions: vec![Ast::ValueIdentifier("x".to_string())],
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


    #[bench]
    fn parse_simple_program_bench(b: &mut Bencher) {
        fn parse_simple_program() {
            let (_, _) = match program(SIMPLE_PROGRAM_INPUT_1.as_bytes()) {
                IResult::Done(rest, v) => (rest, v),
                IResult::Error(e) => panic!("{}", e),
                _ => panic!(),
            };
        }

        b.iter(|| parse_simple_program());
    }

    #[test]
    fn parse_program_with_only_identifier_test() {
        let input_string = "x";
        let (_, value) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };

        assert_eq!(Ast::VecExpression { expressions: vec![Ast::ValueIdentifier("x".to_string())] }, value)
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
                condition: Box::new(Ast::Literal(Datatype::Bool(true))),
                true_expr: Box::new(Ast::VecExpression { expressions: vec![Ast::Literal(Datatype::Bool(true))] }),
                false_expr: Some(Box::new(Ast::VecExpression { expressions: vec![Ast::Literal(Datatype::Bool(true))] }))
            }]
        }, value)
    }
}