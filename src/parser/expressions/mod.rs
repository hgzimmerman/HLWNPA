#[allow(unused_imports)]
use nom::*;
use ast::{Ast, ArithmeticOperator, SExpression};


use parser::operators::*;
use parser::{expression_or_literal_or_identifier_or_struct_or_array,
             literal_or_expression_identifier_or_struct_or_array};
use parser::literal::literal;
use parser::identifier::identifier;
use parser::structure::struct_access;
use parser::function_execution;
use parser::utilities::no_keyword_token_group;





named!(pub sexpr<Ast>,
    alt!(
//        complete!(do_parse!(
//            lhs: no_keyword_token_group >>
//            operator:  arithmetic_unary_operator >>
//            (create_sexpr(operator, lhs, None))
//        )) |
        complete!(do_parse!(
            lhs: no_keyword_token_group >>
            op_rhss: many0!( op_and_rhs ) >>
            (group_sexpr_by_precedence(lhs, op_rhss))
        )) |
        // captures !
        complete!(do_parse!(
            operator: negate >>
            lhs: literal_or_expression_identifier_or_struct_or_array >>
            (create_sexpr(operator, lhs, None))
        ))
    )
);

/// Grab the righthand side
named!(op_and_rhs<(ArithmeticOperator, Option<Ast>)>,
    alt!(
        complete!(do_parse!(
            op: arithmetic_binary_operator >>
            rhs: no_keyword_token_group >>
            ((op, Some(rhs)))
        )) |
        complete!(do_parse!(
            op: arithmetic_unary_operator >>
            ((op, None))
        ))
    )
);

named!(pub sexpr_parens<Ast>,
    delimited!(
        ws!(char!('(')),
        ws!(sexpr),
        ws!(char!(')'))
    )
);



/// This isn't exactly bulletproof, in that this function could terminate the program if a binary operator is provided without a rhs.
/// Therefore, this relies on the parser always providing a rhs for binary operators.
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
        ArithmeticOperator::LogicalAnd => Ast::SExpr(SExpression::LogicalAnd(
            Box::new(lhs),
            Box::new(rhs.expect("rhs should be present")),
        )),
        ArithmeticOperator::LogicalOr => Ast::SExpr(SExpression::LogicalOr(
            Box::new(lhs),
            Box::new(rhs.expect("rhs should be present")),
        )),
    }
}

/// When creating left-aligned groups, it is necessary to reuse the most recent state of the LHS,
/// so the RHS of that old LHS can be replaced.
/// This method if given that LHS, will deconstruct it into its component parts so they can be used construct a new grouping.
fn retrieve_operator_and_operands(ast: &Ast) -> Result<(Option<ArithmeticOperator>, Ast, Option<Ast>), String>{
    match * ast {
        Ast::SExpr( ref sexpr) => {
            match *sexpr {
                SExpression::Multiply(ref lhs, ref rhs) => Ok((Some(ArithmeticOperator::Times), *lhs.clone(), Some(*rhs.clone()))),
                SExpression::Divide(ref lhs, ref rhs) => Ok((Some(ArithmeticOperator::Divide), *lhs.clone(), Some(*rhs.clone()))),
                SExpression::Modulo(ref lhs, ref rhs) => Ok((Some(ArithmeticOperator::Modulo), *lhs.clone(), Some(*rhs.clone()))),
                SExpression::Add(ref lhs, ref rhs) => Ok((Some(ArithmeticOperator::Plus), *lhs.clone(), Some(*rhs.clone()))),
                SExpression::Subtract(ref lhs, ref rhs) => Ok((Some(ArithmeticOperator::Minus), *lhs.clone(), Some(*rhs.clone()))),
                SExpression::Equals(ref lhs, ref rhs) => Ok((Some(ArithmeticOperator::Equals), *lhs.clone(), Some(*rhs.clone()))),
                SExpression::NotEquals(ref lhs, ref rhs) => Ok((Some(ArithmeticOperator::NotEquals), *lhs.clone(), Some(*rhs.clone()))),
                SExpression::GreaterThan(ref lhs, ref rhs) => Ok((Some(ArithmeticOperator::GreaterThan), *lhs.clone(), Some(*rhs.clone()))),
                SExpression::LessThan(ref lhs, ref rhs) => Ok((Some(ArithmeticOperator::LessThan), *lhs.clone(), Some(*rhs.clone()))),
                SExpression::GreaterThanOrEqual(ref lhs, ref rhs) => Ok((Some(ArithmeticOperator::GreaterThanOrEqual), *lhs.clone(), Some(*rhs.clone()))),
                SExpression::LessThanOrEqual(ref lhs, ref rhs) => Ok((Some(ArithmeticOperator::LessThanOrEqual), *lhs.clone(), Some(*rhs.clone()))),
                SExpression::LogicalAnd(ref lhs, ref rhs) => Ok((Some(ArithmeticOperator::LogicalAnd), *lhs.clone(), Some(*rhs.clone()))),
                SExpression::LogicalOr(ref lhs, ref rhs) => Ok((Some(ArithmeticOperator::LogicalOr), *lhs.clone(), Some(*rhs.clone()))),
                SExpression::Invert(ref lhs) => (Ok((Some(ArithmeticOperator::Negate), *lhs.clone(), None))),
                SExpression::Increment(ref lhs) => (Ok((Some(ArithmeticOperator::Increment), *lhs.clone(), None))),
                SExpression::Decrement(ref lhs) => (Ok((Some(ArithmeticOperator::Decrement), *lhs.clone(), None))),
                _ => (Err("Unsupported SExpression".to_string()))
            }
        }
        Ast::Literal(ref literal_dt) => Ok((None, Ast::Literal(literal_dt.clone()), None)),
        _ => (Err("Ast isn't an supported when assigning precedence".to_string()))
    }
}


//TODO, this is currently the biggest cost center for the parser. While it isn't awful, it still isn't great and I should find a way to optimize it.
fn group_sexpr_by_precedence(lhs: Ast, rhss: Vec<(ArithmeticOperator, Option<Ast>)>) -> Ast {
    let mut lhs = lhs;
    let mut previous_op_value: u32 = 0;
    for op_and_rhs in rhss {
        let (op, rhs): (ArithmeticOperator, Option<Ast>) = op_and_rhs;
        let op_value: u32 =  op.clone().into();
        // the a lower value indicates it has more precedence.
        if op_value < previous_op_value {
            let (old_operator, old_lhs, old_rhs) = retrieve_operator_and_operands(&lhs).unwrap(); // todo bad unwrap
            match old_operator {
                Some(old_operator) => {
                    lhs = create_sexpr(old_operator, old_lhs, Some(create_sexpr(op, old_rhs.unwrap(), rhs)) ) // Group left
                },
                None => {
                    // The lack of an operator in the old RHS indicates that this is the first loop, and we can't group left.
                    // So just group right, because there is no difference for the first term.
                    lhs = create_sexpr(op, lhs, rhs) // expand, grouping towards the right.
                }
            }
        } else {
            lhs = create_sexpr(op, lhs, rhs) // expand, grouping towards the right.
        }
        previous_op_value = op_value;
    }
    return lhs;
}


#[cfg(test)]
mod test {
    use super::*;
    use datatype::Datatype;



    #[test]
    fn new_sexpr_single_literal() {
        let (_, value) = match sexpr(b"3") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::Literal(Datatype::Number(3)),
            value
        );
    }


    #[test]
    fn sexpr_identifier_addition() {
        let (_, value) = match sexpr(b"x + 4") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
        Ast::SExpr(SExpression::Add(
            Box::new(Ast::ValueIdentifier("x".to_string())),
            Box::new(Ast::Literal(Datatype::Number(4))),
        )),
        value
        );
    }
    #[test]
    fn expr_precedence_mult_before_add() {
        let (_, value) = match sexpr(b"3 + 4 * 5") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
        Ast::SExpr(SExpression::Add(
            Box::new(Ast::Literal(Datatype::Number(3))),
            Box::new(Ast::SExpr(SExpression::Multiply(
                Box::new(Ast::Literal(Datatype::Number(4))),
                Box::new(Ast::Literal(Datatype::Number(5))),
            ))),
        )),
        value
        );
    }

        #[test]
    fn  sexpr_multi_mult() {
        let (_, value) = match sexpr(b"3 * 4 * 5") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
        Ast::SExpr(SExpression::Multiply(
            Box::new(Ast::SExpr(SExpression::Multiply(
                Box::new(Ast::Literal(Datatype::Number(3))),
                Box::new(Ast::Literal(Datatype::Number(4))),
            ))),
            Box::new(Ast::Literal(Datatype::Number(5))),
        )),
        value
        );
    }

            #[test]
    fn sexpr_multi_mult_four_terms() {
        let (_, value) = match sexpr(b"3 * 4 * 5 * 6") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
        Ast::SExpr(SExpression::Multiply(
            Box::new(Ast::SExpr(SExpression::Multiply(
                Box::new(Ast::SExpr(SExpression::Multiply(
                    Box::new(Ast::Literal(Datatype::Number(3))),
                    Box::new(Ast::Literal(Datatype::Number(4))),
                ))),
                Box::new(Ast::Literal(Datatype::Number(5))),
            ))),
            Box::new(Ast::Literal(Datatype::Number(6)))
        )),
        value
        );
    }

    #[test]
    fn  sexpr_add_mult_add() {
        let (_, value) = match sexpr(b"3 + 4 * 5 + 6") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        use std::collections::HashMap;
        let mut map: HashMap<String, Datatype> = HashMap::new();
        assert_eq!(Datatype::Number(29), value.evaluate(&mut map).unwrap());
        assert_eq!(
        Ast::SExpr(SExpression::Add(
            Box::new(Ast::SExpr(SExpression::Add (
                Box::new(Ast::Literal(Datatype::Number(3))),
                Box::new(Ast::SExpr(SExpression::Multiply(
                    Box::new(Ast::Literal(Datatype::Number(4))),
                    Box::new(Ast::Literal(Datatype::Number(5))),
                ))),
            ))),
            Box::new(Ast::Literal(Datatype::Number(6)))
        )),
        value
        );

    }

     #[test]
    fn new_sexpr_precedence_parse() {
        let (_, value) = match sexpr(b"x > 3 + 5") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
        Ast::SExpr(SExpression::GreaterThan(
            Box::new(Ast::ValueIdentifier("x".to_string())),
            Box::new(Ast::SExpr(SExpression::Add(
                Box::new(Ast::Literal(Datatype::Number(3))),
                Box::new(Ast::Literal(Datatype::Number(5)))
            )))
        )),
        value
        );
    }

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
    fn sexpr_parse_logical_and() {
        let (_, value) = match sexpr(b"true && false") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(SExpression::LogicalAnd(
                Box::new(Ast::Literal(Datatype::Bool(true))),
                Box::new(Ast::Literal(Datatype::Bool(false)))
            )),
            value
        );
    }

    #[test]
    fn sexpr_parse_logical_or() {
        let (_, value) = match sexpr(b"true || false") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(SExpression::LogicalOr(
                Box::new(Ast::Literal(Datatype::Bool(true))),
                Box::new(Ast::Literal(Datatype::Bool(false)))
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
                    Box::new(Ast::SExpr(SExpression::Add(
                        Box::new(Ast::Literal(Datatype::Number(4))),
                        Box::new(Ast::Literal(Datatype::Number(5)))
                    ))),
                    Box::new(Ast::Literal(Datatype::Number(6)))
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
    fn sexpr_eq_and_ineq_parse() {
        // 10 > 3 must evaluate first.
        let (_, value) = match sexpr(b"true == 10 > 3") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
        Ast::SExpr(SExpression::Equals(
            Box::new(Ast::Literal(Datatype::Bool(true))),
            Box::new(Ast::SExpr(SExpression::GreaterThan(
                Box::new(Ast::Literal(Datatype::Number(10))),
                Box::new(Ast::Literal(Datatype::Number(3))),
            )))
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
                Box::new(Ast::SExpr(SExpression::Add(
                    Box::new(Ast::Literal(Datatype::Number(2))),
                    Box::new(Ast::SExpr(SExpression::Multiply(
                        Box::new(Ast::Literal(Datatype::Number(10))),
                        Box::new(Ast::Literal(Datatype::Number(3))),
                    ))),
                ))),
                Box::new(Ast::Literal(Datatype::Number(1))),

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
                Box::new(Ast::SExpr(SExpression::Multiply(
                    Box::new(Ast::Literal(Datatype::Number(10))),
                    Box::new(Ast::Literal(Datatype::Number(3))),
                ))),
                Box::new(Ast::Literal(Datatype::Number(2))),
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
        assert_eq!(
            Ast::SExpr(SExpression::Add(
                Box::new(Ast::SExpr(SExpression::Multiply(
                    Box::new(Ast::SExpr(SExpression::Multiply(
                        Box::new(Ast::Literal(Datatype::Number(10))),
                        Box::new(Ast::Literal(Datatype::Number(3))),
                    ))),
                    Box::new(Ast::Literal(Datatype::Number(2))),
                ))),
                Box::new(Ast::Literal(Datatype::Number(1)))
            )),
            value
        );
        assert_eq!( Datatype::Number(61), value.evaluate(&mut map).unwrap());
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

    #[test]
    fn sexpr_precedence_8_parse() {
        let (_, value) = match sexpr(b"10 > 3 + 5") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
        Ast::SExpr(SExpression::GreaterThan(
            Box::new(Ast::Literal(Datatype::Number(10))),
            Box::new(Ast::SExpr(SExpression::Add(
                Box::new(Ast::Literal(Datatype::Number(3))),
                Box::new(Ast::Literal(Datatype::Number(5)))
            )))
        )),
        value
        );
    }
       #[test]
    fn sexpr_precedence_9_parse() {
        let (_, value) = match sexpr(b"x > 3 + 5") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
        Ast::SExpr(SExpression::GreaterThan(
            Box::new(Ast::ValueIdentifier("x".to_string())),
            Box::new(Ast::SExpr(SExpression::Add(
                Box::new(Ast::Literal(Datatype::Number(3))),
                Box::new(Ast::Literal(Datatype::Number(5)))
            )))
        )),
        value
        );
    }

    /// 10 will multiply with 3 before being divided by 2. This indicates a left to right evaluation.
    #[test]
    fn sexpr_precedence_mult_then_divide_parse() {
        let (_, value) = match sexpr(b"10 * 3 / 2") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };

        use std::collections::HashMap;
        let mut map: HashMap<String, Datatype> = HashMap::new();
        assert_eq!(value.evaluate(&mut map).unwrap(), Datatype::Number(15));
        assert_eq!(
            Ast::SExpr(SExpression::Divide(
                Box::new(Ast::SExpr(SExpression::Multiply(
                    Box::new(Ast::Literal(Datatype::Number(10))),
                    Box::new(Ast::Literal(Datatype::Number(3))),
                ))),
                Box::new(Ast::Literal(Datatype::Number(2))),
            )),
            value
        );
    }

    #[test]
    fn sexpr_fail() {
        let (_, value) = match sexpr(b"5 + 2++") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
            (Ast::SExpr(SExpression::Add(
                Box::new(Ast::Literal(Datatype::Number(5))),
                Box::new(Ast::SExpr(SExpression::Increment(
                    Box::new(Ast::Literal(Datatype::Number(2)))
                )))
            ))),
        value
        );
    }
}