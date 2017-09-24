#[allow(unused_imports)]
use nom::*;
use ast::{Ast, BinaryOperator, SExpression};
use parser::identifier::identifier;
use parser::utilities::expression_or_literal_or_identifier_or_struct_or_array;
use parser::type_signature::type_signature;

// TODO leave the let binding, possibly as a way to declare a const vs mutable structure
named!(pub assignment<Ast>,
    do_parse!(
        ws!(tag!("let")) >>
        id: ws!(identifier) >>
        ws!(tag!(":="))>>
        value: ws!(expression_or_literal_or_identifier_or_struct_or_array) >>
        (Ast::SExpr(Box::new(SExpression::Assignment{identifier: Box::new(id), ast: Box::new(value) })))
    )
);


/// Used for assigning identifiers to types
named!(pub type_assignment<Ast>,
    do_parse!(
        id: identifier >>
        tag!(":") >>
        type_info: alt!( complete!(type_signature) | complete!(identifier)) >> // also takes an identifier that will be checked at runtime to verify it is a structure
        (Ast::SExpr(Box::new(SExpression::TypeAssignment{identifier: Box::new(id), typeInfo: Box::new(type_info) })))
//        (Ast::Expression{ operator: BinaryOperator::TypeAssignment, expr1: Box::new(id), expr2: Box::new(type_info) })
    )
);

/// Used for assigning identifiers to types
named!(pub struct_value_assignment<Ast>,
    do_parse!(
        id: identifier >>
        tag!(":") >>
        value:  expression_or_literal_or_identifier_or_struct_or_array >>
        (Ast::SExpr(Box::new(SExpression::FieldAssignment{identifier: Box::new(id), ast: Box::new(value) })))
//        (Ast::Expression{ operator: BinaryOperator::FieldAssignment, expr1: Box::new(id), expr2: Box::new(value) })
    )
);


#[cfg(test)]
mod test {
    use super::*;
    use datatype::{Datatype, TypeInfo};

    #[test]
    fn parse_assignment_of_literal_test() {
        let input_string = "let b := 8";
        let (_, value) = match assignment(input_string.as_bytes()) {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(Box::new(SExpression::Assignment {
                identifier: Box::new(Ast::ValueIdentifier("b".to_string())),
                ast: Box::new(Ast::Literal(Datatype::Number(8))),
            })),
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
            Ast::SExpr(Box::new(SExpression::TypeAssignment {
                identifier: Box::new(Ast::ValueIdentifier("b".to_string())),
                typeInfo: Box::new(Ast::Type(TypeInfo::Number)),
            })),
            value
        )
    }
}
