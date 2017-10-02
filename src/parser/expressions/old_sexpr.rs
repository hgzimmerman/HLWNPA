#[allow(unused_imports)]
use nom::*;
use ast::{Ast, Operator, SExpression};

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
fn create_sexpr_group_left(lhs: Ast, rhss: Vec<(Operator, Option<Ast>)>) -> Ast {
    let mut lhs = lhs;
    for op_and_rhs in rhss {
        let (op, rhs) = op_and_rhs.clone();
        lhs = create_sexpr(op, lhs, rhs) // Group the current lhs with the current rhs and its operator. This will cause a left to right evaluation.
    }
    return lhs;
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

