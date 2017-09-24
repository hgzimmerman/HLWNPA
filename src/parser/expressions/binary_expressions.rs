#[allow(unused_imports)]
use nom::*;
use ast::{Ast, ArithmeticOperator, SExpression};

use parser::operators::{ arithmetic_binary_operator, arithmetic_unary_operator, negate};
use parser::{expression_or_literal_or_identifier_or_struct_or_array, literal_or_expression_identifier_or_struct_or_array};
use parser::literal::literal;
use parser::identifier::identifier;
use parser::structure::struct_access;



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
        ArithmeticOperator::Increment => Ast::SExpr(SExpression::Increment(Box::new(lhs))),
        ArithmeticOperator::Decrement => Ast::SExpr(SExpression::Decrement(Box::new(lhs))),
        ArithmeticOperator::Negate => Ast::SExpr(SExpression::Invert(Box::new(lhs))),

        ArithmeticOperator::Plus => Ast::SExpr(SExpression::Add(Box::new(lhs), Box::new(rhs.expect("rhs should be present")))),
        ArithmeticOperator::Minus => Ast::SExpr(SExpression::Subtract(Box::new(lhs), Box::new(rhs.expect("rhs should be present")))),
        ArithmeticOperator::Times => Ast::SExpr(SExpression::Multiply(Box::new(lhs), Box::new(rhs.expect("rhs should be present")))),
        ArithmeticOperator::Divide => Ast::SExpr(SExpression::Divide(Box::new(lhs), Box::new(rhs.expect("rhs should be present")))),
        ArithmeticOperator::Modulo => Ast::SExpr(SExpression::Modulo(Box::new(lhs), Box::new(rhs.expect("rhs should be present")))),
        ArithmeticOperator::Equals => Ast::SExpr(SExpression::Equals(Box::new(lhs), Box::new(rhs.expect("rhs should be present")))),
        ArithmeticOperator::NotEquals => Ast::SExpr(SExpression::NotEquals(Box::new(lhs), Box::new(rhs.expect("rhs should be present")))),
        ArithmeticOperator::GreaterThan => Ast::SExpr(SExpression::GreaterThan(Box::new(lhs), Box::new(rhs.expect("rhs should be present")))),
        ArithmeticOperator::LessThan => Ast::SExpr(SExpression::LessThan(Box::new(lhs), Box::new(rhs.expect("rhs should be present")))),
        ArithmeticOperator::GreaterThanOrEqual => Ast::SExpr(SExpression::GreaterThanOrEqual(Box::new(lhs), Box::new(rhs.expect("rhs should be present")))),
        ArithmeticOperator::LessThanOrEqual => Ast::SExpr(SExpression::LessThanOrEqual(Box::new(lhs), Box::new(rhs.expect("rhs should be present")))),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use datatype::Datatype;

    #[test]
    fn sexpr_parse_addition_test() {
        let (_, value) = match sexpr(b"3 + 4") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(Ast::SExpr(SExpression::Add(Box::new(Ast::Literal(Datatype::Number(3))), Box::new(Ast::Literal(Datatype::Number(4))))), value);
    }


    #[test]
    fn sexpr_parse_increment_test() {
        let (_, value) = match sexpr(b"3++") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(Ast::SExpr(SExpression::Increment(Box::new(Ast::Literal(Datatype::Number(3))))), value);
    }

    #[test]
    fn sexpr_parse_negate_test() {
        let (_, value) = match sexpr(b"!true") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(Ast::SExpr(SExpression::Invert(Box::new(Ast::Literal(Datatype::Bool(true))))), value);
    }

    #[test]
    fn sexpr_parse_addition_multiple_test() {
        let (_, value) = match sexpr(b"3 + 4 + 5") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(Ast::SExpr(
            SExpression::Add(
                Box::new(Ast::Literal(Datatype::Number(3))),
                Box::new(Ast::SExpr(SExpression::Add(
                    Box::new(Ast::Literal(Datatype::Number(4))),
                    Box::new(Ast::Literal(Datatype::Number(5)))
                )))
            )
        ), value);
    }
}

