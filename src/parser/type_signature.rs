use nom::*;
use ast::Ast;
use datatype::TypeInfo;

/// _ts indicates that the parser combinator is a getting a type signature
named!(pub type_signature<Ast>,
   ws!(alt!(number_ts | string_ts | bool_ts ))
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