#[allow(unused_imports)]
use nom::*;
use ast::{Ast, BinaryOperator};
use parser::identifier::identifier;
use parser::body::type_assignment_body;
use parser::type_signature::type_signature;
use datatype::{Datatype, TypeInfo};


named!(pub struct_definition<Ast>,
    do_parse!(
        ws!(tag!("struct")) >>
        struct_name: ws!(identifier) >>
        struct_body: ws!(type_assignment_body) >> // todo, create a parser that only accepts bodies with function parameter assignments
        (Ast::Expression{
            operator: BinaryOperator::StructDeclaration,
            expr1: Box::new(struct_name),
            expr2: Box::new(struct_body)
        })
    )
);

#[cfg(test)]
mod test {

    use datatype::TypeInfo;
    use super::*;

    #[test]
    fn parse_struct_definition() {
        let input_string = "struct MyStruct { a_number : Number }";
        let (_, value) = match struct_definition(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };
        let expected_struct_ast = Ast::Expression{
            operator: BinaryOperator::StructDeclaration,
            expr1: Box::new(Ast::ValueIdentifier("MyStruct".to_string())),
            expr2: Box::new(Ast::VecExpression{
                expressions: vec![
                    Ast::Expression{
                        operator: BinaryOperator::FunctionParameterAssignment,
                        expr1: Box::new(Ast::ValueIdentifier("a_number".to_string())),
                        expr2: Box::new(Ast::Type(TypeInfo::Number))
                    }
                ]
            })
        };

        assert_eq!(expected_struct_ast,value);
    }
}