#[allow(unused_imports)]
use nom::*;
use ast::Ast;
use operator::ArithmeticOperator;
use s_expression::SExpression;

use parser::operators::*;
use parser::utilities::no_keyword_token_group;
use parser::identifier::identifier;


named!(pub sexpr<Ast>,
    alt!(
        complete!(do_parse!(
            lhs: no_keyword_token_group >>
            op_rhss: many0!( alt!(op_and_rhs | array_index | struct_field | function_arguments )  ) >>
            (group_sexpr_by_precedence(lhs, op_rhss))
        )) |
        // captures !
        complete!(do_parse!(
            operator: alt!(negate | invert) >>
            lhs:  no_keyword_token_group >>
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

/// Get an index into an array.
named!( array_index<(ArithmeticOperator, Option<Ast>)>,
    do_parse!(
        index: delimited!(
            ws!(char!('[')),
            ws!(sexpr),
            ws!(char!(']'))
        ) >>
        ( (ArithmeticOperator::ArrayAccess, Some(index)) )
    )
);

/// Get a field belonging to a struct
named!( struct_field<(ArithmeticOperator, Option<Ast>)>,
    do_parse!(
        tag!(".") >>
        field: identifier >>
        ( (ArithmeticOperator::StructAccess, Some(field)) )
    )
);


named!( function_arguments<(ArithmeticOperator, Option<Ast>)>,
    do_parse!(
        arguments: delimited!(
            ws!(char!('(')),
            separated_list_complete!(
                ws!(char!(',')),
                ws!(sexpr)
            ),
            ws!(char!(')'))
        ) >>
        ( ArithmeticOperator::ExecuteFunction, Some(Ast::ExpressionList(arguments)))
    )
);


/// This isn't exactly bulletproof, in that this function could terminate the program if a binary operator is provided without a rhs.
/// Therefore, this relies on the parser always providing a rhs for binary operators.
fn create_sexpr(operator: ArithmeticOperator, lhs: Ast, rhs: Option<Ast>) -> Ast {
    match operator {
        //Language Features
        ArithmeticOperator::ArrayAccess => Ast::SExpr(SExpression::AccessArray {
            identifier: Box::new(lhs),
            index: Box::new(rhs.expect("rhs should be present"))
        }),
        ArithmeticOperator::StructAccess => Ast::SExpr(SExpression::AccessStructField {
            identifier: Box::new(lhs),
            field_identifier: Box::new(rhs.expect("rhs should be present"))
        }),
        ArithmeticOperator::ExecuteFunction=> Ast::SExpr(SExpression::ExecuteFn  {
            identifier: Box::new(lhs),
            parameters: Box::new(rhs.expect("rhs should be present"))
        }),
        //Unary
        ArithmeticOperator::Increment => Ast::SExpr(SExpression::Increment(Box::new(lhs))),
        ArithmeticOperator::Decrement => Ast::SExpr(SExpression::Decrement(Box::new(lhs))),
        ArithmeticOperator::Invert => Ast::SExpr(SExpression::Invert(Box::new(lhs))),
        ArithmeticOperator::Negate => Ast::SExpr(SExpression::Negate(Box::new(lhs))),
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
fn retrieve_operator_and_operands(
    ast: &Ast,
) -> Result<(Option<ArithmeticOperator>, Ast, Option<Ast>), String> {
    match *ast {
        Ast::SExpr(ref sexpr) => {
            match *sexpr {
                SExpression::AccessArray {ref identifier, ref index } => Ok((
                    Some(ArithmeticOperator::ArrayAccess),
                    *identifier.clone(),
                    Some(*index.clone())
                )),
                SExpression::AccessStructField {ref identifier, ref field_identifier } => Ok((
                    Some(ArithmeticOperator::StructAccess),
                    *identifier.clone(),
                    Some(*field_identifier.clone())
                )),
                SExpression::ExecuteFn {ref identifier, ref parameters } => Ok((
                    Some(ArithmeticOperator::ExecuteFunction),
                    *identifier.clone(),
                    Some(*parameters.clone())
                )),
                SExpression::Multiply(ref lhs, ref rhs) => Ok((
                    Some(ArithmeticOperator::Times),
                    *lhs.clone(),
                    Some(*rhs.clone()),
                )),
                SExpression::Divide(ref lhs, ref rhs) => Ok((
                    Some(ArithmeticOperator::Divide),
                    *lhs.clone(),
                    Some(*rhs.clone()),
                )),
                SExpression::Modulo(ref lhs, ref rhs) => Ok((
                    Some(ArithmeticOperator::Modulo),
                    *lhs.clone(),
                    Some(*rhs.clone()),
                )),
                SExpression::Add(ref lhs, ref rhs) => Ok((
                    Some(ArithmeticOperator::Plus),
                    *lhs.clone(),
                    Some(*rhs.clone()),
                )),
                SExpression::Subtract(ref lhs, ref rhs) => Ok((
                    Some(ArithmeticOperator::Minus),
                    *lhs.clone(),
                    Some(*rhs.clone()),
                )),
                SExpression::Equals(ref lhs, ref rhs) => Ok((
                    Some(ArithmeticOperator::Equals),
                    *lhs.clone(),
                    Some(*rhs.clone()),
                )),
                SExpression::NotEquals(ref lhs, ref rhs) => Ok((
                    Some(ArithmeticOperator::NotEquals),
                    *lhs.clone(),
                    Some(*rhs.clone()),
                )),
                SExpression::GreaterThan(ref lhs, ref rhs) => Ok((
                    Some(ArithmeticOperator::GreaterThan),
                    *lhs.clone(),
                    Some(*rhs.clone()),
                )),
                SExpression::LessThan(ref lhs, ref rhs) => Ok((
                    Some(ArithmeticOperator::LessThan),
                    *lhs.clone(),
                    Some(*rhs.clone()),
                )),
                SExpression::GreaterThanOrEqual(ref lhs, ref rhs) => Ok((
                    Some(
                        ArithmeticOperator::GreaterThanOrEqual,
                    ),
                    *lhs.clone(),
                    Some(*rhs.clone()),
                )),
                SExpression::LessThanOrEqual(ref lhs, ref rhs) => Ok((
                    Some(
                        ArithmeticOperator::LessThanOrEqual,
                    ),
                    *lhs.clone(),
                    Some(*rhs.clone()),
                )),
                SExpression::LogicalAnd(ref lhs, ref rhs) => Ok((
                    Some(ArithmeticOperator::LogicalAnd),
                    *lhs.clone(),
                    Some(*rhs.clone()),
                )),
                SExpression::LogicalOr(ref lhs, ref rhs) => Ok((
                    Some(ArithmeticOperator::LogicalOr),
                    *lhs.clone(),
                    Some(*rhs.clone()),
                )),
                SExpression::Invert(ref lhs) => {
                    (Ok((Some(ArithmeticOperator::Invert), *lhs.clone(), None)))
                }
                SExpression::Negate(ref lhs) => {
                    (Ok((Some(ArithmeticOperator::Negate), *lhs.clone(), None)))
                },
                SExpression::Increment(ref lhs) => {
                    (Ok((Some(ArithmeticOperator::Increment), *lhs.clone(), None)))
                }
                SExpression::Decrement(ref lhs) => {
                    (Ok((Some(ArithmeticOperator::Decrement), *lhs.clone(), None)))
                }
                _ => (Err("Unsupported SExpression".to_string())),
            }
        }
//        Ast::Literal(ref literal_dt) => Ok((None, Ast::Literal(literal_dt.clone()), None)),
        _ => {
            (Err(
                "Ast isn't an supported when assigning precedence".to_string(),
            ))
        }
    }
}


fn group_sexpr_by_precedence(lhs: Ast, rhss: Vec<(ArithmeticOperator, Option<Ast>)>) -> Ast {
    let mut lhs = lhs;
    let mut previous_op_value: u32 = 0;
    for op_and_rhs in rhss {
        let (op, rhs): (ArithmeticOperator, Option<Ast>) = op_and_rhs;
        let op_value: u32 = op.clone().into();
        // the a lower value indicates it has more precedence.
        if op_value < previous_op_value {
            let (old_operator, old_lhs, old_rhs) = retrieve_operator_and_operands(&lhs).unwrap(); // TODO possibly bad unwrap
            match old_operator {
                Some(old_operator) => {
                    lhs = create_sexpr(
                        old_operator,
                        old_lhs,
                        Some(create_sexpr(op, old_rhs.unwrap(), rhs)),
                    ) // Group left
                }
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
    use datatype::{Datatype,TypeInfo};



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
    fn sexpr_multi_mult() {
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
    fn sexpr_add_mult_add() {
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
    fn sexpr_parse_invert() {
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
    fn sexpr_parse_negate() {
        let (_, value) = match sexpr(b"-40") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };

        use std::collections::HashMap;
        let mut map: HashMap<String, Datatype> = HashMap::new();

        assert_eq!(
        Ast::SExpr(SExpression::Negate(
            Box::new(Ast::Literal(Datatype::Number(40))),
        )),
        value
        );
        assert_eq!(
            Datatype::Number(-40),
            value.evaluate(&mut map).unwrap()
        )
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
    fn sexpr_add_with_increment() {
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

    #[test]
    fn sexpr_add_with_increment_first() {
        let (_, value) = match sexpr(b"5++ + 2") {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        };
        assert_eq!(
        (Ast::SExpr(SExpression::Add(
            Box::new(Ast::SExpr(SExpression::Increment(
                Box::new(Ast::Literal(Datatype::Number(5)))
            ))),
            Box::new(Ast::Literal(Datatype::Number(2)))
        ))),
        value
        );
    }

    #[test]
    fn parse_array_access_test() {
        let input_string = r##"
        array_identifier[0]
        "##;
        let (_, value) = match sexpr(input_string.as_bytes()) {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(SExpression::AccessArray {
                identifier: Box::new(Ast::ValueIdentifier("array_identifier".to_string())),
                index: Box::new(Ast::Literal(Datatype::Number(0))),
            }),
            value
        )
    }
    #[test]
    fn multiple_dimensional_array_access() {
        let input_string = "
        x[2][1]
        ";
        let (_, ast) = match sexpr(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };

        assert_eq!(
            Ast::SExpr(SExpression::AccessArray {
                identifier: Box::new(Ast::SExpr(SExpression::AccessArray {
                    identifier: Box::new(Ast::ValueIdentifier("x".to_string())),
                    index: Box::new(Ast::Literal(Datatype::Number(2)))
                })),
                index: Box::new(Ast::Literal(Datatype::Number(1)))
            }),
            ast
        )

    }
    #[test]
    fn parse_array_access_on_new_array_test() {
        let input_string = r##"
        [12, 13, 14][0]
        "##;
        let (_, value) = match sexpr(input_string.as_bytes()) {
            IResult::Done(r, v) => (r, v),
            IResult::Error(e) => panic!("{:?}", e),
            _ => panic!(),
        };
        assert_eq!(
            Ast::SExpr(SExpression::AccessArray {
                identifier: Box::new(Ast::Literal(Datatype::Array {
                    value: vec![
                        Datatype::Number(12),
                        Datatype::Number(13),
                        Datatype::Number(14),
                    ],
                    type_: TypeInfo::Number,
                })),
                index: Box::new(Ast::Literal(Datatype::Number(0))),
            }),
            value
        )
    }



    #[test]
    fn parse_struct_access_name() {
        let input_string = "structVariable.field";
        let (_, value) = match sexpr(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };
        let expected_ast = Ast::SExpr(SExpression::AccessStructField {
            identifier: Box::new(Ast::ValueIdentifier("structVariable".to_string())),
            field_identifier: Box::new(Ast::ValueIdentifier("field".to_string())),
        });
        assert_eq!(expected_ast, value)
    }

}
