use std::ops::Sub;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;

use std::cmp::PartialOrd;
use std::cmp::Ordering;

use lang_result::*;
use Ast;

use std::collections::HashMap;


#[derive(PartialEq, Debug, Clone)]
pub enum Datatype {

    Number(i32),
    String(String),
    Array {
        value: Vec<Datatype>,
        type_: TypeInfo, // the type of data allowed in the array.
    },
    Bool(bool),
    None,
    Function {
        parameters: Box<Ast>,
        body: Box<Ast>,
        return_type: Box<TypeInfo>, // this box isn't needed
    },
    Struct {
        struct_type: String,
        map: HashMap<String, (TypeInfo, Option<Datatype>)>
    }
}

impl PartialOrd for Datatype {
    fn partial_cmp(&self, rhs: &Datatype) -> Option<Ordering> {
        match *self {
            Datatype::Number(lhs) => {
                if let Datatype::Number(rhs_number) = *rhs {
                    if lhs < rhs_number {
                        Some(Ordering::Less)
                    } else if  lhs > rhs_number {
                        Some(Ordering::Greater)
                    } else {
                        Some(Ordering::Equal)
                    }
                } else {
                    None
                }
            }
            Datatype::String(ref lhs) => {
                if let Datatype::String(rhs_string) = rhs.clone() {
                    let lhs = lhs.clone();
                    if lhs < rhs_string {
                        Some(Ordering::Less)
                    } else if  lhs > rhs_string {
                        Some(Ordering::Greater)
                    } else {
                        Some(Ordering::Equal)
                    }
                } else {
                    None
                }
            }
            Datatype::Bool(lhs) => {
                if let Datatype::Bool(rhs_bool) = *rhs {
                    if lhs < rhs_bool  {
                        Some(Ordering::Less)
                    } else if lhs > rhs_bool {
                        Some(Ordering::Greater)
                    } else {
                        Some(Ordering::Equal)
                    }
                } else {
                    None
                }
            }
            //TODO lots of cloning here, this can be more efficient.
            Datatype::Array { value: ref lhs_array, type_: ref lhs_type } => {
                if let Datatype::Array {value: rhs_array, type_: rhs_type} = rhs.clone() {
                    if lhs_type.clone() == rhs_type && lhs_array.len() == rhs_array.len() {
                        let matches = lhs_array.clone().into_iter().zip(rhs_array.into_iter()).filter(|&(ref lhs, ref rhs)| lhs.clone() == rhs.clone()).count();
                        if matches == lhs_array.len() {
                            Some(Ordering::Equal)
                        } else {
                            Some (Ordering::Less)
                        }
                    } else{
                        Some(Ordering::Less)
                    }
                } else {
                    None
                }
            }
            //Datatype::Function
            //TODO More cloning than I'm comfortable with, make more efficient.
            Datatype::Struct {struct_type: ref lhs_struct_type, map: ref lhs_map } => {
                if let Datatype::Struct {struct_type: rhs_struct_type, map: rhs_map} = rhs.clone() {
                    if lhs_struct_type.clone() == rhs_struct_type {
                        for (lhs_key, lhs_value) in lhs_map.clone().into_iter() {
                            // clone the rhs value out of the rhs_map so it can be compared.
                            if rhs_map.get(&lhs_key).map(|rhs_value: &(TypeInfo,Option<Datatype>)| rhs_value.clone()) == Some(lhs_value) {
                                continue
                            } else {
                                return Some(Ordering::Less)
                            }
                        }
                        Some(Ordering::Equal)
                    } else {
                        Some(Ordering::Less)
                    }
                } else {
                    Some(Ordering::Less)
                }

            }
            _ => { None }
        }
    }
}



impl Sub for Datatype {
    type Output = LangResult;
    fn sub(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number(lhs) => {
                match other {
                    Datatype::Number(rhs) => return Ok(Datatype::Number(lhs - rhs)),
                    _ => Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            _ => Err(LangError::UnsupportedArithimaticOperation),
        }
    }
}

impl Add for Datatype {
    type Output = LangResult;
    fn add(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number(lhs) => {
                match other {
                    Datatype::Number(rhs) => return Ok(Datatype::Number(lhs + rhs)),
                    Datatype::String(rhs) => {
                        return Ok(Datatype::String(format!("{}{}", lhs, rhs))); // add the string to the number.
                    }
                    _ => return Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            Datatype::String(lhs) => {
                match other {
                    Datatype::Number(rhs) => {
                        return Ok(Datatype::String(format!("{}{}", lhs, rhs))); // add the number to the string
                    }
                    Datatype::String(rhs) => {
                        return Ok(Datatype::String(format!("{}{}", lhs, rhs))); // add the string to the string
                    }
                    _ => return Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            _ => return Err(LangError::UnsupportedArithimaticOperation),
        }
    }
}

impl Mul for Datatype {
    type Output = LangResult;
    fn mul(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number(lhs) => {
                match other {
                    Datatype::Number(rhs) => return Ok(Datatype::Number(lhs * rhs)),
                    _ => return Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            _ => return Err(LangError::UnsupportedArithimaticOperation),
        }
    }
}

impl Div for Datatype {
    type Output = LangResult;
    fn div(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number(lhs) => {
                match other {
                    Datatype::Number(rhs) => {
                        if rhs == 0 {
                            return Err(LangError::DivideByZero);
                        }
                        return Ok(Datatype::Number(lhs / rhs));
                    }
                    _ => return Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            _ => return Err(LangError::UnsupportedArithimaticOperation),
        }
    }
}

impl Rem for Datatype {
    type Output = LangResult;
    fn rem(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number(lhs) => {
                match other {
                    Datatype::Number(rhs) => return Ok(Datatype::Number(lhs % rhs)),
                    _ => return Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            _ => return Err(LangError::UnsupportedArithimaticOperation),
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum TypeInfo {
    Number,
    String,
    Array(Box<TypeInfo>),
    Bool,
    None,
    Function,
    Struct
}


impl From<Datatype> for TypeInfo {
    fn from(datatype: Datatype) -> TypeInfo {
        match datatype {
            Datatype::Number(_) => TypeInfo::Number,
            Datatype::String(_) => TypeInfo::String,
            Datatype::Array { value, type_ } => TypeInfo::Array(Box::new(type_)),
            Datatype::Bool(_) => TypeInfo::Bool,
            Datatype::None => TypeInfo::None,
            Datatype::Function { .. } => TypeInfo::Function,
            Datatype::Struct { .. } => TypeInfo::Struct
        }
    }
}


#[test]
fn datatype_equality_tests() {
    // I reimplemented PartialEq to accommodate HashMap (which doesn't implement it)
    assert_eq!(Datatype::Number(30), Datatype::Number(30));
    assert_ne!(Datatype::Number(23), Datatype::Number(30));
    assert_eq!(Datatype::String("Hello".to_string()), Datatype::String("Hello".to_string()));
    assert_eq!(Datatype::Bool(true), Datatype::Bool(true));
    assert_eq!(Datatype::Array {value: vec!(), type_: TypeInfo::Number}, Datatype::Array {value: vec!(), type_: TypeInfo::Number});
    assert_eq!(Datatype::Array {value: vec!(Datatype::Bool(true)), type_: TypeInfo::Bool}, Datatype::Array {value: vec!(Datatype::Bool(true)), type_: TypeInfo::Bool});
    assert_ne!(Datatype::Array {value: vec!(Datatype::Bool(true)), type_: TypeInfo::Bool}, Datatype::Array {value: vec!(Datatype::Bool(true), Datatype::Bool(true)), type_: TypeInfo::Number});
    assert_eq!(Datatype::Struct {struct_type: "StructName".to_string(), map: HashMap::new()}, Datatype::Struct {struct_type: "StructName".to_string(), map: HashMap::new()});

    let mut map: HashMap<String, (TypeInfo, Option<Datatype>)> = HashMap::new();
    map.insert("field".to_string(), ( TypeInfo::Bool, Some(Datatype::Bool(true)) ));
    assert_ne!(Datatype::Struct {struct_type: "StructName".to_string(), map: map.clone()}, Datatype::Struct {struct_type: "StructName".to_string(), map: HashMap::new()});

    let mut other_map: HashMap<String, (TypeInfo, Option<Datatype>)> = HashMap::new();
    other_map.insert("field".to_string(), ( TypeInfo::Bool, Some(Datatype::Bool(true)) ));
    assert_eq!(Datatype::Struct {struct_type: "StructName".to_string(), map: map}, Datatype::Struct {struct_type: "StructName".to_string(), map: other_map});
}