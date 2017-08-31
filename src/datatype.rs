use std::ops::Sub;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;

use lang_result::*;
use Ast;


#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Datatype {
    Number ( i32 ),
    String(String),
    Array {value: Vec<Datatype>, type_: Box<Datatype>}, // I'm sort of losing type safety here.
    Bool (bool),
    None,
    Function {parameters: Box<Ast>, body: Box<Ast>, output_type: Box<Datatype>},
    //Object { value: Vec<Datatype>, v_table: Vec<Ast>}
}

impl Sub for Datatype {
    type Output = LangResult;
    fn sub(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number ( lhs) => {
                match other {
                    Datatype::Number(rhs) => {
                        return Ok(Datatype::Number( lhs - rhs ) )
                    }
                    _ => Err(LangError::UnsupportedArithimaticOperation)
                }
            },
            _ => Err(LangError::UnsupportedArithimaticOperation)
        }
    }
}

impl Add for Datatype {
    type Output = LangResult;
    fn add(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number ( lhs) => {
                match other {
                    Datatype::Number(rhs) => {
                        return Ok(Datatype::Number ( lhs + rhs ))
                    },
                    Datatype::String( rhs) => {
                        return Ok(Datatype::String(format!("{}{}",lhs,rhs))) // add the string to the number.
                    },
                    _ => return Err(LangError::UnsupportedArithimaticOperation)
                }
            },
            Datatype::String(lhs) => {
                match other {
                    Datatype::Number( rhs) => {
                        return Ok(Datatype::String(format!("{}{}",lhs,rhs)) ) // add the number to the string
                    },
                    Datatype::String( rhs ) => {
                        return Ok(Datatype::String(format!("{}{}",lhs,rhs)) ) // add the string to the string
                    },
                    _ => return Err(LangError::UnsupportedArithimaticOperation)
                }
            },
            _ => {
                return Err(LangError::UnsupportedArithimaticOperation)
            }
        }
    }
}

impl Mul for Datatype {
    type Output = LangResult;
    fn mul(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number( lhs) => {
                match other {
                    Datatype::Number( rhs) => {
                        return Ok(Datatype::Number( lhs * rhs ))
                    }
                    _ => return Err(LangError::UnsupportedArithimaticOperation)
                }
            },
            _ => return Err(LangError::UnsupportedArithimaticOperation)
        }
    }
}

impl Div for Datatype {
    type Output = LangResult;
    fn div(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number( lhs) => {
                match other {
                    Datatype::Number( rhs) => {
                        if rhs == 0 {
                            return Err(LangError::DivideByZero)
                        }
                        return Ok(Datatype::Number( lhs / rhs ))
                    }
                    _ => return Err(LangError::UnsupportedArithimaticOperation)
                }
            },
            _ => return Err(LangError::UnsupportedArithimaticOperation)
        }
    }
}

impl Rem for Datatype {
    type Output = LangResult;
    fn rem(self, other: Datatype) -> LangResult {
        match self {
            Datatype::Number (lhs) => {
                match other {
                    Datatype::Number( rhs) => {
                        return Ok(Datatype::Number( lhs % rhs ) )
                    }
                    _ => return Err(LangError::UnsupportedArithimaticOperation)
                }
            },
            _ => return Err(LangError::UnsupportedArithimaticOperation)
        }
    }
}