#[allow(unused_imports)]
use nom::*;
use ast::Ast;

use parser::expressions::any_expression_parens;
use parser::literal::literal;
use parser::identifier::identifier;
use parser::assignment::assignment;

named!(pub expression_or_literal_or_identifier<Ast>,
    alt!(any_expression_parens | literal | identifier)
);

named!(pub expression_or_literal_or_identifier_or_assignment<Ast>,
    alt!(any_expression_parens | literal | identifier | assignment)
);
