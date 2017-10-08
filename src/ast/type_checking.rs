use ast::abstract_syntax_tree::Ast;
use ast::type_info::TypeInfo;
use std::collections::HashMap;
use ast::s_expression::SExpression;

pub enum TypeError {
    TypeMismatch,
    UnsupportedOperation,
    LhsNotAnIdentifier,
    IdentifierDoesntExist(String),
    MalformedAST
}

pub type TypeResult = Result<TypeInfo, TypeError>;
type TypeStore = HashMap<String, TypeInfo>;

impl Ast {
    fn check_types( &self, mut type_store: &mut TypeStore ) -> Result<TypeInfo, TypeError> {
        match *self {
            Ast::SExpr(ref sexpr) => {
                match *sexpr {
                    SExpression::Add(ref lhs, ref rhs) => {
                        TypeInfo::from(lhs.check_types(type_store)?) + TypeInfo::from(rhs.check_types(type_store)?)
                    }
                    SExpression::Subtract(ref lhs, ref rhs) => {
                        TypeInfo::from(lhs.check_types(type_store)?) - TypeInfo::from(rhs.check_types(type_store)?)
                    }
                    SExpression::Multiply(ref lhs, ref rhs) => {
                        TypeInfo::from(lhs.check_types(type_store)?) * TypeInfo::from(rhs.check_types(type_store)?)
                    }
                    SExpression::Divide(ref lhs, ref rhs) => {
                        lhs.check_types(type_store)? / TypeInfo::from(rhs.check_types(type_store)?)
                    }
                    SExpression::Modulo(ref lhs, ref rhs) => {
                        lhs.check_types(type_store)? % rhs.check_types(type_store)?
                    }
                    SExpression::Equals(_, _) => {
                        Ok(TypeInfo::Bool)
                    }
                    SExpression::NotEquals(_, _) => {
                        Ok(TypeInfo::Bool)
                    }
                    SExpression::GreaterThan(_, _) => {
                        Ok(TypeInfo::Bool)
                    }
                    SExpression::LessThan(_, _) => {
                        Ok(TypeInfo::Bool)
                    }
                    SExpression::GreaterThanOrEqual(_, _) => {
                        Ok(TypeInfo::Bool)
                    }
                    SExpression::LessThanOrEqual(_, _ ) => {
                        Ok(TypeInfo::Bool)
                    }
                    SExpression::LogicalAnd(_, _) => {
                        Ok(TypeInfo::Bool)
                    }
                    SExpression::LogicalOr(_, _) => {
                        Ok(TypeInfo::Bool)
                    }
                    // TODO, consider moving mutability into this checker? I believe it can be done.
                    SExpression::VariableDeclaration {
                        identifier: ref lhs,
                        ast: ref rhs,
                    } |
                    SExpression::ConstDeclaration {
                        identifier: ref lhs,
                        ast: ref rhs,
                    } |
                    SExpression::TypeAssignment {
                        identifier: ref lhs,
                        type_info: ref rhs,
                    } |
                    SExpression::FieldAssignment {
                        identifier: ref lhs,
                        ast: ref rhs,
                    } |
                    SExpression::DeclareFunction {
                        identifier: ref lhs,
                        function_datatype: ref rhs,
                    } => {
                        let rhs_type = rhs.check_types(type_store)?;
                        if let Ast::ValueIdentifier(ref ident) = ** lhs {
                            type_store.insert(ident.clone(), rhs_type.clone());
                            Ok(rhs_type)
                        } else {
                            Err(TypeError::LhsNotAnIdentifier)
                        }
                    }

                    SExpression::Assignment {
                        ref identifier,
                        ref ast
                    } => {
                        let rhs_type = ast.check_types(type_store)?;
                        if let Ast::ValueIdentifier(ref ident) = **identifier {
                            match type_store.get(ident) {
                                Some(lhs_type) => {
                                    //TODO not implemented in full yet
                                    if lhs_type == &rhs_type {
                                        return Ok(rhs_type)
                                    } else {
                                        return Err(TypeError::TypeMismatch)
                                    }
                                }
                                None => {
                                    return Err(TypeError::IdentifierDoesntExist(ident.clone()))
                                }
                            }
                            type_store.insert(ident.clone(), rhs_type.clone());
                            Ok(rhs_type)
                        } else {
                            return Err(TypeError::LhsNotAnIdentifier)
                        }
                    }
                    SExpression::Loop {
                        ref conditional,
                        ref body,
                    } => {
                        let _ = conditional.check_types(type_store)?;
                        body.check_types(type_store)
                    }
                    SExpression::AccessArray {
                        ref identifier,
                        ref index
                    } => {
                        if let Ast::ValueIdentifier(ref ident) = **identifier {
                            match type_store.get(ident) {
                                Some(lhs_type) => {
                                    //TODO NEED AN _ANY_ TYPE instead of typeinfo :: number, currently only matches arrays of Numbers
                                    if lhs_type == &TypeInfo::Array(Box::new(TypeInfo::Number)) { // TODO ANY type
                                        return Ok(lhs_type.clone()) // The lhs will give a specific Array type, ie. Array<Number> vs the "rhs" in this case which is just Array<Any>
                                    } else {
                                        return Err(TypeError::TypeMismatch)
                                    }
                                }
                                None => {
                                    return Err(TypeError::IdentifierDoesntExist(ident.clone()))
                                }
                            }
                        } else {
                            return Err(TypeError::LhsNotAnIdentifier)
                        }
                    }
                    SExpression::GetArrayLength(_) => {
                        Ok(TypeInfo::Number)
                    }
                    SExpression::Range { start: ref _start, end: ref _end} => {
                        Ok(TypeInfo::Array(Box::new(TypeInfo::Number)))
                    }
                    SExpression::ExecuteFn {
                        ref identifier,
                        ref parameters
                    } => {
                        let parameter_types: Vec<TypeInfo> = match **parameters {
                            Ast::ExpressionList(ref expressions) => {
                                let mut evaluated_expressions: Vec<TypeInfo> = vec![];
                                for e in expressions {
                                    match e.check_types(&mut type_store) {
                                        Ok(dt) => evaluated_expressions.push(dt),
                                        Err(err) => return Err(err),
                                    }
                                }
                                evaluated_expressions
                            }
                            _ => return Err(TypeError::MalformedAST)
                        };
                        unimplemented!()


                    }
                    _ => unimplemented!()
                }

            }
            _ => unimplemented!()
        }

    }
}