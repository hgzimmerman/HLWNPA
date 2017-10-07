use ast::Ast;
use lang_result::LangError;
use s_expression::SExpression;
use std::collections::HashMap;

pub type MutabilityMap = HashMap<String, Mutability>;

#[derive(Debug, Clone)]
pub enum Mutability {
    Mutable,
    Immutable
}

#[derive(Debug, Clone)]
pub enum MutabilityError {
    CanNotAssignToConstVariable,
    CanNotRedeclareConst,
    VariableDoesNotExist,
    IsNotAVariable,
    CanNotRedeclareFunction,
    CanNotRedeclareStruct

}


impl Ast {
    pub fn check_mutability_semantics(&self, map: &mut HashMap<String, Mutability>) -> Result<(), MutabilityError> {
        match *self {
            Ast::ExpressionList( ref expressions) => {
                // TODO I would like to be able to do this, but this means that the REPL, which gets list of 1 for every line entered, will copy the map, so none of the rules are ever enforced.
                // TODO Until expression lists with one element are hoisted (replaced) to just become the single element, this will not work perfectly (different functions cannot use the same variable names with different mutability states)
                let mut cloned_map = map.clone(); // Clone the map, so you can use different mutability rules in sibling scopes.
                for expression in expressions {
                    let _ = expression.check_mutability_semantics(&mut cloned_map)?;
                }
                return Ok(())
            }
            Ast::SExpr(ref s_expression) => {
                match *s_expression {
                    SExpression::Assignment{ref identifier, ref ast} => { // a := 5
                        let resolved_id: String = match **identifier {
                            Ast::ValueIdentifier(ref id) => id.clone(),
                            _ => return Err(MutabilityError::IsNotAVariable) // Error, AST malformed, couldn't resolve the id
                        };
                        if let Some(mutablity) = map.get(&resolved_id) {
                            match *mutablity {
                                Mutability::Mutable => Ok(()),
                                Mutability::Immutable => Err(MutabilityError::CanNotAssignToConstVariable) // tried to assign a value to immutable value
                            }
                        } else {
                            Err(MutabilityError::VariableDoesNotExist) // variable doesn't exist yet
                        }
                    }
                    SExpression::ConstDeclaration {ref identifier, ref ast} => { // const a := 5
                        let resolved_id: String = match **identifier {
                            Ast::ValueIdentifier(ref id) => id.clone(),
                            _ => return Err(MutabilityError::IsNotAVariable) // Error, AST malformed, couldn't resolve the id
                        };
                        if let Some(_) = map.get(&resolved_id) {
                            Err(MutabilityError::CanNotRedeclareConst) // tried to assign a value to immutable value
                        } else {
                            map.insert(resolved_id, Mutability::Immutable); // prevent reassignment of the fn
                            Ok(())
                        }
                    },
                    SExpression::VariableDeclaration { ref identifier, ref ast } => {
                        // let a := 5
                        let resolved_id: String = match **identifier {
                            Ast::ValueIdentifier(ref id) => id.clone(),
                            _ => return Err(MutabilityError::IsNotAVariable) // Error, AST malformed, couldn't resolve the id
                        };
                        {
                            if let Some(mutability) = map.get(&resolved_id) {
                                match *mutability {
                                    Mutability::Mutable => return Ok(()), // You are allowed to reassign other let variables, although there isn't really a reason to.
                                    Mutability::Immutable => return Err(MutabilityError::CanNotRedeclareConst) // tried to assign a value to immutable value
                                }
                            }
                        }
                        map.insert(resolved_id, Mutability::Immutable); // prevent reassignment of the fn
                        Ok(())
                    }
                    SExpression::CreateFunction { ref identifier, ref function_datatype } => {
                        let resolved_id: String = match **identifier {
                            Ast::ValueIdentifier(ref id) => id.clone(),
                            _ => return Err(MutabilityError::IsNotAVariable) // Error, AST malformed, couldn't resolve the id
                        };
                        if let Some(_) = map.get(&resolved_id) {
                            Err(MutabilityError::CanNotRedeclareFunction) // can't reassign functions
                        } else {
                            map.insert(resolved_id, Mutability::Immutable); // prevent reassignment of the fn
                            Ok(())
                        }
                    },
                    SExpression::StructDeclaration { ref identifier, ref struct_type_info} => {
                        let resolved_id: String = match **identifier {
                            Ast::ValueIdentifier(ref id) => id.clone(),
                            _ => return Err(MutabilityError::IsNotAVariable) // Error, AST malformed, couldn't resolve the id
                        };
                        if let Some(_) = map.get(&resolved_id) {
                            Err(MutabilityError::CanNotRedeclareStruct) // can't reassign struct type
                        } else {
                            map.insert(resolved_id, Mutability::Immutable); // prevent reassignment of the struct
                            Ok(())
                        }
                    }

                    _ => {
                        Ok(()) // if the expression doesn't add anything to the variable store, we don't care about it.
                    }
                }
            }
            _ => Ok(())
        }
    }
}