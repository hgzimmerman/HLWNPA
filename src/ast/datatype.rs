use std::ops::Sub;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;

use std::cmp::PartialOrd;
use std::cmp::Ordering;

use lang_result::*;
use ast::abstract_syntax_tree::Ast;
use ast::type_info::TypeInfo;

use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub type RcDatatype = Rc<Datatype>;
pub type VariableStore = HashMap<String, RcDatatype>; // Simplified interface to the map used to store Datatypes behind identifiers (String).

#[derive(PartialEq, Debug, Clone)]
pub enum Datatype {
    Number(i32),
    Float(f64),
    String(String),
    Array {
        value: Vec<RcDatatype>,
        type_: TypeInfo, // the type of data allowed in the array.
    },
    Bool(bool),
    None,
    Function {
        parameters: Box<Ast>,
        body: Box<Ast>,
        return_type: TypeInfo
    },
    Struct { map: HashMap<String, Datatype> }, // Actualized struct that holds real data.
    StructType{ identifier: String, type_information: TypeInfo}, // type_information will point to a TypeInfo that is a Struct{map: HashMap<String, TypeInfo> } that encodes the types used in the sturct
}


impl fmt::Display for Datatype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Datatype::Number(ref value) => write!(f, "{}", value),
            Datatype::Float(ref value) => write!(f, "{}", value),
            Datatype::String(ref value) => write!(f, "{}", value),
            Datatype::Bool(ref value) => write!(f, "{}", value),
            Datatype::Array {
                ref value,
                ref type_,
            } => write!(f, "[{:?}]{:?}", value, type_),
            Datatype::None => write!(f, "NONE"),
            Datatype::Function {
                ref parameters,
                body: ref _body,
                ref return_type,
            } => write!(f, "{:?} -> {:?}", parameters, return_type),
            Datatype::Struct { ref map } => write!(f, "{{{:?}}}", map),
            Datatype::StructType{ ref identifier, ref type_information}  => write!(f, "{:?}: {:?}", identifier, type_information),
        }
    }
}


impl PartialOrd for Datatype {
    fn partial_cmp(&self, rhs: &Datatype) -> Option<Ordering> {
        match *self {
            Datatype::Number(lhs) => {
                if let Datatype::Number(rhs_number) = *rhs {
                    if lhs < rhs_number {
                        Some(Ordering::Less)
                    } else if lhs > rhs_number {
                        Some(Ordering::Greater)
                    } else {
                        Some(Ordering::Equal)
                    }
                } else {
                    None
                }
            }
            Datatype::Float(lhs) => {
                match *rhs {
                    Datatype::Float(rhs) => {
                        if lhs < rhs {
                            Some(Ordering::Less)
                        } else if lhs > rhs {
                            Some(Ordering::Greater)
                        } else {
                            Some(Ordering::Equal)
                        }
                    }
                    Datatype::Number(rhs) => {
                        let rhs = rhs as f64;
                        if lhs < rhs {
                            Some(Ordering::Less)
                        } else if lhs > rhs {
                            Some(Ordering::Greater)
                        } else {
                            Some(Ordering::Equal)
                        }
                    }
                    _ => None
                }
            }
            Datatype::String(ref lhs) => {
                if let &Datatype::String(ref rhs_string) = rhs {
                    if lhs < rhs_string {
                        Some(Ordering::Less)
                    } else if lhs > rhs_string {
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
                    if lhs < rhs_bool {
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
            Datatype::Array {
                value: ref lhs_array,
                type_: ref lhs_type,
            } => {
                if let &Datatype::Array {
                    value: ref rhs_array,
                    type_: ref rhs_type,
                } = rhs
                {
                    if lhs_type == rhs_type && lhs_array.len() == rhs_array.len() {
                        let matches = lhs_array
                            .into_iter()
                            .zip(rhs_array.into_iter())
                            .filter(|&(ref lhs, ref rhs)| lhs == rhs)
                            .count();
                        if matches == lhs_array.len() {
                            Some(Ordering::Equal)
                        } else {
                            Some(Ordering::Less)
                        }
                    } else {
                        Some(Ordering::Less)
                    }
                } else {
                    None
                }
            }
            //Datatype::Function
            Datatype::Struct { map: ref lhs_map } => {
                if let &Datatype::Struct { map: ref rhs_map } = rhs {
                    for (lhs_key, lhs_value) in lhs_map.into_iter() {
                        // clone the rhs value out of the rhs_map so it can be compared.
                        if rhs_map.get(lhs_key) == Some(lhs_value) {
                            continue;
                        } else {
                            return Some(Ordering::Less);
                        }
                    }
                    Some(Ordering::Equal)
                } else {
                    Some(Ordering::Less)
                }
            }
            _ => None,
        }
    }
}



impl Sub for Datatype {
    type Output = LangResult;
    fn sub(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number(lhs) => {
                match other {
                    Datatype::Number(rhs) => return Ok(Rc::new(Datatype::Number(lhs - rhs))),
                    Datatype::Float(rhs) => return Ok(Rc::new(Datatype::Float(lhs as f64 - rhs))),
                    _ => Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            Datatype::Float(lhs) => {
                match other {
                    Datatype::Number(rhs) => return Ok(Rc::new(Datatype::Float(lhs - rhs as f64))),
                    Datatype::Float(rhs) => return Ok(Rc::new(Datatype::Float(lhs - rhs))),
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
                    Datatype::Number(rhs) => return Ok(Rc::new(Datatype::Number(lhs + rhs))),
                    Datatype::String(rhs) => {
                        return Ok(Rc::new(Datatype::String(format!("{}{}", lhs, rhs)))); // add the string to the number.
                    },
                    Datatype::Float(rhs) => return Ok(Rc::new(Datatype::Float(lhs as f64 + rhs))),
                    _ => return Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            Datatype::Float(lhs) => {
                match other {
                    Datatype::Number(rhs) => return Ok(Rc::new(Datatype::Float(lhs + rhs as f64))),
                    Datatype::String(rhs) => {
                        return Ok(Rc::new(Datatype::String(format!("{}{}", lhs, rhs)))); // add the string to the number.
                    }
                    Datatype::Float(rhs) => return Ok(Rc::new(Datatype::Float(lhs + rhs))),
                    _ => return Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            Datatype::String(lhs) => {
                match other {
                    Datatype::Number(rhs) => {
                        return Ok(Rc::new(Datatype::String(format!("{}{}", lhs, rhs)))); // add the number to the string
                    }
                    Datatype::Float(rhs) => {
                        return Ok(Rc::new(Datatype::String(format!("{}{}", lhs, rhs)))); // add the number to the string
                    }
                    Datatype::String(rhs) => {
                        return Ok(Rc::new(Datatype::String(format!("{}{}", lhs, rhs)))); // add the string to the string
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
                    Datatype::Number(rhs) => return Ok(Rc::new(Datatype::Number(lhs * rhs))),
                    Datatype::Float(rhs) => return Ok(Rc::new(Datatype::Float(lhs as f64 * rhs))),
                    _ => return Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            Datatype::Float(lhs) => {
                match other {
                    Datatype::Number(rhs) => return Ok(Rc::new(Datatype::Float(lhs * rhs as f64))),
                    Datatype::Float(rhs) => return Ok(Rc::new(Datatype::Float(lhs * rhs))),
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
                        return Ok(Rc::new(Datatype::Number(lhs / rhs)));
                    }
                    Datatype::Float(rhs) => {
                        let lhs = lhs as f64;
                        if rhs == 0.0 {
                            return Err(LangError::DivideByZero);
                        }
                        return Ok(Rc::new(Datatype::Float(lhs / rhs)));
                    }
                    _ => return Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            Datatype::Float(lhs) => {
                match other {
                    Datatype::Number(rhs) => {
                        let rhs = rhs as f64;
                        if rhs == 0.0 {
                            return Err(LangError::DivideByZero);
                        }
                        return Ok(Rc::new(Datatype::Float(lhs / rhs)));
                    }
                    Datatype::Float(rhs) => {
                        if rhs == 0.0 {
                            return Err(LangError::DivideByZero);
                        }
                        return Ok(Rc::new(Datatype::Float(lhs / rhs)));
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
                    Datatype::Number(rhs) => return Ok(Rc::new(Datatype::Number(lhs % rhs))),
                    _ => return Err(LangError::UnsupportedArithimaticOperation),
                }
            }
            _ => return Err(LangError::UnsupportedArithimaticOperation),
        }
    }
}


/// I reimplemented PartialEq for Datatype to accommodate the HashMap in Struct (which doesn't implement it)
/// This test checks that I didn't break the re-implementation.
#[test]
fn datatype_equality_tests() {
    assert_eq!(Datatype::Number(30), Datatype::Number(30));
    assert_ne!(Datatype::Number(23), Datatype::Number(30));
    assert_eq!(
        Datatype::String("Hello".to_string()),
        Datatype::String("Hello".to_string())
    );
    assert_eq!(Datatype::Bool(true), Datatype::Bool(true));
    assert_ne!(Datatype::Bool(false), Datatype::Bool(true));
    assert_eq!(
        Datatype::Array {
            value: vec![],
            type_: TypeInfo::Number,
        },
        Datatype::Array {
            value: vec![],
            type_: TypeInfo::Number,
        }
    );
    assert_eq!(
        Datatype::Float(0.0),
        Datatype::Float(0.0)
    );
    assert_eq!(
        *(Datatype::Float(0.0) + Datatype::Number(1)).unwrap(),
        Datatype::Float(1.0)
    );
    assert_eq!(
        Datatype::Array {
            value: vec![Rc::new(Datatype::Bool(true))],
            type_: TypeInfo::Bool,
        },
        Datatype::Array {
            value: vec![Rc::new(Datatype::Bool(true))],
            type_: TypeInfo::Bool,
        }
    );
    assert_ne!(
        Datatype::Array {
            value: vec![Rc::new(Datatype::Bool(true))],
            type_: TypeInfo::Bool,
        },
        Datatype::Array {
            value: vec![Rc::new(Datatype::Bool(true)), Rc::new(Datatype::Bool(true))],
            type_: TypeInfo::Number,
        }
    );
    assert_eq!(
        Datatype::Struct { map: HashMap::new() },
        Datatype::Struct { map: HashMap::new() }
    );

    let mut map: HashMap<String, Datatype> = HashMap::new();
    map.insert("field".to_string(), Datatype::Bool(true));
    assert_ne!(
        Datatype::Struct { map: map.clone() },
        Datatype::Struct { map: HashMap::new() }
    );

    let mut other_map: HashMap<String, Datatype> = HashMap::new();
    other_map.insert("field".to_string(), Datatype::Bool(true));
    assert_eq!(
        Datatype::Struct { map: map },
        Datatype::Struct { map: other_map }
    );
}
