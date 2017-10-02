#[allow(unused_imports)]
use nom::*;
use ast::Ast;
use datatype::TypeInfo;

/// _ts indicates that the parser combinator is a getting a type signature
named!(pub type_signature<Ast>,
   ws!(alt!(number_ts | string_ts | bool_ts | array_ts ))
);

named!(number_ts<Ast>,
    value!(
       Ast::Type( TypeInfo::Number),
       tag!("Number")
    )
);
named!(string_ts<Ast>,
    value!(
        Ast::Type( TypeInfo::String),
        tag!("String")
    )
);
named!(bool_ts<Ast>,
    value!(
        Ast::Type(TypeInfo::Bool),
        tag!("Bool")
    )
);

named!(array_ts<Ast>,
    do_parse!(
        contained_type: delimited!(
            char!('['),
            type_signature, // TODO find a way to support custom types
            char!(']')
        ) >>
        (Ast::Type(TypeInfo::Array(Box::new( get_type_from_ast(contained_type)) )))
    )
);

/// From an AST extract the type info.
/// Can panic.
fn get_type_from_ast(ast: Ast) -> TypeInfo {
    match ast {
        Ast::Type(info) => info,
        _ => panic!("Tried to get type from non-type")
    }
}