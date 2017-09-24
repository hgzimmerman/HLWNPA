use parser::literal::string_literal;

use nom::*;
use ast::{Ast, SExpression};

use std::boxed::Box;

named!(pub include<Ast>,
    do_parse!(
        ws!(tag!("include")) >>
        filename: string_literal >>
        ( Ast::SExpr(Box::new(SExpression::Include(
            Box::new(filename)
        ))))
    )
);