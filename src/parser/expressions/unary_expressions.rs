use nom::*;
use ast::{Ast, BinaryOperator, UnaryOperator};
use datatype::Datatype;
use parser::operators::unary_operator;
use parser::expression_or_literal_or_identifier; // move this to utilities once applicable.

named!(unary_expr<Ast>,
    do_parse!(
        op: unary_operator >>
        l: expression_or_literal_or_identifier >>
         (Ast::UnaryExpression{ operator: op, expr: Box::new(l)})
    )
);
named!(pub unary_expr_parens<Ast>,
    delimited!(char!('('), unary_expr, char!(')') )
);
