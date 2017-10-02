#[allow(unused_imports)]
use nom::*;
use ast::Ast;
use s_expression::SExpression;
use parser::body::body;
use std::boxed::Box;
use parser::expressions::sexpr;
use parser::identifier::identifier;
use datatype::Datatype;


named!(pub for_loop<Ast>,
    do_parse!(
        ws!(tag!("for")) >>
        variable: ws!(identifier) >>
        ws!(tag!("in")) >>
        array: ws!(sexpr) >>
        for_body: ws!(body) >>

        ( create_for_loop(variable, array, for_body) )
    )
);

fn create_for_loop(identifier: Ast, array: Ast, for_body: Ast) -> Ast {
    Ast::ExpressionList(vec![
        Ast::SExpr(SExpression::Assignment {
            identifier: Box::new(Ast::ValueIdentifier("index".to_string())), // todo, change the index id to some randomly generated hash value
            ast: Box::new(Ast::Literal(Datatype::Number(0))) // 0 index
        }),

        Ast::SExpr(SExpression::Loop {
            conditional: Box::new(Ast::SExpr(SExpression::LessThan (
                Box::new(Ast::ValueIdentifier("index".to_string())),
                Box::new(Ast::SExpr(SExpression::GetArrayLength(Box::new(array.clone())))) // TODO consider just getting the array length here and avoiding polluting the SExpression enum with more arbitray types and avoiding the clone()
            ) )),
            body: Box::new(Ast::ExpressionList(vec![
                Ast::SExpr(SExpression::Assignment {
                    identifier: Box::new(identifier),
                    ast: Box::new(Ast::SExpr(SExpression::AccessArray {
                        identifier: Box::new(array), //TODO this could create a new array for every iteration. I should target this with a pre-optimizer before the program executes.
                        index: Box::new(Ast::ValueIdentifier("index".to_string())),
                    }))
                }), // Assign the value at the index to the given identifier
                for_body, // execute the specified body
                Ast::SExpr(
                    SExpression::Assignment {
                        identifier: Box::new(Ast::ValueIdentifier("index".to_string())),
                        ast: Box::new(Ast::SExpr(SExpression::Increment(Box::new(Ast::ValueIdentifier("index".to_string())))))
                    }
                ) // increment the index
            ]))
        })

    ])
}

    #[test]
    fn for_loop_parse() {
        let input_string = r#"
        for i in [0,2] {
            3
        }
         "#;
        let (_, ast) = match for_loop(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        use datatype::TypeInfo;


        let expected_ast = create_for_loop(
            Ast::ValueIdentifier("i".to_string()),
            Ast::Literal(Datatype::Array {
                value: vec![Datatype::Number(0), Datatype::Number(2)],
                type_: TypeInfo::Number
            }),
            Ast::ExpressionList(vec![Ast::Literal(Datatype::Number(3))])
        );

        assert_eq!(expected_ast, ast);
    }

