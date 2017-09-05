mod number;
use self::number::number_literal;

mod string;
use self::string::string_literal;

mod boolean;
use self::boolean::bool_literal;

mod array;
//use self::array::array_literal;

#[allow(unused_imports)]
use nom::*;
use ast::Ast;

/// put all literal types here
named!(pub literal<Ast>,
    alt!(number_literal | string_literal | bool_literal)
);
