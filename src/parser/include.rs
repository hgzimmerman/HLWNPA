use parser::literal::string_literal;

#[allow(unused_imports)]
use nom::*;
use ast::{Ast, SExpression};

use std::boxed::Box;

named!(pub include<Ast>,
    do_parse!(
        ws!(tag!("include")) >>
        filename: string_literal >>
        ( Ast::SExpr(SExpression::Include(
            Box::new(filename)
        )))
    )
);
