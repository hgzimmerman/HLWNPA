use ast::{Ast};

#[allow(unused_imports)]
use nom::*;


mod operators;

mod expressions;
use self::expressions::sexpr;

mod identifier;

mod literal;

mod utilities;

mod assignment;
use self::assignment::*;

mod type_signature;

mod function;
use self::function::*;

mod body;

mod control_flow;
use self::control_flow::control_flow;

mod structure;
use self::structure::{struct_definition, create_struct_instance};

mod include;
use self::include::include;

///Anything that generates an AST node.
named!(any_ast<Ast>,
    alt_complete!(
        sexpr | // works as a stand in for tokens groups captured no_keyword_token_group
        include |
        declaration |
        control_flow |
        struct_definition |
        create_struct_instance |
        function
    ) // Order is very important here
);



named!(pub program<Ast>,
    do_parse!(
        e: many1!(ws!(any_ast)) >>
        (Ast::ExpressionList( e ))
    )
);



#[cfg(test)]
mod test {
    use super::*;
    use testing::test_constants::SIMPLE_PROGRAM_INPUT_1;
    use nom::IResult;
    use test::Bencher;
    use ast::{Datatype, TypeInfo};
    use preprocessor::preprocess;
    use std::boxed::Box;
    use s_expression::SExpression;

    /// assign the value 7 to x
    /// create a function that takes a number
    /// call the function with x
    #[test]
    fn parse_simple_program_and_validate_ast_test() {
        let (_, value) = match program(SIMPLE_PROGRAM_INPUT_1.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        let expected_assignment: Ast = Ast::SExpr(SExpression::VariableDeclaration {
            identifier: Box::new(Ast::ValueIdentifier("x".to_string())),
            ast: Box::new(Ast::SExpr(SExpression::Add(
                Box::new(Ast::Literal(Datatype::Number(3))),
                Box::new(Ast::Literal(Datatype::Number(4))),
            ))),
        });

        let expected_fn: Ast = Ast::SExpr(SExpression::DeclareFunction {
            identifier: Box::new(Ast::ValueIdentifier("test_function".to_string())),
            function_datatype: Box::new(Ast::Literal(Datatype::Function {
                parameters: Box::new(Ast::ExpressionList (
                    vec![Ast::SExpr(SExpression::TypeAssignment {
                        identifier: Box::new(Ast::ValueIdentifier("a".to_string())),
                        type_info: Box::new(Ast::Type(TypeInfo::Number))
                    })],
                )),
                body: Box::new(Ast::ExpressionList(vec![
                        Ast::SExpr(
                            SExpression::Add(
                                Box::new(Ast::ValueIdentifier ( "a".to_string() )),
                                Box::new(Ast::Literal ( Datatype::Number(8))),
                            )
                        )
                    ])),
                return_type: TypeInfo::Number,
            })),
        });
        let expected_fn_call: Ast = Ast::SExpr(SExpression::ExecuteFn {
            identifier: Box::new(Ast::ValueIdentifier("test_function".to_string())),
            parameters: Box::new(Ast::ExpressionList(
                vec![Ast::ValueIdentifier("x".to_string())],
            )),
        });

        let expected_program_ast: Ast = Ast::ExpressionList(vec![
                expected_assignment,
                expected_fn,
                expected_fn_call
            ]);

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

    #[bench]
    fn preprocess_and_parse_simple_program_bench(b: &mut Bencher) {
        fn parse_simple_program() {
            let preprocessed = preprocess(SIMPLE_PROGRAM_INPUT_1);
            let (_, _) = match program(preprocessed.as_bytes()) {
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

        assert_eq!(Ast::ExpressionList ( vec![Ast::ValueIdentifier("x".to_string())] ), value)
    }


    #[test]
    fn parse_program_with_if_test() {
        let input_string = "if true { true } else { true }";
        let (_, value) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };

        assert_eq!(Ast::ExpressionList (
            vec![Ast::Conditional {
                condition: Box::new(Ast::Literal(Datatype::Bool(true))),
                true_expr: Box::new(Ast::ExpressionList(vec![Ast::Literal(Datatype::Bool(true))])),
                false_expr: Some(Box::new(Ast::ExpressionList(vec![Ast::Literal(Datatype::Bool(true))])))
            }]
        ), value)
    }


    #[test]
    fn parse_program_with_array_assignment() {
        let input_string = r##"
         let myArray := [8, 3]
        "##;
        let (_, _) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };
    }

    #[test]
    fn parse_program_with_array_access() {
        let input_string = r##"
         let value_in_array := existing_array[8]
        "##;
        let (_, _) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };
    }

    #[test]
    fn parse_program_with_struct_definition() {
        let input_string = r##"struct MyStruct {
            a : Number
        }
        "##;
        let (_, _) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };
    }

    #[test]
    fn parse_program_with_struct_creation() {
        let input_string = r##"new MyStruct {
            a : 8
        }
        "##;
        let (_, _) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };
    }

    #[test]
    fn parse_program_with_struct_instance_creation_and_assignment() {
        let input_string = r##"let instance := new MyStruct {
            a : 8
        }
        "##;
        let (_, _) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };
    }

    #[test]
    fn parse_program_with_struct_access_and_assignment() {
        let input_string = r##"
        let outside_value := myStructInstance.field
        "##;
        let (_, _) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };
    }

    #[test]
    fn verify_program_with_escapes_in_strings() {
        let input_string = "
        \"\nHello\nWorld\n\"
        ";
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };

        assert_eq!(Ast::ExpressionList (
            vec![
                Ast::Literal(Datatype::String("\nHello\nWorld\n".to_string()))
            ]
        ), ast)

    }


}
