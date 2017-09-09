mod if_expression;
pub use self::if_expression::if_expression;

mod while_loop;
pub use self::while_loop::while_loop;

use nom::*;
use ast::Ast;

named!(pub control_flow<Ast>,
    ws!(alt!(while_loop | if_expression))
);