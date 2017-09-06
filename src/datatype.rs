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
        map: HashMap<String, Datatype>
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
