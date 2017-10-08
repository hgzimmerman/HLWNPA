#[allow(unused_imports)]
use nom::*;
use ast::{Ast, TypeInfo};
use parser::identifier::identifier;

/// _ts indicates that the parser combinator is a getting a type signature
named!(pub type_signature<TypeInfo>,
   ws!(alt!(number_ts | string_ts | bool_ts | array_ts | custom_ts ))
);

named!(number_ts<TypeInfo>,
    value!(
       TypeInfo::Number,
       tag!("Number")
    )
);
named!(string_ts<TypeInfo>,
    value!(
        TypeInfo::String,
        tag!("String")
    )
);
named!(bool_ts<TypeInfo>,
    value!(
        TypeInfo::Bool,
        tag!("Bool")
    )
);

named!(array_ts<TypeInfo>,
    do_parse!(
        contained_type: delimited!(
            char!('['),
            type_signature, // TODO find a way to support custom types directly in the type_signature parser and datatype.
            char!(']')
        ) >>
        (TypeInfo::Array(Box::new( contained_type ) ))
    )
);

named!(custom_ts<TypeInfo>,
    do_parse!(
        id: identifier >>
        (TypeInfo::StructType{ identifier: extract_string_from_identifier(id) })
    )
);

fn extract_string_from_identifier(identifier: Ast) -> String {
    match identifier {
        Ast::ValueIdentifier(value) => value,
        _ => panic!("Parser for identifier returned something other than a ValueIdentifier.")
    }
}

/// From an AST extract the type info.
/// Can panic.
fn get_type_from_ast(ast: Ast) -> TypeInfo {
    match ast {
        Ast::Type(info) => info,
        _ => panic!("Tried to get type from non-type")
    }
}