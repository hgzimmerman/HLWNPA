#[allow(unused_imports)]
use nom::*;
use ast::Ast;
use s_expression::SExpression;
use parser::identifier::identifier;
use parser::type_signature::type_signature;
use parser::expressions::sexpr;

named!(let_declaration<Ast>,
    do_parse!(
        ws!(tag!("let")) >>
        id: ws!(identifier) >>
        ws!(tag!(":="))>>
        value: sexpr >>
        (Ast::SExpr(SExpression::VariableDeclaration{identifier: Box::new(id), ast: Box::new(value) }))
    )
);

named!(const_declaration<Ast>,
    do_parse!(
        ws!(tag!("const")) >>
        id: ws!(identifier) >>
        ws!(tag!(":="))>>
        value: sexpr >>
        (Ast::SExpr(SExpression::ConstDeclaration{identifier: Box::new(id), ast: Box::new(value) }))
    )
);

named!(pub declaration<Ast>,
    alt!(let_declaration | const_declaration)
);


/// Used for assigning identifiers to types
named!(pub type_assignment<Ast>,
    do_parse!(
        id: identifier >>
        tag!(":") >>
        type_info: alt!( complete!(type_signature) | complete!(identifier)) >> // also takes an identifier that will be checked at runtime to verify it is a structure
        (Ast::SExpr(SExpression::TypeAssignment{identifier: Box::new(id), type_info: Box::new(type_info) }))
    )
);

/// Used for assigning identifiers to types
named!(pub struct_value_assignment<Ast>,
    do_parse!(
        id: identifier >>
        tag!(":") >>
        value: sexpr >>
        (Ast::SExpr(SExpression::FieldAssignment{identifier: Box::new(id), ast: Box::new(value) }))
    )
);


#[cfg(test)]
mod test {
    use super::*;
    use datatype::{Datatype, TypeInfo};

    #[test]
    fn parse_assignment_of_literal_test() {
        let input_string = "let b := 8";
        let (_, value) = match declaration(input_string.as_bytes()) {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(SExpression::VariableDeclaration {
                identifier: Box::new(Ast::ValueIdentifier("b".to_string())),
                ast: Box::new(Ast::Literal(Datatype::Number(8))),
            }),
            value
        )
    }


    #[test]
    fn parse_type_assignment_of_number_test() {
        let input_string = "b : Number";
        let (_, value) = match type_assignment(input_string.as_bytes()) {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(SExpression::TypeAssignment {
                identifier: Box::new(Ast::ValueIdentifier("b".to_string())),
                type_info: Box::new(Ast::Type(TypeInfo::Number)),
            }),
            value
        )
    }
}
