use ast::abstract_syntax_tree::Ast;
use ast::type_info::TypeInfo;
use std::collections::HashMap;
use ast::s_expression::SExpression;

pub enum TypeError {
    TypeMismatch,
    UnsupportedOperation,
    LhsNotAnIdentifier,
    IdentifierDoesntExist(String)
}

pub type TypeResult = Result<TypeInfo, TypeError>;
type TypeStore = HashMap<String, TypeInfo>;

impl Ast {
    fn check_types( &self, type_store: &mut TypeStore ) -> Result<TypeInfo, TypeError> {
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
                    // TODO, consider moving mutability into this checker?
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

                    _ => unimplemented!()
                }

            }
            _ => unimplemented!()
        }

    }
}