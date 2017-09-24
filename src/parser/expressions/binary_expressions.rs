#[allow(unused_imports)]
use nom::*;
use ast::{Ast, ArithmeticOperator, SExpression};

use parser::operators::{binary_operator, arithmetic_binary_operator, arithmetic_unary_operator, negate};
use parser::{expression_or_literal_or_identifier_or_struct_or_array, literal_or_expression_identifier_or_struct_or_array};
use parser::literal::literal;
use parser::identifier::identifier;
use parser::structure::struct_access;

named!(binary_expr<Ast>,
    do_parse!(
       l1: expression_or_literal_or_identifier_or_struct_or_array >>
       op: binary_operator >>
       l2: expression_or_literal_or_identifier_or_struct_or_array >>
       (Ast::Expression{ operator: op, expr1: Box::new(l1), expr2: Box::new(l2)})
    )
);
named!(pub binary_expr_parens<Ast>,
    delimited!(char!('('), binary_expr, char!(')'))
);


named!(pub sexpr<Ast>,
    alt!(
        complete!(do_parse!(
            lhs: alt!( literal | struct_access | identifier ) >> // TODO, make this alt allow access to literals and variable accesses, but not explicit expressions, stack overflow errors.
            operator: arithmetic_binary_operator >>
            rhs: expression_or_literal_or_identifier_or_struct_or_array >>
            (create_sexpr(operator, lhs, Some(rhs)))
        )) |
        complete!(do_parse!(
            lhs: alt!(literal | struct_access |identifier ) >>
            operator:  arithmetic_unary_operator >>
            (create_sexpr(operator, lhs, None))
        )) |
        complete!(do_parse!(
            operator: negate >>
            lhs: literal_or_expression_identifier_or_struct_or_array >>
            (create_sexpr(operator, lhs, None))
        ))

    )
);

/// This isn't exactly perfect.
fn create_sexpr(operator: ArithmeticOperator, lhs: Ast, rhs: Option<Ast>) -> Ast {
    match operator {
        ArithmeticOperator::Increment => Ast::SExpr(Box::new(SExpression::Increment(Box::new(lhs)))),
        ArithmeticOperator::Decrement => Ast::SExpr(Box::new(SExpression::Decrement(Box::new(lhs)))),
        ArithmeticOperator::Negate => Ast::SExpr(Box::new(SExpression::Invert(Box::new(lhs)))),

        ArithmeticOperator::Plus => Ast::SExpr(Box::new(SExpression::Add(Box::new(lhs), Box::new(rhs.expect("rhs should be present"))))),
        ArithmeticOperator::Minus => Ast::SExpr(Box::new(SExpression::Subtract(Box::new(lhs), Box::new(rhs.expect("rhs should be present"))))),
        ArithmeticOperator::Times => Ast::SExpr(Box::new(SExpression::Multiply(Box::new(lhs), Box::new(rhs.expect("rhs should be present"))))),
        ArithmeticOperator::Divide => Ast::SExpr(Box::new(SExpression::Divide(Box::new(lhs), Box::new(rhs.expect("rhs should be present"))))),
        ArithmeticOperator::Modulo => Ast::SExpr(Box::new(SExpression::Modulo(Box::new(lhs), Box::new(rhs.expect("rhs should be present"))))),
        ArithmeticOperator::Equals => Ast::SExpr(Box::new(SExpression::Equals(Box::new(lhs), Box::new(rhs.expect("rhs should be present"))))),
        ArithmeticOperator::NotEquals => Ast::SExpr(Box::new(SExpression::NotEquals(Box::new(lhs), Box::new(rhs.expect("rhs should be present"))))),
        ArithmeticOperator::GreaterThan => Ast::SExpr(Box::new(SExpression::GreaterThan(Box::new(lhs), Box::new(rhs.expect("rhs should be present"))))),
        ArithmeticOperator::LessThan => Ast::SExpr(Box::new(SExpression::LessThan(Box::new(lhs), Box::new(rhs.expect("rhs should be present"))))),
        ArithmeticOperator::GreaterThanOrEqual => Ast::SExpr(Box::new(SExpression::GreaterThanOrEqual(Box::new(lhs), Box::new(rhs.expect("rhs should be present"))))),
        ArithmeticOperator::LessThanOrEqual => Ast::SExpr(Box::new(SExpression::LessThanOrEqual(Box::new(lhs), Box::new(rhs.expect("rhs should be present"))))),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use datatype::Datatype;
    use ast::BinaryOperator;

//    #[test]
//    fn parse_addition_test() {
//        let (_, value) = match binary_expr(b"3 + 4") {
//            IResult::Done(r, v) => (r, v),
//            IResult::Error(e) => panic!("{:?}", e),
//            _ => panic!(),
//        };
//        assert_eq!(Ast::Expression { operator: BinaryOperator::Plus, expr1: Box::new(Ast::Literal(Datatype::Number(3))), expr2: Box::new(Ast::Literal(Datatype::Number(4))) }, value);
//    }
//
//    #[test]
//    fn parse_addition_parens_test() {
//        let (_, value) = match binary_expr_parens(b"(3 + 4)") {
//            IResult::Done(r, v) => (r, v),
//            IResult::Error(e) => panic!("{:?}", e),
//            _ => panic!(),
//        };
//        assert_eq!(Ast::Expression { operator: BinaryOperator::Plus, expr1: Box::new(Ast::Literal(Datatype::Number(3))), expr2: Box::new(Ast::Literal(Datatype::Number(4))) }, value);
//    }
//
//    #[test]
//    fn parse_nested_addition_parens_test() {
//        let (_, value) = match binary_expr_parens(b"((3 + 4) + 7)") {
//            IResult::Done(r, v) => (r, v),
//            IResult::Error(e) => panic!("{:?}", e),
//            _ => panic!(),
//        };
//        assert_eq!(
//            Ast::Expression {
//                operator: BinaryOperator::Plus,
//                expr1: Box::new(
//                    Ast::Expression {
//                        operator: BinaryOperator::Plus,
//                        expr1: Box::new(Ast::Literal(Datatype::Number(3))),
//                        expr2: Box::new(Ast::Literal(Datatype::Number(4)))
//                    }
//                ),
//                expr2: Box::new(Ast::Literal(Datatype::Number(7)))
//            }, value
//        );
//    }
//
//    #[test]
//    fn parse_string_and_number_addition_test() {
//        let (_, value) = match binary_expr_parens(b"(3 + \"Hi\")") {
//            IResult::Done(r, v) => (r, v),
//            IResult::Error(e) => panic!("{:?}", e),
//            _ => panic!(),
//        };
//        assert_eq!(Ast::Expression { operator: BinaryOperator::Plus, expr1: Box::new(Ast::Literal(Datatype::Number(3))), expr2: Box::new(Ast::Literal(Datatype::String("Hi".to_string()))) }, value);
//    }

    #[test]
    fn sexpr_parse_addition_test() {
        let (_, value) = match sexpr(b"3 + 4") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(Ast::SExpr(Box::new(SExpression::Add(Box::new(Ast::Literal(Datatype::Number(3))), Box::new(Ast::Literal(Datatype::Number(4)))))), value);
    }


    #[test]
    fn sexpr_parse_increment_test() {
        let (_, value) = match sexpr(b"3++") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(Ast::SExpr(Box::new(SExpression::Increment(Box::new(Ast::Literal(Datatype::Number(3)))))), value);
    }

    #[test]
    fn sexpr_parse_negate_test() {
        let (_, value) = match sexpr(b"!true") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(Ast::SExpr(Box::new(SExpression::Invert(Box::new(Ast::Literal(Datatype::Bool(true)))))), value);
    }

    #[test]
    fn sexpr_parse_addition_multiple_test() {
        let (_, value) = match sexpr(b"3 + 4 + 5") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(Ast::SExpr(
            Box::new(SExpression::Add(
                Box::new(Ast::Literal(Datatype::Number(3))),
                Box::new(Ast::SExpr(Box::new(SExpression::Add(
                    Box::new(Ast::Literal(Datatype::Number(4))),
                    Box::new(Ast::Literal(Datatype::Number(5)))
                ))))
            ))
        ), value);
    }
}

