use ast::datatype::Datatype;
use std::cmp::Ordering;
use std::collections::HashMap;
use ast::type_checking::{TypeResult, TypeError};
use ast::Ast;
use ast::s_expression::SExpression;

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
    Function{parameters: Vec<TypeInfo>, return_type: Box<TypeInfo>}, // TODO, this needs to encode the parameters and the return type, instead of just the return type.
    Struct { map: HashMap<String, TypeInfo> },
    StructType{identifier: String},
    Any
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
                parameters,
                body: _body,
                return_type
            } => {
                if let Ast::ExpressionList(list) = *parameters {
                    TypeInfo::Function{
                        parameters: list
                           .iter()
                           .map(|x| {
                               if let Ast::SExpr(ref s_expression) = *x {
                                   if let SExpression::TypeAssignment{ref identifier, ref type_info} = *s_expression {
                                       match **type_info {
                                           Ast::Type(ref t_i) => {
                                               TypeInfo::from(t_i.clone())
                                           }
                                           _ => panic!("Malformed AST")
                                       }
                                   } else { panic!("Malformed AST")}
                               } else { panic!("Malformed AST")}
                           })
                           .collect(),
                        return_type: Box::new(return_type)
                    }
                } else { panic!("Malformed AST")}
            },
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
            TypeInfo::Function{ref parameters, ref return_type} => {
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
            TypeInfo::Any => {
                Some(Ordering::Equal)
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
                    TypeInfo::Any => return Ok(TypeInfo::Number),
                    _ => return Err(TypeError::UnsupportedOperation(self, other)),
                }
            }
            TypeInfo::Float => {
                match other {
                    TypeInfo::Number=> return Ok(TypeInfo::Float),
                    TypeInfo::String => {
                        return Ok(TypeInfo::String); // add the string to the number.
                    }
                    TypeInfo::Float => return Ok(TypeInfo::Float),
                    TypeInfo::Any => return Ok(TypeInfo::Float),
                    _ => return Err(TypeError::UnsupportedOperation(self, other)),
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
                    TypeInfo::Any => return Ok(TypeInfo::String),
                    _ => return Err(TypeError::UnsupportedOperation(self, other)),
                }
            }
            TypeInfo::Any => {
                Ok(other) // TODO confirm if this is what I want.
            }
            _ => return Err(TypeError::UnsupportedOperation(self, other)),
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
                    _ => Err(TypeError::UnsupportedOperation(self, other)),
                }
            }
            TypeInfo::Float => {
                match other {
                    TypeInfo::Number => return Ok(TypeInfo::Float),
                    TypeInfo::Float => return Ok(TypeInfo::Float),
                    _ => Err(TypeError::UnsupportedOperation(self, other)),
                }
            }
            TypeInfo::Any => {
                Ok(other)
            }
            _ => Err(TypeError::UnsupportedOperation(self, other)),
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
                    _ => Err(TypeError::UnsupportedOperation(self, other)),
                }
            }
            TypeInfo::Float => {
                match other {
                    TypeInfo::Number => return Ok(TypeInfo::Float),
                    TypeInfo::Float => return Ok(TypeInfo::Float),
                    _ => Err(TypeError::UnsupportedOperation(self, other)),
                }
            }
            TypeInfo::Any => {
                Ok(other)
            }
            _ => Err(TypeError::UnsupportedOperation(self, other)),
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
                    _ => Err(TypeError::UnsupportedOperation(self, other)),
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
                    _ => Err(TypeError::UnsupportedOperation(self, other)),
                }
            }
            _ => Err(TypeError::UnsupportedOperation(self, other)),
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
                    _ => Err(TypeError::UnsupportedOperation(self, other)),
                }
            }
            _ => Err(TypeError::UnsupportedOperation(self, other)),
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

    #[test]
    fn convert_fn_to_type() {
        let function = Datatype::Function{
            parameters: Box::new(Ast::ExpressionList(vec![])),
            body: Box::new(Ast::ExpressionList(vec![])),
            return_type: TypeInfo::Number
        };
        assert_eq!(TypeInfo::Function {parameters: vec![], return_type: Box::new(TypeInfo::Number)}, TypeInfo::from(function))
    }

    #[test]
    fn convert_parameter_fn_to_type() {
        use ast;
        let function = Datatype::Function{
            parameters: Box::new(Ast::ExpressionList(vec![
                Ast::SExpr(SExpression::TypeAssignment {
                    identifier: Box::new(Ast::ValueIdentifier("a".to_string())),
                    type_info: Box::new(Ast::Type(TypeInfo::Number))
                }),
                Ast::SExpr(SExpression::TypeAssignment {
                    identifier: Box::new(Ast::ValueIdentifier("b".to_string())),
                    type_info: Box::new(Ast::Type(TypeInfo::String))
                }),

            ])),
            body: Box::new(Ast::ExpressionList(vec![])),
            return_type: TypeInfo::Number
        };
        assert_eq!(TypeInfo::Function {parameters: vec![ TypeInfo::Number, TypeInfo::String ], return_type: Box::new(TypeInfo::Number)}, TypeInfo::from(function))
    }


}