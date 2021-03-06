#[allow(unused_imports)]
use nom::*;
use ast::Ast;
use s_expression::SExpression;
use parser::identifier::identifier;
use parser::body::{type_assignment_body, struct_init_body};


named!(pub struct_definition<Ast>,
    do_parse!(
        ws!(tag!("struct")) >>
        struct_name: ws!(identifier) >>
        struct_body: ws!(type_assignment_body) >> // todo, create a parser that only accepts bodies with function parameter assignments
        (Ast::SExpr(SExpression::StructDeclaration{
            identifier: Box::new(struct_name),
            struct_type_info: Box::new(struct_body)
        }))
    )
);


named!(pub create_struct_instance<Ast>,
    do_parse!(
        ws!(tag!("new")) >>
        struct_type: ws!(identifier) >>
        body: ws!(struct_init_body) >>
        (Ast::SExpr(SExpression::CreateStruct{
            identifier: Box::new(struct_type),
            struct_datatype: Box::new(body)
        }))
    )
);

#[cfg(test)]
mod test {

    use ast::{Datatype, TypeInfo};
    use super::*;

    #[test]
    fn parse_struct_definition() {
        let input_string = "struct MyStruct { a_number : Number }";
        let (_, value) = match struct_definition(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };
        let expected_struct_ast = Ast::SExpr(SExpression::StructDeclaration {
            identifier: Box::new(Ast::ValueIdentifier("MyStruct".to_string())),
            struct_type_info: Box::new(Ast::ExpressionList(vec![
                Ast::SExpr(SExpression::TypeAssignment {
                    identifier: Box::new(Ast::ValueIdentifier("a_number".to_string())),
                    type_info: Box::new(Ast::Type(TypeInfo::Number)),
                }),
            ])),
        });

        assert_eq!(expected_struct_ast, value);
    }



    #[test]
    fn parse_new_struct() {
        let input_string = r##"new MyStruct {
            a : 8
        }""##;
        let (_, value) = match create_struct_instance(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };
        let expected_ast = Ast::SExpr(SExpression::CreateStruct {
            identifier: Box::new(Ast::ValueIdentifier("MyStruct".to_string())),
            struct_datatype: Box::new(Ast::ExpressionList(vec![
                Ast::SExpr(SExpression::FieldAssignment {
                    identifier: Box::new(Ast::ValueIdentifier("a".to_string())),
                    ast: Box::new(Ast::Literal(Datatype::Number(8))),
                }),
            ])),
        });

        assert_eq!(expected_ast, value);


    }


}
