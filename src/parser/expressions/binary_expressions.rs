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


/// As a rule, the most deeply nested expressions will evaluate first.
/// It is a goal of this parser's design to evaluate operands with the same operator type with left to right order.
/// It is also a goal to establish an order of operators, causing ones with higher precedence to be nested deeper, and therefore evaluated first.
///
/// The order in which the capture groups appear corresponds to their precedence,
/// with the first capture group having the highest precedence.
named!(pub sexpr<Ast>,
    alt!(
        // captures ++, --
        complete!(do_parse!(
            lhs: alt!(literal | struct_access | identifier | sexpr_parens ) >>
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


        // catchall, will catch the last two (or one) elements and their operator.
        // All binary capture groups must be enumerated in this alt in their order of appearance above.
        complete!(
             ws!(alt!(
                 sexpr_multiplicative |
                 sexpr_additive |
                 sexpr_inequality |
                 sexpr_equality |
                 literal |
                 struct_access |
                 function_execution |
                 identifier
             ))
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
        lhs: alt!(literal | struct_access | identifier | sexpr_parens ) >>
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
        lhs: alt!(literal | struct_access | identifier | sexpr_parens ) >>
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
        lhs: alt!(literal | struct_access | identifier | sexpr_parens ) >>
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
        lhs: alt!(literal | struct_access | identifier | sexpr_parens ) >>
        rhs_operator_extensions: many1!(do_parse!(
            operator: alt!(arithmetic_binary_equality_operator)>>
            rhs: alt!( complete!(sexpr_multiplicative) | complete!(sexpr_additive) | complete!(sexpr_inequality) | literal | struct_access | identifier | sexpr_parens ) >>
            ((operator, Some(rhs)))
        )) >>
        (create_sexpr_group_left(lhs, rhs_operator_extensions))
    )
);


/// This isn't exactly bulletproof, in that this could fail if a binary operator is provided without a rhs.
/// This relies on the parser always providing a rhs for binary operators.
fn create_sexpr(operator: ArithmeticOperator, lhs: Ast, rhs: Option<Ast>) -> Ast {
//    println!("create_sexpr lhs:{:?}, rhs{:?}", lhs, rhs);
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
/// completely avoiding the problem of recursively trying to parse the same expression.
///
fn create_sexpr_group_left(lhs: Ast, rhss: Vec<(ArithmeticOperator, Option<Ast>)>) -> Ast {
//    println!("Create_sexpr_group_left lhs:{:?}, rhss{:?}", lhs, rhss);
    let mut lhs = lhs;
    for op_and_rhs in rhss {
        let (op, rhs) = op_and_rhs.clone();
        lhs = create_sexpr(op, lhs, rhs) // Group the current lhs with the current rhs and its operator. This will cause a left to right evaluation.
    }
    return lhs;
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
