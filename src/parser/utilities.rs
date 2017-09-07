#[allow(unused_imports)]
use nom::*;
use ast::Ast;

use parser::expressions::any_expression_parens;
use parser::literal::literal;
use parser::identifier::identifier;
use parser::assignment::assignment;
use parser::structure::{create_struct_instance, struct_access};


//TODO this is misnamed, now that it matches other sequences, fix that
named!(pub expression_or_literal_or_identifier<Ast>,
    alt!(
        complete!(any_expression_parens) |
        complete!(literal) |
        complete!(identifier) |
        complete!(create_struct_instance) |
        complete!(struct_access)
    )
);

named!(pub expression_or_literal_or_identifier_or_assignment<Ast>,
    alt!(any_expression_parens | literal | identifier | assignment)
);
