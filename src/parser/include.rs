use parser::literal::string_literal;

use nom::*;
use ast::{Ast, UnaryOperator};

use std::boxed::Box;

named!(pub include<Ast>,
    do_parse!(
        ws!(tag!("include")) >>
        filename: string_literal >>
        ( Ast::UnaryExpression {
            operator: UnaryOperator::Include,
            expr: Box::new(filename)
        }  )
    )
);