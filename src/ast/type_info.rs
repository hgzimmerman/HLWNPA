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
    Function(Box<TypeInfo>),
    Struct { map: HashMap<String, TypeInfo> },
    StructType{identifier: String},
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
            Datatype::Function {
                parameters: _parameters,
                body: _body,
                return_type
            } => TypeInfo::Function(Box::new(return_type)),
            Datatype::Struct { map } => {
                let mut type_map = HashMap::new();
                for tuple in map.into_iter() {
                    let (key, value) = tuple;
                    type_map.insert(key, TypeInfo::from(value));
                }
                TypeInfo::Struct { map: type_map }
            }
            Datatype::StructType{ identifier, type_information} => TypeInfo::StructType{ identifier: identifier }, // Generally isn't useful.
        }
    }
}


impl PartialOrd for TypeInfo {
    fn partial_cmp(&self, rhs: &TypeInfo) -> Option<Ordering> {
        match *self {
            TypeInfo::Number => {
                if let TypeInfo::Number = *rhs {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }
            TypeInfo::Float => {
                if let TypeInfo::Float = *rhs {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }

            TypeInfo::String => {
                if let TypeInfo::String = *rhs {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }
            TypeInfo::Array(ref lhs_contained_type) => {
                if let TypeInfo::Array(ref rhs_contained_type) = *rhs {
                    lhs_contained_type.partial_cmp(rhs_contained_type)
                } else {
                    None
                }
            }
            TypeInfo::Bool => {
                if let TypeInfo::Bool = *rhs {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }
            TypeInfo::None => {
                if let TypeInfo::None = *rhs {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }
            TypeInfo::Function(ref type_info) => {
                None // TODO, is there a better way to do this? I don't think that functions should be compared as that could require an Ast traversal, which would require loading the AST into the fn typeinfo.
            }
            TypeInfo::Struct { map: ref lhs_map } => {
                if let TypeInfo::Struct { map: ref rhs_map } = *rhs {
                    if lhs_map == rhs_map {
                        Some(Ordering::Equal)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            TypeInfo::StructType { ref identifier } => {
                if let TypeInfo::StructType { identifier: ref rhs_identifier } = *rhs {
                    identifier.partial_cmp(rhs_identifier)
                } else {
                    None
                }
            }
        }
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
                    TypeInfo::Number => return Ok(TypeInfo::Number),
                    _ => Err(TypeError::UnsupportedOperation),
                }
            }
            _ => Err(TypeError::UnsupportedOperation),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn comparison_type_info() {
        assert!(TypeInfo::Bool == TypeInfo::Bool);
        assert!(TypeInfo::Number == TypeInfo::Number);
        assert!(TypeInfo::Float == TypeInfo::Float);
        assert!(TypeInfo::String == TypeInfo::String);
        assert!(TypeInfo::Array(Box::new(TypeInfo::Number)) == TypeInfo::Array(Box::new(TypeInfo::Number)) );
        assert!(TypeInfo::Array(Box::new(TypeInfo::Number)) != TypeInfo::Array(Box::new(TypeInfo::String)) );

    }

    #[test]
    fn comparison_type_info_with_operations() {
        assert!(TypeInfo::Number == (TypeInfo::Number + TypeInfo::Number).unwrap());
        assert!(TypeInfo::Float == (TypeInfo::Float + TypeInfo::Number).unwrap());
        assert!(TypeInfo::String == (TypeInfo::String + TypeInfo::Number).unwrap());
    }

    #[test]
    fn convert_from_datatype_and_perform_operation_and_compare() {
        // TODO flesh these tests out more.
        assert!(TypeInfo::Number == (TypeInfo::from(Datatype::Number(10)) + TypeInfo::from(Datatype::Number(21))).unwrap());
    }


}