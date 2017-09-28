#[allow(unused_imports)]
use nom::*;
use ast::Ast;

use parser::literal::literal;
use parser::identifier::identifier;
use parser::assignment::assignment;
use parser::structure::{create_struct_instance, struct_access};
use parser::array::array_access;
use parser::function_execution;
use parser::control_flow::control_flow;
use parser::expressions::{sexpr, sexpr_parens};


//TODO this is misnamed, now that it matches other sequences, fix that
//named!(pub expression_or_literal_or_identifier<Ast>,
//    alt!(
//        complete!(sexpr) |
//        complete!(literal) |
//        complete!(struct_access) | // must come before identifier
//        complete!(create_struct_instance) |
//        complete!(identifier)
////        complete!(array_access)
//    )
//);

/// Because array_access requires something that resolves something to an array, using recursive combinators would overflow the stack.
/// So don't use this in array_access.
/// This decision prevents easy access for nested array notation like: array[30][3]
// TODO: this is misnamed, function execution is missing from the name
named!(pub expression_or_literal_or_identifier_or_struct_or_array<Ast>,
    alt!(
        complete!(sexpr) |
        complete!(function_execution) |
        complete!(identifier) |
        complete!(create_struct_instance) | // consider making a combinator without this one, only assignment cares about this.
        complete!(array_access)
    )
);


/// When you want the literal to match before the sexpr
named!(pub literal_or_expression_identifier_or_struct_or_array<Ast>,
    alt!(

        complete!(literal) |
        complete!(sexpr) |
        complete!(sexpr_parens) |
        complete!(struct_access) |
        complete!(function_execution) |
        complete!(identifier) |
        complete!(create_struct_instance) | // consider making a combinator without this one, only assignment cares about this.
        complete!(array_access)
    )
);

named!(pub expression_or_literal_or_identifier_or_assignment<Ast>,
    alt!(
        complete!(sexpr) |
        complete!(literal) |
        complete!(struct_access) |
        create_struct_instance |
        function_execution |
        control_flow |
        identifier |
        assignment
    )
);

/// Any token that cannot recurse into itself (ie contain an expression)
/// nor contains a keyword.
///
/// This is used in the sexpr parser, as anything that could parse an expression could blow up the
/// stack, and that parser isn't interested in evaluating assignments, function definitions, etc...
named!(pub no_keyword_token_group <Ast>,
    alt!(
        complete!(literal) |
        complete!(struct_access) |
        complete!(array_access) |
        complete!(function_execution) |
        complete!(identifier) |
        complete!(sexpr_parens)
    )
);