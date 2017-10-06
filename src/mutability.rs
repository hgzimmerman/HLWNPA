use ast::Ast;
use lang_result::LangError;
use s_expression::SExpression;
use std::collections::HashMap;

pub enum Mutability {
    Mutable,
    Immutable
}

impl Ast {
    // TODO, create custom error type for mutability rules
    pub fn check_mutability_semantics(&self, map: &mut HashMap<String, Mutability>) -> Result<(), ()> {
        match *self {
            Ast::ExpressionList( ref expressions) => {
                for expression in expressions {
                    let _ = expression.check_mutability_semantics(map)?;
                }
                return Ok(())
            }
            Ast::SExpr(ref s_expression) => {
                match *s_expression {
                    SExpression::Assignment{ref identifier, ref ast} => { // a = 5
                        let resolved_id: String = match **identifier {
                            Ast::ValueIdentifier(ref id) => id.clone(),
                            _ => return Ok(()) // Error, AST malformed, couldn't resolve the id
                        };
                        if let Some(mutablity) = map.get(&resolved_id) {
                            match *mutablity {
                                Mutability::Mutable => Ok(()),
                                Mutability::Immutable => Err(()) // tried to assign a value to immutable value
                            }
                        } else {
                            Err(()) // variable doesn't exist yet
                        }
                    }
                    SExpression::ConstDeclaration {ref identifier, ref ast} => { // const a := 5
                        let resolved_id: String = match **identifier {
                            Ast::ValueIdentifier(ref id) => id.clone(),
                            _ => return Ok(()) // Error, AST malformed, couldn't resolve the id
                        };
                        if let Some(_) = map.get(&resolved_id) {
                            Err(()) // tried to assign a value to immutable value
                        } else {
                            map.insert(resolved_id, Mutability::Immutable); // prevent reassignment of the fn
                            Ok(())
                        }
                    },
                    SExpression::VariableDeclaration { ref identifier, ref ast } => {
                        // let a := 5
                        let resolved_id: String = match **identifier {
                            Ast::ValueIdentifier(ref id) => id.clone(),
                            _ => return Ok(()) // Error, AST malformed, couldn't resolve the id
                        };
                        {
                            if let Some(mutability) = map.get(&resolved_id) {
                                match *mutability {
                                    Mutability::Mutable => return Ok(()), // You are allowed to reassign other let variables, although there isn't really a reason to.
                                    Mutability::Immutable => return Err(()) // tried to assign a value to immutable value
                                }
                            }
                        }
                        map.insert(resolved_id, Mutability::Immutable); // prevent reassignment of the fn
                        Ok(())
                    }
                    SExpression::CreateFunction { ref identifier, ref function_datatype } => {
                        let resolved_id: String = match **identifier {
                            Ast::ValueIdentifier(ref id) => id.clone(),
                            _ => return Ok(()) // Error, AST malformed, couldn't resolve the id
                        };
                        if let Some(_) = map.get(&resolved_id) {
                            Err(()) // can't reassign functions
                        } else {
                            map.insert(resolved_id, Mutability::Immutable); // prevent reassignment of the fn
                            Ok(())
                        }
                    },
                    SExpression::StructDeclaration { ref identifier, ref struct_type_info} => {
                        let resolved_id: String = match **identifier {
                            Ast::ValueIdentifier(ref id) => id.clone(),
                            _ => return Ok(()) // Error, AST malformed, couldn't resolve the id
                        };
                        if let Some(_) = map.get(&resolved_id) {
                            Err(()) // can't reassign struct type
                        } else {
                            map.insert(resolved_id, Mutability::Immutable); // prevent reassignment of the struct
                            Ok(())
                        }
                    }

                    _ => {
                        Ok(()) // if the structure doesn't add anything to the variable store, we don't care about it.
                    }
                }
            }
            _ => Ok(())
        }
    }
}