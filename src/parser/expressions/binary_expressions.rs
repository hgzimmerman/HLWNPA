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


/// As a rule, the most deeply nested expressions will evaluate first.
/// It is a goal of this parser's design to evaluate operands with the same operator type with left to right order.
/// It is also a goal to establish an order of operators, causing ones with higher precedence to be nested deeper, and therefore evaluated first.
///
/// The order in which the capture groups appear corresponds to their precedence,
/// with the first capture group having the highest precedence.
///
/// Because of the excessive recursiveness of this parser, it has become pretty slow.
named!(pub sexpr_old<Ast>,
    alt!(
        // captures ++, --
        complete!(do_parse!(
            lhs: no_keyword_token_group >>
            operator:  arithmetic_unary_operator >>
            (create_sexpr(operator, lhs, None))
        )) |
        // captures !
        complete!(do_parse!(
            operator: negate >>
            lhs: literal_or_expression_identifier_or_struct_or_array >>
            (create_sexpr(operator, lhs, None))
        )) |
        // captures * / %. Will be evaluated left to right.
        complete!(do_parse!(
           lhs: complete!(sexpr_multiplicative) >>
            // This general operator capture is fine, because the sexpr_multiplicative will fail
            // on non multiplication, division, modulo operators, implying that the next operator
            // must not be one of those. This prevents the last two terms from evaluating right to left.
           operator: arithmetic_binary_operator >>
           rhs: expression_or_literal_or_identifier_or_struct_or_array >>
           (create_sexpr(operator, lhs, Some(rhs)))
        )) |
        // captures + -. Will be evaluated left to right.
        complete!(do_parse!(
           lhs: complete!(sexpr_additive) >>
           operator: arithmetic_binary_operator >>
           rhs: alt!(expression_or_literal_or_identifier_or_struct_or_array) >>
           (create_sexpr(operator, lhs, Some(rhs)))
        )) |
        // captures > < >= <=. Left to right.
        complete!(do_parse!(
           lhs: complete!(sexpr_inequality) >>
           operator: arithmetic_binary_operator >>
           rhs: alt!(expression_or_literal_or_identifier_or_struct_or_array) >>
           (create_sexpr(operator, lhs, Some(rhs)))
        )) |
        // captures == !=. Left to right.
        complete!(do_parse!(
           lhs: complete!(sexpr_equality) >>
           operator: arithmetic_binary_operator >>
           rhs: alt!(expression_or_literal_or_identifier_or_struct_or_array) >>
           (create_sexpr(operator, lhs, Some(rhs)))
        )) |
        complete!(do_parse!(
           lhs: complete!(sexpr_boolean) >>
           operator: arithmetic_binary_operator >>
           rhs: alt!(expression_or_literal_or_identifier_or_struct_or_array) >>
           (create_sexpr(operator, lhs, Some(rhs)))
        )) |

        // catchall, will catch the last two (or one) elements and their operator.
        // All binary capture groups must be enumerated in this alt in their order of appearance above.
        complete!(
             ws!(alt!(
                 complete!(sexpr_multiplicative) |
                 complete!(sexpr_additive) |
                 complete!(sexpr_inequality) |
                 complete!(sexpr_equality) |
                 complete!(sexpr_boolean) |
                 complete!(literal) |
                 complete!(struct_access) |
                 complete!(function_execution) |
                 complete!(identifier) // Should always be last, as it could match defined struct identifiers
             ))
        )
    )
);

named!(pub sexpr<Ast>,
    alt!(
        complete!(do_parse!(
            lhs: no_keyword_token_group >>
            operator:  arithmetic_unary_operator >>
            (create_sexpr(operator, lhs, None))
        )) |
        complete!(do_parse!(
            lhs: no_keyword_token_group >>
            op_rhss: many0!( op_and_rhs ) >>
            (create_sexpr_group_left_alt(lhs, op_rhss))
        )) |
        // captures !
        complete!(do_parse!(
            operator: negate >>
            lhs: literal_or_expression_identifier_or_struct_or_array >>
            (create_sexpr(operator, lhs, None))
        ))
    )
);

named!(op_and_rhs<(ArithmeticOperator, Option<Ast>)>,
    do_parse!(
        op: arithmetic_binary_operator >>
        rhs: no_keyword_token_group >>
        ((op, Some(rhs)))
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
        lhs: no_keyword_token_group >>
        rhs_operator_extensions: many1!(do_parse!(
            operator: alt!(arithmetic_binary_multiplicative_operator)>>
            rhs: alt!(literal | struct_access | identifier | sexpr_parens ) >>
            ((operator, Some(rhs)))
        )) >>
        (create_sexpr_group_left(lhs, rhs_operator_extensions))
    )
);

named!(sexpr_additive<Ast>,
    do_parse!(
        lhs: no_keyword_token_group >>
        rhs_operator_extensions: many1!(do_parse!(
            operator: alt!(arithmetic_binary_additive_operator)>>
            rhs: alt!( complete!(sexpr_multiplicative) | literal | struct_access | identifier | sexpr_parens ) >>
            ((operator, Some(rhs)))
        )) >>
        (create_sexpr_group_left(lhs, rhs_operator_extensions))
    )
);

named!(sexpr_inequality<Ast>,
    do_parse!(
        lhs: no_keyword_token_group >>
        rhs_operator_extensions: many1!(do_parse!(
            operator: alt!(arithmetic_binary_inequality_operator)>>
            rhs: alt!( complete!(sexpr_multiplicative) | complete!(sexpr_additive) | literal | struct_access | identifier | sexpr_parens ) >>
            ((operator, Some(rhs)))
        )) >>
        (create_sexpr_group_left(lhs, rhs_operator_extensions))
    )
);

named!(sexpr_equality<Ast>,
    do_parse!(
        lhs: no_keyword_token_group >>
        rhs_operator_extensions: many1!(do_parse!(
            operator: alt!(arithmetic_binary_equality_operator)>>
            rhs: alt!( complete!(sexpr_multiplicative) | complete!(sexpr_additive) | complete!(sexpr_inequality) | literal | struct_access | identifier | sexpr_parens ) >>
            ((operator, Some(rhs)))
        )) >>
        (create_sexpr_group_left(lhs, rhs_operator_extensions))
    )
);

named!(sexpr_boolean<Ast>,
    do_parse!(
        lhs: no_keyword_token_group >>
        rhs_operator_extensions: many1!(do_parse!(
            operator: alt!(arithmetic_binary_boolean_operator)>>
            rhs: alt!( complete!(sexpr_multiplicative) | complete!(sexpr_additive) | complete!(sexpr_inequality) | complete!(sexpr_equality) | literal | struct_access | identifier | sexpr_parens ) >>
            ((operator, Some(rhs)))
        )) >>
        (create_sexpr_group_left(lhs, rhs_operator_extensions))
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
fn retrieve_operator_and_operands(ast: Ast) -> Result<(Option<ArithmeticOperator>, Ast, Option<Ast>), String>{
    match ast {
        Ast::SExpr(sexpr) => {
            match sexpr {
                SExpression::Multiply(lhs, rhs) => Ok((Some(ArithmeticOperator::Times), *lhs, Some(*rhs))),
                SExpression::Divide(lhs, rhs) => Ok((Some(ArithmeticOperator::Divide), *lhs, Some(*rhs))),
                SExpression::Modulo(lhs, rhs) => Ok((Some(ArithmeticOperator::Modulo), *lhs, Some(*rhs))),
                SExpression::Add(lhs, rhs) => Ok((Some(ArithmeticOperator::Plus), *lhs, Some(*rhs))),
                SExpression::Subtract(lhs, rhs) => Ok((Some(ArithmeticOperator::Minus), *lhs, Some(*rhs))),
                SExpression::Equals(lhs, rhs) => Ok((Some(ArithmeticOperator::Equals), *lhs, Some(*rhs))),
                SExpression::NotEquals(lhs, rhs) => Ok((Some(ArithmeticOperator::NotEquals), *lhs, Some(*rhs))),
                SExpression::GreaterThan(lhs, rhs) => Ok((Some(ArithmeticOperator::GreaterThan), *lhs, Some(*rhs))),
                SExpression::LessThan(lhs, rhs) => Ok((Some(ArithmeticOperator::LessThan), *lhs, Some(*rhs))),
                SExpression::GreaterThanOrEqual(lhs, rhs) => Ok((Some(ArithmeticOperator::GreaterThanOrEqual), *lhs, Some(*rhs))),
                SExpression::LessThanOrEqual(lhs, rhs) => Ok((Some(ArithmeticOperator::LessThanOrEqual), *lhs, Some(*rhs))),
                SExpression::LogicalAnd(lhs, rhs) => Ok((Some(ArithmeticOperator::LogicalAnd), *lhs, Some(*rhs))),
                SExpression::LogicalOr(lhs, rhs) => Ok((Some(ArithmeticOperator::LogicalOr), *lhs, Some(*rhs))),
                _ => (Err("Unsupported SExpression".to_string()))
            }
        }
        Ast::Literal(literal_dt) => Ok((None, Ast::Literal(literal_dt), None)),
        _ => (Err("Ast isn't an supported when assigning precedence".to_string()))
    }
}

/// Given a left hand side AST and a list of operator, right hand side AST pairs,
/// make the LHS contain the existing LHS, and then the operator and RHS.
/// This has the effect of grouping the resulting AST on the LHS, causing it to be evaluated left to right when evaluated.
/// So `1 + 2 + 3 + 4` would be grouped as `((1 + 2) + 3) + 4`.
/// In order to calculate the sum, the innermost expression must be evaluated first, meaning 1 + 2 must be summed first.
///
/// This implementation was born out of the need to group to the LHS,
/// while avoiding stack overflow errors caused by the parser recursively calling itself on the first term
/// (before it had any chance to reject based on the operator, partially breaking the recursion).
/// Instead, this allows the parser to define for a specific operator group
/// that it will capture all operators with the same precedence,
/// then pass that group to this function, ensuring that they will be evaluated left to right,
/// completely avoiding the problem of recursively trying to parse the same expression, and blowing
/// out the stack in the process.
fn create_sexpr_group_left(lhs: Ast, rhss: Vec<(ArithmeticOperator, Option<Ast>)>) -> Ast {
    let mut lhs = lhs;
    for op_and_rhs in rhss {
        let (op, rhs) = op_and_rhs.clone();
        lhs = create_sexpr(op, lhs, rhs) // Group the current lhs with the current rhs and its operator. This will cause a left to right evaluation.
    }
    return lhs;
}


//TODO, this is currently the biggest cost center for the parser. While it isn't aweful, it still isn't great and I should find a way to optimize it.
fn create_sexpr_group_left_alt(lhs: Ast, rhss: Vec<(ArithmeticOperator, Option<Ast>)>) -> Ast {
    let mut lhs = lhs;
    let mut previous_op_value: u32 = 0;
    for op_and_rhs in rhss {
        let (op, rhs): (ArithmeticOperator, Option<Ast>) = op_and_rhs;
        let op_value: u32 =  op.clone().into();
        // the a lower value indicates it has more precedence.
        if op_value < previous_op_value {
            let (old_operator, old_lhs, old_rhs) = retrieve_operator_and_operands(lhs.clone()).unwrap(); // todo bad unwrap
            match old_operator {
                Some(old_operator) => {
                    lhs = create_sexpr(old_operator, old_lhs, Some(create_sexpr(op, old_rhs.unwrap(), rhs)) ) // Group left // Group the current lhs with the current rhs and its operator. This will cause a left to right evaluation.
                },
                None => {
                    // The lack of an operator in the old RHS indicates that this is the first loop, and we can't group left.
                    // So just group right, because there is no difference.
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

//
//    #[test]
//    fn new_sexpr_test_1() {
//        let (_, value) = match sexpr_new(b"3 + 4") {
//            IResult::Done(r, v) => (r, v),
//            IResult::Error(e) => panic!("{:?}", e),
//            _ => panic!(),
//        };
//        assert_eq!(
//            Ast::SExpr(SExpression::Add(
//                Box::new(Ast::Literal(Datatype::Number(3))),
//                Box::new(Ast::Literal(Datatype::Number(4))),
//            )),
//            value
//        );
//    }
//
//
//    #[test]
//    fn new_sexpr_test_2() {
//        let (_, value) = match sexpr_new(b"3") {
//            IResult::Done(r, v) => (r, v),
//            IResult::Error(e) => panic!("{:?}", e),
//            _ => panic!(),
//        };
//        assert_eq!(
//            Ast::Literal(Datatype::Number(3)),
//            value
//        );
//    }
//
//    #[test]
//    fn  new_sexpr_test_3() {
//        let (_, value) = match sexpr(b"3 + 4 + 5") {
//            IResult::Done(r, v) => (r, v),
//            IResult::Error(e) => panic!("{:?}", e),
//            _ => panic!(),
//        };
//        assert_eq!(
//        Ast::SExpr(SExpression::Add(
//            Box::new(Ast::SExpr(SExpression::Add(
//                Box::new(Ast::Literal(Datatype::Number(3))),
//                Box::new(Ast::Literal(Datatype::Number(4))),
//            ))),
//            Box::new(Ast::Literal(Datatype::Number(5))),
//        )),
//        value
//        );
//    }
//

    #[test]
    fn sexpr_identifier() {
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
    fn new_sexpr_test_4() {
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
    fn  new_sexpr_test_5() {
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
    fn  new_sexpr_test_6() {
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
//
//    #[test]
//    fn  new_sexpr_test_5() {
//        let (_, value) = match sexpr(b"3 + 4 * 5 + 6") {
//            IResult::Done(r, v) => (r, v),
//            IResult::Error(e) => panic!("{:?}", e),
//            _ => panic!(),
//        };
//        use std::collections::HashMap;
//        let mut map: HashMap<String, Datatype> = HashMap::new();
//        assert_eq!(Datatype::Number(29), value.evaluate(&mut map).unwrap());
//        assert_eq!(
//        Ast::SExpr(SExpression::Add(
//            Box::new(Ast::SExpr(SExpression::Add (
//                Box::new(Ast::Literal(Datatype::Number(3))),
//                Box::new(Ast::SExpr(SExpression::Multiply(
//                    Box::new(Ast::Literal(Datatype::Number(4))),
//                    Box::new(Ast::Literal(Datatype::Number(5))),
//                ))),
//            ))),
//            Box::new(Ast::Literal(Datatype::Number(6)))
//        )),
//        value
//        );
//
//    }
//
//     #[test]
//    fn new_sexpr_precedence_parse() {
//        let (_, value) = match sexpr_new(b"x > 3 + 5") {
//            IResult::Done(r, v) => (r, v),
//            IResult::Error(e) => panic!("{:?}", e),
//            IResult::Incomplete(i) => panic!("{:?}", i),
//        };
//        assert_eq!(
//        Ast::SExpr(SExpression::GreaterThan(
//            Box::new(Ast::ValueIdentifier("x".to_string())),
//            Box::new(Ast::SExpr(SExpression::Add(
//                Box::new(Ast::Literal(Datatype::Number(3))),
//                Box::new(Ast::Literal(Datatype::Number(5)))
//            )))
//        )),
//        value
//        );
//    }

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
    fn sexpr_div_parse() {
        let (_, value) = match sexpr_multiplicative(b"10 / 3") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        use std::collections::HashMap;
        let mut map: HashMap<String, Datatype> = HashMap::new();
        assert_eq!(value.evaluate(&mut map).unwrap(), Datatype::Number(3));
        assert_eq!(
            Ast::SExpr(SExpression::Divide(
                Box::new(Ast::Literal(Datatype::Number(10))),
                Box::new(Ast::Literal(Datatype::Number(3))),
            )),
            value
        );
    }

    #[test]
    fn sexpr_ineq_parse() {
        let (_, value) = match sexpr_inequality(b"10 > 3") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
        Ast::SExpr(SExpression::GreaterThan(
            Box::new(Ast::Literal(Datatype::Number(10))),
            Box::new(Ast::Literal(Datatype::Number(3))),
        )),
        value
        );
    }

    #[test]
    fn sexpr_eq_parse() {
        let (_, value) = match sexpr_equality(b"10 != 3") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
        Ast::SExpr(SExpression::NotEquals(
            Box::new(Ast::Literal(Datatype::Number(10))),
            Box::new(Ast::Literal(Datatype::Number(3))),
        )),
        value
        );
    }

    #[test]
    fn sexpr_eq_multiple_parse() {
        let (_, value) = match sexpr_equality(b"10 != 3 == true") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
        Ast::SExpr(SExpression::Equals(
            Box::new(Ast::SExpr(SExpression::NotEquals(
                Box::new(Ast::Literal(Datatype::Number(10))),
                Box::new(Ast::Literal(Datatype::Number(3))),
            ))),
            Box::new(Ast::Literal(Datatype::Bool(true)))
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
    fn sexpr_precedence_7_parse() {
        let (_, value) = match sexpr(b"10 * 3 * 2 * 1") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
            Ast::SExpr(SExpression::Multiply(
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
}
