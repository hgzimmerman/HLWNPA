#[allow(unused_imports)]
use nom::*;
use ast::{Ast, BinaryOperator};
use parser::identifier::identifier;
use parser::utilities::expression_or_literal_or_identifier;
use parser::type_signature::type_signature;

// TODO leave the let binding, possibly as a way to declare a const vs mutable structure
named!(pub assignment<Ast>,
    do_parse!(
        ws!(tag!("let")) >>
        id: ws!(identifier) >>
        ws!(tag!(":="))>>
        value: ws!(expression_or_literal_or_identifier) >>
        (Ast::Expression{ operator: BinaryOperator::Assignment, expr1: Box::new(id), expr2: Box::new(value) })
    )
);


/// Used for assigning identifiers to types
named!(pub type_assignment<Ast>,
    do_parse!(
        id: identifier >>
        tag!(":") >>
        type_info: type_signature >>
        (Ast::Expression{ operator: BinaryOperator::FunctionParameterAssignment, expr1: Box::new(id), expr2: Box::new(type_info) })
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
            Ast::Expression {
                operator: BinaryOperator::Assignment,
                expr1: Box::new(Ast::ValueIdentifier("b".to_string())),
                expr2: Box::new(Ast::Literal(Datatype::Number(8))),
            },
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
        assert_eq!(Ast::Expression {operator: BinaryOperator::FunctionParameterAssignment, expr1: Box::new(Ast::ValueIdentifier ( "b".to_string())), expr2: Box::new(Ast::Type ( TypeInfo::Number)) }, value)
    }
}