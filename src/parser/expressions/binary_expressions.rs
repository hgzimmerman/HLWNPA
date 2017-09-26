#[allow(unused_imports)]
use nom::*;
use ast::{Ast, ArithmeticOperator, SExpression};

use parser::operators::{arithmetic_binary_operator, arithmetic_unary_operator, negate, arithmetic_binary_multiplicative_operator };
use parser::{expression_or_literal_or_identifier_or_struct_or_array,
             literal_or_expression_identifier_or_struct_or_array};
use parser::literal::literal;
use parser::identifier::identifier;
use parser::structure::struct_access;
use parser::function_execution;


//Todo create another alt option where just a single token can be accepted, this will allow wrapping of identifiers and literals with () via sexpr_parens, those can then be removed from the top-level program parser as they are covered in the sexpr parser.
named!(pub sexpr<Ast>,
    alt!(
        complete!(do_parse!(
            lhs: alt!( sexpr_multiplicative ) >>
            operator: arithmetic_binary_operator >>
            rhs: expression_or_literal_or_identifier_or_struct_or_array>>
            (create_sexpr(operator, lhs, Some(rhs)))
        )) |
        complete!(do_parse!(
            lhs: alt!( literal | struct_access | identifier | sexpr_parens ) >>
            operator: arithmetic_binary_operator >>
            rhs: expression_or_literal_or_identifier_or_struct_or_array >>
            (create_sexpr(operator, lhs, Some(rhs)))
        )) |
        complete!(do_parse!(
            lhs: alt!(literal | struct_access | identifier | sexpr_parens ) >>
            operator:  arithmetic_unary_operator >>
            (create_sexpr(operator, lhs, None))
        )) |
        complete!(do_parse!(
            operator: negate >>
            lhs: literal_or_expression_identifier_or_struct_or_array >>
            (create_sexpr(operator, lhs, None))
        )) |
        complete!(
             ws!(alt!( literal | struct_access | function_execution | identifier )) // match any of these types
        )
    )
);

named!(pub sexpr_parens<Ast>,
    delimited!(
        ws!(char!('(')),
        ws!(sexpr),
        ws!(char!(')'))
    )
);

named!(sexpr_multiplicative<Ast>,
    do_parse!(
        lhs: alt!( literal | struct_access | function_execution | identifier) >>
        operator: arithmetic_binary_multiplicative_operator >>
        rhs: alt!( complete!(sexpr_multiplicative) | literal | struct_access | function_execution | identifier) >>
        (create_sexpr(operator, lhs, Some(rhs)))
    )
);


/// This isn't exactly bulletproof, in that this could fail if a binary operator is provided without a rhs.
/// This relies on the parser always providing a rhs for binary operators.
fn create_sexpr(operator: ArithmeticOperator, lhs: Ast, rhs: Option<Ast>) -> Ast {
    match operator {
        //Unary
        ArithmeticOperator::Increment => Ast::SExpr(SExpression::Increment(Box::new(lhs))),
        ArithmeticOperator::Decrement => Ast::SExpr(SExpression::Decrement(Box::new(lhs))),
        ArithmeticOperator::Negate => Ast::SExpr(SExpression::Invert(Box::new(lhs))),
        //Binary
        ArithmeticOperator::Plus => Ast::SExpr(SExpression::Add(
            Box::new(lhs),
            Box::new(rhs.expect("rhs should be present")),
        )),
        ArithmeticOperator::Minus => Ast::SExpr(SExpression::Subtract(
            Box::new(lhs),
            Box::new(rhs.expect("rhs should be present")),
        )),
        ArithmeticOperator::Times => Ast::SExpr(SExpression::Multiply(
            Box::new(lhs),
            Box::new(rhs.expect("rhs should be present")),
        )),
        ArithmeticOperator::Divide => Ast::SExpr(SExpression::Divide(
            Box::new(lhs),
            Box::new(rhs.expect("rhs should be present")),
        )),
        ArithmeticOperator::Modulo => Ast::SExpr(SExpression::Modulo(
            Box::new(lhs),
            Box::new(rhs.expect("rhs should be present")),
        )),
        ArithmeticOperator::Equals => Ast::SExpr(SExpression::Equals(
            Box::new(lhs),
            Box::new(rhs.expect("rhs should be present")),
        )),
        ArithmeticOperator::NotEquals => Ast::SExpr(SExpression::NotEquals(
            Box::new(lhs),
            Box::new(rhs.expect("rhs should be present")),
        )),
        ArithmeticOperator::GreaterThan => Ast::SExpr(SExpression::GreaterThan(
            Box::new(lhs),
            Box::new(rhs.expect("rhs should be present")),
        )),
        ArithmeticOperator::LessThan => Ast::SExpr(SExpression::LessThan(
            Box::new(lhs),
            Box::new(rhs.expect("rhs should be present")),
        )),
        ArithmeticOperator::GreaterThanOrEqual => Ast::SExpr(SExpression::GreaterThanOrEqual(
            Box::new(lhs),
            Box::new(rhs.expect("rhs should be present")),
        )),
        ArithmeticOperator::LessThanOrEqual => Ast::SExpr(SExpression::LessThanOrEqual(
            Box::new(lhs),
            Box::new(rhs.expect("rhs should be present")),
        )),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use datatype::Datatype;

    #[test]
    fn sexpr_parse_addition() {
        let (_, value) = match sexpr(b"3 + 4") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Add(
                Box::new(Ast::Literal(Datatype::Number(3))),
                Box::new(Ast::Literal(Datatype::Number(4))),
            )),
            value
        );
    }


    #[test]
    fn sexpr_parse_increment() {
        let (_, value) = match sexpr(b"3++") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Increment(
                Box::new(Ast::Literal(Datatype::Number(3))),
            )),
            value
        );
    }

    #[test]
    fn sexpr_parse_negate() {
        let (_, value) = match sexpr(b"!true") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Invert(
                Box::new(Ast::Literal(Datatype::Bool(true))),
            )),
            value
        );
    }

    #[test]
    fn sexpr_parse_addition_multiple() {
        let (_, value) = match sexpr(b"3 + 4 + 5") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Add(
                Box::new(Ast::Literal(Datatype::Number(3))),
                Box::new(Ast::SExpr(SExpression::Add(
                    Box::new(Ast::Literal(Datatype::Number(4))),
                    Box::new(Ast::Literal(Datatype::Number(5))),
                ))),
            )),
            value
        );
    }


    #[test]
    fn sexpr_parens_parse() {
        let (_, value) = match sexpr(b"(3 + 4) + 5") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Add(
                Box::new(Ast::SExpr(SExpression::Add(
                    Box::new(Ast::Literal(Datatype::Number(3))),
                    Box::new(Ast::Literal(Datatype::Number(4))),
                ))),
                Box::new(Ast::Literal(Datatype::Number(5))),
            )),
            value
        );
    }

    #[test]
    fn sexpr_parens_rhs_parse() {
        let (_, value) = match sexpr(b"3 + (4 + 5)") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Add(
                Box::new(Ast::Literal(Datatype::Number(3))),
                Box::new(Ast::SExpr(SExpression::Add(
                    Box::new(Ast::Literal(Datatype::Number(4))),
                    Box::new(Ast::Literal(Datatype::Number(5))),
                ))),
            )),
            value
        );
    }

    #[test]
    fn sexpr_parens_triple_parse() {
        let (_, value) = match sexpr(b"3 + (4 + 5 + 6)") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Add(
                Box::new(Ast::Literal(Datatype::Number(3))),
                Box::new(Ast::SExpr(SExpression::Add(
                    Box::new(Ast::Literal(Datatype::Number(4))),
                    Box::new(Ast::SExpr(SExpression::Add(
                        Box::new(Ast::Literal(Datatype::Number(5))),
                        Box::new(Ast::Literal(Datatype::Number(6))),
                    ))),
                ))),
            )),
            value
        );
    }

    #[test]
    fn sexpr_parens_negate_parse_1() {
        let (_, value) = match sexpr(b"!(true)") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Invert(
                Box::new(Ast::Literal(Datatype::Bool(true))),
            )
        ), value);
    }

    #[test]
    fn sexpr_parens_negate_parse() {
        let (_, value) = match sexpr_parens(b"(!true)") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Invert(
                Box::new(Ast::Literal(Datatype::Bool(true))),
            )),
            value
        );
    }

    #[test]
    fn sexpr_mult_parse() {
        let (_, value) = match sexpr_multiplicative(b"10 * 3") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Multiply(
                Box::new(Ast::Literal(Datatype::Number(10))),
                Box::new(Ast::Literal(Datatype::Number(3))),
            )),
            value
        );
    }

    #[test]
    fn sexpr_precedence_1_parse() {
        let (_, value) = match sexpr(b"10 * 3 + 1") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Add(
                Box::new(Ast::SExpr(SExpression::Multiply(
                    Box::new(Ast::Literal(Datatype::Number(10))),
                    Box::new(Ast::Literal(Datatype::Number(3))),
                ))),
                Box::new(Ast::Literal(Datatype::Number(1))),
            )),
            value
        );
    }

    #[test]
    fn sexpr_precedence_2_parse() {
        let (_, value) = match sexpr(b"(10 * 3) + 1") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Add(
                Box::new(Ast::SExpr(SExpression::Multiply(
                    Box::new(Ast::Literal(Datatype::Number(10))),
                    Box::new(Ast::Literal(Datatype::Number(3))),
                ))),
                Box::new(Ast::Literal(Datatype::Number(1))),
            )),
            value
        );
    }

    #[test]
    fn sexpr_precedence_3_parse() {
        let (_, value) = match sexpr(b"2 + 10 * 3 + 1") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Add(
                Box::new(Ast::Literal(Datatype::Number(2))),
                Box::new(Ast::SExpr(SExpression::Add(
                    Box::new(Ast::SExpr(SExpression::Multiply(
                        Box::new(Ast::Literal(Datatype::Number(10))),
                        Box::new(Ast::Literal(Datatype::Number(3))),
                    ))),
                    Box::new(Ast::Literal(Datatype::Number(1))),
                )))
            )),
            value
        );
    }

    #[test]
    fn sexpr_precedence_4_parse() {
        let (_, value) = match sexpr(b"10 * 3 * 2") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Multiply(
                Box::new(Ast::Literal(Datatype::Number(10))),
                Box::new(Ast::SExpr(SExpression::Multiply(
                    Box::new(Ast::Literal(Datatype::Number(3))),
                    Box::new(Ast::Literal(Datatype::Number(2))),
                ))),
            )),
            value
        );
    }

    #[test]
    fn sexpr_precedence_5_parse() {
        let (_, value) = match sexpr(b"10 * 3 * 2 + 1") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };

        use std::collections::HashMap;
        let mut map: HashMap<String, Datatype> = HashMap::new();
        assert_eq!(value.evaluate(&mut map).unwrap(), Datatype::Number(61));
        assert_eq!(
            Ast::SExpr(SExpression::Add(
                Box::new(Ast::SExpr(SExpression::Multiply(
                    Box::new(Ast::Literal(Datatype::Number(10))),
                    Box::new(Ast::SExpr(SExpression::Multiply(
                        Box::new(Ast::Literal(Datatype::Number(3))),
                        Box::new(Ast::Literal(Datatype::Number(2))),
                    ))),
                ))),
                Box::new(Ast::Literal(Datatype::Number(1)))
            )),
            value
        );
    }

    #[test]
    fn sexpr_precedence_6_parse() {
        let (_, value) = match sexpr(b"2 + 10 * 3 ") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Add(
                Box::new(Ast::Literal(Datatype::Number(2))),
                Box::new(Ast::SExpr(SExpression::Multiply(
                    Box::new(Ast::Literal(Datatype::Number(10))),
                    Box::new(Ast::Literal(Datatype::Number(3))),
                ))),
            )),
            value
        );
    }
}
