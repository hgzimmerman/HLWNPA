use ast::datatype::Datatype;
use std::cmp::Ordering;
use std::collections::HashMap;

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

//TODO, implement this. It is never used, but should be accurate
impl PartialOrd for TypeInfo {
    fn partial_cmp(&self, _rhs: &TypeInfo) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
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