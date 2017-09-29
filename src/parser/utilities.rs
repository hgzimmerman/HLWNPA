#[allow(unused_imports)]
use nom::*;
use ast::Ast;

use parser::literal::literal;
use parser::identifier::identifier;
use parser::assignment::assignment;
use parser::structure::{create_struct_instance, struct_access};
use parser::function::function_execution;
use parser::control_flow::control_flow;
use parser::expressions::{sexpr, sexpr_parens};


named!(pub expression_or_literal_or_identifier_or_struct_or_array<Ast>,
    alt!(
        complete!(sexpr) |
        complete!(create_struct_instance) // consider making a combinator without this one, only assignment cares about this.
    )
);


named!(pub expression_or_literal_or_identifier_or_assignment<Ast>,
    alt!(
        complete!(sexpr) |
        create_struct_instance |
        control_flow |
        assignment
    )
);

/// Any token that cannot directly recurse into itself (ie contain an expression as its first token)
/// nor contains a keyword.
///
/// This is used in the sexpr parser, as anything that could parse an expression could blow up the
/// stack, and that parser isn't interested in evaluating assignments, function definitions, etc...
named!(pub no_keyword_token_group <Ast>,
    alt!(
        complete!(literal) |
        complete!(struct_access) | // TODO just like array_access, struct_access and function_execution can be directly rolled into sexpr.
        complete!(function_execution) |
        complete!(identifier) |
        complete!(sexpr_parens)
    )
);
