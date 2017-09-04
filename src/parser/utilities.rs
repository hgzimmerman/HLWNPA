
use nom::*;

named!(pub expression_or_literal_or_identifier<Ast>,
    alt!(any_expression_parens | literal | identifier)
);