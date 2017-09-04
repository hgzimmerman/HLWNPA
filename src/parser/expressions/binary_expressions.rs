#[allow(unused_imports)]
use nom::*;
use ast::{Ast};

use parser::operators::binary_operator;
use parser::expression_or_literal_or_identifier;

named!(binary_expr<Ast>,
    do_parse!(
       l1: expression_or_literal_or_identifier >>
       op: binary_operator >>
       l2: expression_or_literal_or_identifier >>
       (Ast::Expression{ operator: op, expr1: Box::new(l1), expr2: Box::new(l2)})
    )
);
named!(pub binary_expr_parens<Ast>,
    delimited!(char!('('), binary_expr, char!(')'))
);

#[cfg(test)]
mod test {
    use super::*;
    use datatype::Datatype;
    use ast::BinaryOperator;

    #[test]
    fn parse_addition_test() {
        let (_, value) = match binary_expr(b"3 + 4") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(Ast::Expression { operator: BinaryOperator::Plus, expr1: Box::new(Ast::Literal(Datatype::Number(3))), expr2: Box::new(Ast::Literal(Datatype::Number(4))) }, value);
    }

    #[test]
    fn parse_addition_parens_test() {
        let (_, value) = match binary_expr_parens(b"(3 + 4)") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(Ast::Expression { operator: BinaryOperator::Plus, expr1: Box::new(Ast::Literal(Datatype::Number(3))), expr2: Box::new(Ast::Literal(Datatype::Number(4))) }, value);
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
                    Ast::Expression {
                        operator: BinaryOperator::Plus,
                        expr1: Box::new(Ast::Literal(Datatype::Number(3))),
                        expr2: Box::new(Ast::Literal(Datatype::Number(4)))
                    }
                ),
                expr2: Box::new(Ast::Literal(Datatype::Number(7)))
            }, value
        );
    }

    #[test]
    fn parse_string_and_number_addition_test() {
        let (_, value) = match binary_expr_parens(b"(3 + \"Hi\")") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(Ast::Expression { operator: BinaryOperator::Plus, expr1: Box::new(Ast::Literal(Datatype::Number(3))), expr2: Box::new(Ast::Literal(Datatype::String("Hi".to_string()))) }, value);
    }
}
