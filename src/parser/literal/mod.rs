pub mod number; // TODO move number_raw to other file/module so this can remove the "pub"
use self::number::number_literal;

mod string;
use self::string::string_literal;

mod boolean;
use self::boolean::bool_literal;

mod array;
use self::array::array_literal;

#[allow(unused_imports)]
use nom::*;
use ast::Ast;

/// put all literal types here
named!(pub literal<Ast>,
    alt!(
        complete!(array_literal) |
        complete!(number_literal) |
        complete!(string_literal) |
        complete!(bool_literal)
    )
);
