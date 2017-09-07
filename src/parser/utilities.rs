#[allow(unused_imports)]
use nom::*;
use ast::Ast;

use parser::expressions::any_expression_parens;
use parser::literal::literal;
use parser::identifier::identifier;
use parser::assignment::assignment;
use parser::structure::{create_struct_instance, struct_access};
use parser::array::array_access;


//TODO this is misnamed, now that it matches other sequences, fix that
named!(pub expression_or_literal_or_identifier<Ast>,
    alt!(
        complete!(any_expression_parens) |
        complete!(literal) |
        complete!(struct_access) | // must come before identifier
        complete!(create_struct_instance) |
        complete!(identifier)
//        complete!(array_access)
    )
);

/// Because array_access requires something that resolves something to an array, using recursive combinators would overflow the stack.
/// So don't use this in array_access.
/// This decision prevents easy access for nested array notation like: array[30][3]
named!(pub expression_or_literal_or_identifier_or_struct_or_array<Ast>,
    alt!(
        complete!(any_expression_parens) |
        complete!(literal) |
        complete!(struct_access) |
        complete!(identifier) |
        complete!(create_struct_instance) | // consider making a combinator without this one, only assignment cares about this.
        complete!(array_access)
    )
);

named!(pub expression_or_literal_or_identifier_or_assignment<Ast>,
    alt!(
        any_expression_parens |
        literal |
        struct_access |
        create_struct_instance |
        identifier |
        assignment
    )
);
