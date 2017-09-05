use ast::Ast;
#[allow(unused_imports)]
use nom::*;
use datatype::{Datatype, TypeInfo};
use super::literal;

pub const TYPE_MISMATCH_ERROR: u32 = 10001;


/// Grab a list of literals delimited by [ ] and then check if their datatypes are the same
named!(pub array_literal<Ast>,
    do_parse!(
        array: delimited!(
            ws!(char!('[')),
            array_values,
            ws!(char!(']'))
        ) >>
        (
            Ast::Literal( Datatype::Array {
                value: array.iter().map(|arr_member| match arr_member {
                    &Ast::Literal(ref datatype) => datatype.clone(), // get the datatype out of a borrowed context
                    _ => unreachable!() // Because the literal function defined in literal/mod.rs only returns literals, we know that this is unreachable.
                }).collect(),

                type_: match array.iter().map(|arr_member| match arr_member {
                    &Ast::Literal(ref datatype) => return TypeInfo::from(datatype.clone()),// get the datatype out of a borrowed context return IResult::Error(ErrorKind::Custom(420)),
                    _ => unreachable!()
                }).fold(
                    // initial value, this will become 'acc'
                    Ok( TypeInfo::from(match array[0].clone()   {
                        Ast::Literal(ref datatype) => datatype.clone(), // get the datatype out of a borrowed context
                        _ => unreachable!() //
                    })),
                    // check if each element is the same
                    |acc, x| if Ok(x) == acc {
                        acc
                    } else {
                        Err("types don't match")
                    }
                ) {
                    Ok(k) => k,
                    Err(_) => return IResult::Error(ErrorKind::Custom(TYPE_MISMATCH_ERROR))
                }
            })
        )
    )
);

named!(array_values<Vec<Ast> >,
  separated_list_complete!(
    ws!(tag!(",")),
    literal
  )
);

#[test]
fn parse_array_bool_literal_test() {
    let (_, value) = match array_literal(b"[true]") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Literal ( Datatype::Array{value: vec![Datatype::Bool(true)], type_: TypeInfo::Bool}), value)
}

#[test]
fn parse_array_multiple_bool_literal_test() {
    let (_, value) = match array_literal(b"[true, true, false]") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Literal ( Datatype::Array{value: vec![Datatype::Bool(true), Datatype::Bool(true), Datatype::Bool(false)], type_: TypeInfo::Bool}), value)
}


#[test]
fn parse_array_bool_test() {
    let (_, value) = match array_values(b"true, true") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(vec![Ast::Literal(Datatype::Bool(true)),Ast::Literal(Datatype::Bool(true))], value)
}


#[test]
fn fail_parse_array_mismatched_literal_test() {
    let error = array_literal(b"[true, 8]");
    assert_eq!(IResult::Error(ErrorKind::Custom(TYPE_MISMATCH_ERROR)), error );
}

#[test]
fn parse_array_multiple_number_literal_test() {
    let (_, value) = match array_literal(b"[12, 13, 14]") {
        IResult::Done(r, v) => (r, v),
        IResult::Error(e) => panic!("{:?}", e),
        _ => panic!(),
    };
    assert_eq!(Ast::Literal ( Datatype::Array{value: vec![Datatype::Number(12), Datatype::Number(13), Datatype::Number(14)], type_: TypeInfo::Number}), value)
}