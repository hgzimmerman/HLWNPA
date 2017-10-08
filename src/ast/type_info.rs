use ast::datatype::Datatype;
use std::cmp::Ordering;
use std::collections::HashMap;
use ast::type_checking::{TypeResult, TypeError};


use std::ops::Sub;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;

#[derive(PartialEq, Debug, Clone)]
pub enum TypeInfo {
    Number,
    Float,
    String,
    Array(Box<TypeInfo>),
    Bool,
    None,
    Function,
    Struct { map: HashMap<String, TypeInfo> },
    StructType,
}




impl From<Datatype> for TypeInfo {
    fn from(datatype: Datatype) -> TypeInfo {
        match datatype {
            Datatype::Number(_) => TypeInfo::Number,
            Datatype::Float(_) => TypeInfo::Float,
            Datatype::String(_) => TypeInfo::String,
            Datatype::Array {
                value: _value,
                type_,
            } => TypeInfo::Array(Box::new(type_)),
            Datatype::Bool(_) => TypeInfo::Bool,
            Datatype::None => TypeInfo::None,
            Datatype::Function { .. } => TypeInfo::Function,
            Datatype::Struct { map } => {
                let mut type_map = HashMap::new();
                for tuple in map.into_iter() {
                    let (key, value) = tuple;
                    type_map.insert(key, TypeInfo::from(value));
                }
                TypeInfo::Struct { map: type_map }
            }
            Datatype::StructType(_) => TypeInfo::StructType, // Generally isn't useful.
        }
    }
}
//TODO, implement this. It is never used, but should be accurate
impl PartialOrd for TypeInfo {
    fn partial_cmp(&self, _rhs: &TypeInfo) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl Add for TypeInfo {
    type Output = TypeResult;
    fn add(self, other: TypeInfo) -> TypeResult {
        match self {
            TypeInfo::Number => {
                match other {
                    TypeInfo::Number => return Ok(TypeInfo::Number),
                    TypeInfo::String => {
                        return Ok(TypeInfo::String)// add the string to the number.
                    },
                    TypeInfo::Float => return Ok(TypeInfo::Float),
                    _ => return Err(TypeError::UnsupportedOperation),
                }
            }
            TypeInfo::Float => {
                match other {
                    TypeInfo::Number=> return Ok(TypeInfo::Float),
                    TypeInfo::String => {
                        return Ok(TypeInfo::String); // add the string to the number.
                    }
                    TypeInfo::Float => return Ok(TypeInfo::Float),
                    _ => return Err(TypeError::UnsupportedOperation),
                }
            }
            TypeInfo::String => {
                match other {
                    TypeInfo::Number => {
                        return Ok(TypeInfo::String);
                    }
                    TypeInfo::Float => {
                        return Ok(TypeInfo::String);
                    }
                    TypeInfo::String => {
                        return Ok(TypeInfo::String);
                    }
                    _ => return Err(TypeError::UnsupportedOperation),
                }
            }
            _ => return Err(TypeError::UnsupportedOperation),
        }
    }
}



impl Sub for TypeInfo {
    type Output = TypeResult;
    fn sub(self, other: TypeInfo) -> TypeResult {
        match self {
            TypeInfo::Number => {
                match other {
                    TypeInfo::Number => return Ok(TypeInfo::Number),
                    TypeInfo::Float => return Ok(TypeInfo::Float),
                    _ => Err(TypeError::UnsupportedOperation),
                }
            }
            TypeInfo::Float => {
                match other {
                    TypeInfo::Number => return Ok(TypeInfo::Float),
                    TypeInfo::Float => return Ok(TypeInfo::Float),
                    _ => Err(TypeError::UnsupportedOperation),
                }
            }
            _ => Err(TypeError::UnsupportedOperation),
        }
    }
}

impl Mul for TypeInfo {
    type Output = TypeResult;
    fn mul(self, other: TypeInfo) -> TypeResult {
        match self {
            TypeInfo::Number => {
                match other {
                    TypeInfo::Number => return Ok(TypeInfo::Number),
                    TypeInfo::Float => return Ok(TypeInfo::Float),
                    _ => Err(TypeError::UnsupportedOperation),
                }
            }
            TypeInfo::Float => {
                match other {
                    TypeInfo::Number => return Ok(TypeInfo::Float),
                    TypeInfo::Float => return Ok(TypeInfo::Float),
                    _ => Err(TypeError::UnsupportedOperation),
                }
            }
            _ => Err(TypeError::UnsupportedOperation),
        }
    }
}

impl Div for TypeInfo {
    type Output = TypeResult;
    fn div(self, other: TypeInfo) -> TypeResult {
        match self {
            TypeInfo::Number => {
                match other {
                    TypeInfo::Number => {
                        return Ok(TypeInfo::Number);
                    }
                    TypeInfo::Float => {
                        return Ok(TypeInfo::Float);
                    }
                    _ => Err(TypeError::UnsupportedOperation),
                }
            }
            TypeInfo::Float => {
                match other {
                    TypeInfo::Number => {
                        return Ok(TypeInfo::Float);
                    }
                    TypeInfo::Float => {
                        return Ok(TypeInfo::Float);
                    }
                    _ => Err(TypeError::UnsupportedOperation),
                }
            }
            _ => Err(TypeError::UnsupportedOperation),
        }
    }
}

impl Rem for TypeInfo {
    type Output = TypeResult;
    fn rem(self, other: TypeInfo) -> TypeResult {
        match self {
            TypeInfo::Number => {
                match other {
                    TypeInfo::Number=> return Ok(TypeInfo::Number),
                    _ => Err(TypeError::UnsupportedOperation),
                }
            }
            _ => Err(TypeError::UnsupportedOperation),
        }
    }
}
