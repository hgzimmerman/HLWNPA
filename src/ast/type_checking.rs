use ast::abstract_syntax_tree::Ast;
use ast::type_info::TypeInfo;
use std::collections::HashMap;
use ast::s_expression::SExpression;

#[derive(Debug, Clone, PartialEq)]
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
                                    if lhs_type == &TypeInfo::Array(Box::new(TypeInfo::Any)) {
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
                                let mut cloned_type_store = type_store.clone();
                                let mut evaluated_expressions: Vec<TypeInfo> = vec![];
                                for e in expressions {
                                    match e.check_types(&mut cloned_type_store) {
                                        Ok(dt) => evaluated_expressions.push(dt),
                                        Err(err) => return Err(err),
                                    }
                                }
                                evaluated_expressions
                            }
                            _ => return Err(TypeError::MalformedAST)
                        };
                        // Next, we need to use the identifier to get the function parameter types, and the function return type.
                        unimplemented!("Function")

                    }
                    _ => unimplemented!("SExpr")
                }
            }
            Ast::Literal(ref datatype) => {
                Ok(TypeInfo::from( datatype.clone() ))
            }
            Ast::ValueIdentifier(ref identifier) => {
                // if the typestore has the value
                if let Some(stored_type) = type_store.get(identifier) {
                    Ok(stored_type.clone())
                } else {
                    return Ok(TypeInfo::Any); // Hasn't been initialized
                }
            }
            Ast::ExpressionList(ref expressions) => {
                let mut checked_type: TypeInfo = TypeInfo::Any;
                for e in expressions {
                    checked_type = e.check_types(type_store)?;
                }
                Ok(checked_type)
            }
            _ => unimplemented!("AST")
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;
    use parser::program;
    use nom::IResult;

    #[test]
    fn throw_error_on_type_mismatch_assignment() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
        let a := 5
        a := "Hello"
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeError::TypeMismatch, ast.check_types(&mut map).unwrap_err());
    }

    #[test]
    /// Reassigning the variable will allow its type to change.
    fn different_type_reassignment() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
        let a := 5
        let a := "Hello"
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeInfo::String, ast.check_types(&mut map).unwrap());
    }

    #[test]
    fn throw_error_on_type_mismatch_addition_assignment() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
        let a := 5
        a := "Hello" + 5
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeError::TypeMismatch, ast.check_types(&mut map).unwrap_err());
    }

    #[test]
    fn throw_error_on_type_mismatch_self_addition_assignment() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
        let a := 5
        a := "Hello" + a
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeError::TypeMismatch, ast.check_types(&mut map).unwrap_err());
    }

    #[test]
    fn number_is_number() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
        40
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeInfo::Number, ast.check_types(&mut map).unwrap());
    }

    #[test]
    fn string_is_string() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
        "Hello"
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeInfo::String, ast.check_types(&mut map).unwrap());
    }

    #[test]
    fn assignment_is_of_type_string() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
        let a := "Hello"
        a
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeInfo::String, ast.check_types(&mut map).unwrap());
    }

    #[test]
    fn number_plus_float_plus_number_is_a_float() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
        5 + 10.0 + 2
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeInfo::Float, ast.check_types(&mut map).unwrap());
    }
    #[test]
    fn array_is_array() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
        let a := [5]
        a := [6]
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeInfo::Array(Box::new(TypeInfo::Number)), ast.check_types(&mut map).unwrap());
    }

    #[test]
    fn array_type_mismatch() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
        let a := [5]
        a := ["Hello"]
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeError::TypeMismatch, ast.check_types(&mut map).unwrap_err());
    }

    #[test]
    fn throw_error_on_unsupported_division() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
        "Hello" / 5
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeError::UnsupportedOperation, ast.check_types(&mut map).unwrap_err());
    }

}