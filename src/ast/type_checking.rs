use ast::abstract_syntax_tree::Ast;
use ast::type_info::TypeInfo;
use std::collections::HashMap;
use ast::s_expression::SExpression;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    TypeMismatch(TypeInfo, TypeInfo),
    UnsupportedOperation(TypeInfo, TypeInfo),
    LhsNotAnIdentifier,
    IdentifierDoesntExist(String),
    MalformedAST,
    // Mutability
    CanNotAssignToConstVariable,
    CanNotRedeclareConst,
    VariableDoesNotExist, // Remove?
    IsNotAVariable, // Remove?
    CanNotRedeclareFunction,
    CanNotRedeclareStructType
}

use ast::datatype::Datatype;
#[derive(Debug, Clone, PartialEq)]
pub enum Mutability {
    Mutable(TypeInfo),
    Immutable(TypeInfo)
}

impl From<Datatype> for Mutability {
    fn from(datatype: Datatype) -> Mutability {
        Mutability::Mutable(TypeInfo::from(datatype))
    }
}

impl Mutability {
    fn get_type(self) -> TypeInfo {
        match self {
            Mutability::Mutable(ti) => ti,
            Mutability::Immutable(ti) => ti
        }
    }

    fn from_type_result(type_result: TypeResult) -> MutabilityResult {
        match type_result {
            Ok(ti) => Ok(Mutability::Mutable(ti)),
            Err(e) => Err(e)
        }
    }
}

pub type TypeResult = Result<TypeInfo, TypeError>;
pub type MutabilityResult = Result<Mutability, TypeError>;
pub type TypeStore = HashMap<String, Mutability>;


impl Ast {
    pub fn check_types( &self, mut type_store: &mut TypeStore ) -> MutabilityResult {
        match *self {
            Ast::SExpr(ref sexpr) => {
                match *sexpr {
                    SExpression::Add(ref lhs, ref rhs) => {
                        Mutability::from_type_result(
                            lhs.check_types(type_store)?.get_type()
                                + rhs.check_types(type_store)?.get_type()
                        )
                    }
                    SExpression::Subtract(ref lhs, ref rhs) => {
                        Mutability::from_type_result(
                            lhs.check_types(type_store)?.get_type()
                                - rhs.check_types(type_store)?.get_type()
                        )
                    }
                    SExpression::Multiply(ref lhs, ref rhs) => {
                        Mutability::from_type_result(
                            lhs.check_types(type_store)?.get_type()
                                * rhs.check_types(type_store)?.get_type()
                        )
                    }
                    SExpression::Divide(ref lhs, ref rhs) => {
                        Mutability::from_type_result(
                            lhs.check_types(type_store)?.get_type()
                                / rhs.check_types(type_store)?.get_type()
                        )
                    }
                    SExpression::Modulo(ref lhs, ref rhs) => {
                        Mutability::from_type_result(
                            lhs.check_types(type_store)?.get_type()
                                % rhs.check_types(type_store)?.get_type()
                        )
                    }
                    SExpression::Equals(_, _) => {
                        Ok(Mutability::Mutable(TypeInfo::Bool))
                    }
                    SExpression::NotEquals(_, _) => {
                        Ok(Mutability::Mutable(TypeInfo::Bool))
                    }
                    SExpression::GreaterThan(_, _) => {
                        Ok(Mutability::Mutable(TypeInfo::Bool))
                    }
                    SExpression::LessThan(_, _) => {
                        Ok(Mutability::Mutable(TypeInfo::Bool))
                    }
                    SExpression::GreaterThanOrEqual(_, _) => {
                        Ok(Mutability::Mutable(TypeInfo::Bool))
                    }
                    SExpression::LessThanOrEqual(_, _ ) => {
                        Ok(Mutability::Mutable(TypeInfo::Bool))
                    }
                    SExpression::LogicalAnd(_, _) => {
                        Ok(Mutability::Mutable(TypeInfo::Bool))
                    }
                    SExpression::LogicalOr(_, _) => {
                        Ok(Mutability::Mutable(TypeInfo::Bool))
                    }
                    // TODO, consider moving mutability into this checker? I believe it can be done.
                    SExpression::VariableDeclaration {
                        ref identifier,
                        ref ast,
                    } => {
                        let rhs_mutability: Mutability = ast.check_types(type_store)?;
                        if let Ast::ValueIdentifier(ref ident) = **identifier {
                            // hold errors that may be generated when checking types
                            let mut error: Option<TypeError> = None;

                            match type_store.get(ident) {
                                // If the variable is found, its mutability needs to be checked
                                Some(lhs_mutability) => {
                                    match *lhs_mutability {
                                        Mutability::Mutable(ref lhs_type) => {
                                            // Re declaring a variable allows it to change types
                                        }
                                        Mutability::Immutable(_) => {
                                            error = Some(TypeError::CanNotRedeclareConst)
                                        }
                                    }
                                }
                                // If the variable doesn't exist yet fall through to not return an error
                                None => {}
                            }

                            if let Some(e) = error {
                                return Err(e)
                            } else {
                                type_store.insert(ident.clone(), Mutability::Mutable(rhs_mutability.clone().get_type()));
                                Ok(rhs_mutability)
                            }


                        } else {
                            Err(TypeError::LhsNotAnIdentifier)
                        }
                    }

                    SExpression::ConstDeclaration {
                        ref identifier,
                        ref ast,
                    } => {
                        let rhs_mutability: Mutability = ast.check_types(type_store)?;
                        if let Ast::ValueIdentifier(ref ident) = **identifier {
                            // hold errors that may be generated when checking types
                            let mut error: Option<TypeError> = None;

                            match type_store.get(ident) {
                                // If the variable is found, its mutability needs to be checked
                                Some(lhs_mutability) => {
                                    error = Some(TypeError::CanNotRedeclareConst)
                                }
                                // If the variable doesn't exist yet fall through to not return an error
                                None => {}
                            }

                            if let Some(e) = error {
                                return Err(e)
                            } else {
                                type_store.insert(ident.clone(), Mutability::Immutable(rhs_mutability.clone().get_type()));
                                Ok(rhs_mutability)
                            }

                        } else {
                            Err(TypeError::LhsNotAnIdentifier)
                        }
                    }

                    SExpression::DeclareFunction {
                        ref identifier,
                        ref function_datatype,
                    } => {
                        let rhs_mutability: Mutability = function_datatype.check_types(type_store)?;
                        // TODO, should I check if the righthand side is a function datatype???
                        if let Ast::ValueIdentifier(ref ident) = **identifier {
                            // hold errors that may be generated when checking types
                            let mut error: Option<TypeError> = None;

                            match type_store.get(ident) {
                                // If the variable is found, its mutability needs to be checked
                                Some(lhs_mutability) => {
                                    error = Some(TypeError::CanNotRedeclareFunction)
                                }
                                // If the variable doesn't exist yet fall through to not return an error
                                None => {}
                            }

                            if let Some(e) = error {
                                return Err(e)
                            } else {
                                type_store.insert(ident.clone(), Mutability::Immutable(rhs_mutability.clone().get_type()));
                                Ok(rhs_mutability)
                            }
                        } else {
                            Err(TypeError::LhsNotAnIdentifier)
                        }
                    }
                    SExpression::StructDeclaration {
                        ref identifier,
                        ref struct_type_info,
                    } => {
                        let rhs_mutability: Mutability = struct_type_info.check_types(type_store)?;
                        // TODO, should I check if the righthand side is a struct type info?
                        if let Ast::ValueIdentifier(ref ident) = **identifier {
                            // hold errors that may be generated when checking types
                            let mut error: Option<TypeError> = None;

                            match type_store.get(ident) {
                                // If the variable is found, its mutability needs to be checked
                                Some(lhs_mutability) => {
                                    error = Some(TypeError::CanNotRedeclareStructType)
                                }
                                // If the variable doesn't exist yet fall through to not return an error
                                None => {}
                            }

                            if let Some(e) = error {
                                return Err(e)
                            } else {
                                type_store.insert(ident.clone(), Mutability::Immutable(rhs_mutability.clone().get_type()));
                                Ok(rhs_mutability)
                            }
                        } else {
                            Err(TypeError::LhsNotAnIdentifier)
                        }
                    }

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
                        let rhs_mutability: Mutability = ast.check_types(type_store)?;
                        if let Ast::ValueIdentifier(ref ident) = **identifier {

                            // hold errors that may be generated when checking types
                            let mut error: Option<TypeError> = None;

                            match type_store.get(ident) {
                                Some(lhs_mutability) => {
                                    match *lhs_mutability {
                                        Mutability::Mutable(_) => {
                                            if lhs_mutability.clone().get_type() != rhs_mutability.clone().get_type() {
                                                error = Some(TypeError::TypeMismatch(lhs_mutability.clone().get_type(), rhs_mutability.clone().get_type()))
                                            }
                                        }
                                        Mutability::Immutable(_) => {
                                            error = Some(TypeError::CanNotAssignToConstVariable)
                                        }
                                    }
                                }
                                None => {
                                    error = Some(TypeError::IdentifierDoesntExist(ident.clone()))
                                }
                            }

                            if let Some(e) = error {
                                return Err(e)
                            } else {
                                type_store.insert(ident.clone(), Mutability::Mutable(rhs_mutability.clone().get_type()));
                                Ok(rhs_mutability)
                            }
                        } else {
                            return Err(TypeError::LhsNotAnIdentifier)
                        }
                    }
                    SExpression::Loop {
                        ref conditional,
                        ref body,
                    } => {
                        let _ = conditional.check_types(type_store)?; // Possibly return an error on checking the conditional's type.
                        body.check_types(type_store)
                    }
                    SExpression::AccessArray {
                        ref identifier,
                        ref index
                    } => {
                        if let Ast::ValueIdentifier(ref ident) = **identifier {
                            match type_store.get(ident) {
                                Some(lhs_type) => {
                                    if lhs_type.clone().get_type() == TypeInfo::Array(Box::new(TypeInfo::Any)) {
                                        return Ok(lhs_type.clone()) // The lhs will give a specific Array type, ie. Array<Number> vs the "rhs" in this case which is just Array<Any>
                                    } else {
                                        return Err(TypeError::TypeMismatch(lhs_type.clone().get_type(), TypeInfo::Array(Box::new(TypeInfo::Any)) ))
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
                        Ok(Mutability::Mutable(TypeInfo::Number))
                    }
                    SExpression::Range { start: ref _start, end: ref _end} => {
                        Ok(Mutability::Mutable(TypeInfo::Array(Box::new(TypeInfo::Number))))
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
                                        Ok(dt) => evaluated_expressions.push(dt.get_type()),
                                        Err(err) => return Err(err),
                                    }
                                }
                                evaluated_expressions
                            }
                            _ => return Err(TypeError::MalformedAST)
                        };

                        if let Ast::ValueIdentifier(ref id) = **identifier {
                            if let Some(ref possible_fn_datatype) = type_store.get(id) {
                                if let TypeInfo::Function { ref parameters, ref return_type } = (*possible_fn_datatype).clone().get_type() {
                                    let parameter_matches: Vec<TypeResult> = parameter_types
                                        .iter()
                                        .zip( parameters.iter() )
                                        .map( |(input_type, expected_type)| {
                                            if input_type == expected_type {
                                                Ok(input_type.clone())
                                            } else {
                                                return Err(TypeError::TypeMismatch(input_type.clone(), expected_type.clone()))
                                            }
                                        } ).collect();

                                        for e in parameter_matches {
                                            if let Err(type_error) = e {
                                                return Err(type_error)
                                            }
                                        }

                                        return Ok(Mutability::Mutable(*return_type.clone()))
                                } {
                                    Err(TypeError::MalformedAST)
                                }
                            } else {
                                Err(TypeError::IdentifierDoesntExist(id.clone()))
                            }
                        } else {
                            Err(TypeError::IsNotAVariable)
                        }
                    }

                    SExpression::CreateStruct {
                        ref identifier,
                        ref struct_datatype
                    } => {
                        unimplemented!()
                    }
                    SExpression::AccessStructField {
                        ref identifier,
                        ref field_identifier
                    } => {
                        unimplemented!()
                    }


                    SExpression::Print(_) => {
                        return Ok(Mutability::Mutable(TypeInfo::String))
                    }
                    SExpression::Include(_) => {
                        Ok(Mutability::Mutable(TypeInfo::Any)) // TODO Verify what the include operator returns, consider a No-return type
                    }
                    SExpression::Invert(ref parameter) => {
                        parameter.check_types(type_store)
                    }
                    SExpression::Negate(ref parameter) => {
                        parameter.check_types(type_store)
                    }
                    SExpression::Increment(ref parameter) => {
                        parameter.check_types(type_store)
                    }
                    SExpression::Decrement(ref parameter) => {
                        parameter.check_types(type_store)
                    }

                }
            }
            Ast::Literal(ref datatype) => {
                Ok(Mutability::Mutable(TypeInfo::from( datatype.clone() )))
            }
            Ast::ValueIdentifier(ref identifier) => {
                // if the typestore has the value
                if let Some(stored_mutability_and_type) = type_store.get(identifier) {
                    Ok(stored_mutability_and_type.clone())
                } else {
                    return Ok(Mutability::Mutable(TypeInfo::Any)); // Hasn't been initialized
                }
            }
            Ast::ExpressionList(ref expressions) => {
                let mut checked_type: Mutability = Mutability::Mutable(TypeInfo::Any);
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


        assert_eq!(TypeError::TypeMismatch(TypeInfo::Number, TypeInfo::String), ast.check_types(&mut map).unwrap_err() as TypeError);
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

        assert_eq!(TypeInfo::String, ast.check_types(&mut map).unwrap().get_type());
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

        assert_eq!(TypeError::TypeMismatch(TypeInfo::Number, TypeInfo::String), ast.check_types(&mut map).unwrap_err());
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

        assert_eq!(TypeError::TypeMismatch(TypeInfo::Number, TypeInfo::String), ast.check_types(&mut map).unwrap_err());
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

        assert_eq!(TypeInfo::Number, ast.check_types(&mut map).unwrap().get_type());
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

        assert_eq!(TypeInfo::String, ast.check_types(&mut map).unwrap().get_type());
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

        assert_eq!(TypeInfo::String, ast.check_types(&mut map).unwrap().get_type());
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

        assert_eq!(TypeInfo::Float, ast.check_types(&mut map).unwrap().get_type());
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

        assert_eq!(TypeInfo::Array(Box::new(TypeInfo::Number)), ast.check_types(&mut map).unwrap().get_type());
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

        assert_eq!(TypeError::TypeMismatch(TypeInfo::Array(Box::new(TypeInfo::Number)), TypeInfo::Array(Box::new(TypeInfo::String))),
                   ast.check_types(&mut map).unwrap_err());
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

        assert_eq!(TypeError::UnsupportedOperation(TypeInfo::String, TypeInfo::Number), ast.check_types(&mut map).unwrap_err());
    }


    #[test]
    fn throw_error_on_function_execution() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
        fn my_function() -> Number {
            5
        }
        let a := "Hello"
        a := my_function()
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeError::TypeMismatch(TypeInfo::String, TypeInfo::Number), ast.check_types(&mut map).unwrap_err());
    }

    #[test]
    fn type_check_function_execution() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
        fn my_function() -> Number {
            5
        }
        let a := 7
        a := my_function()
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeInfo::Number, ast.check_types(&mut map).unwrap().get_type());
    }

    #[test]
    fn mutability_const_redeclaration_throws_error() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
            const a := 5
            let a := 4
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeError::CanNotRedeclareConst, ast.check_types(&mut map).unwrap_err());
    }

    #[test]
    fn mutability_const_reassignment_throws_error() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
            const a := 5
            a := 4
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeError::CanNotAssignToConstVariable, ast.check_types(&mut map).unwrap_err());
    }

    #[test]
    fn mutability_function_reassignment_throws_error() {
        let mut map: TypeStore = TypeStore::new();
        let input_string = r##"
            fn a() -> Number { 7 }
            a := 4
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(TypeError::CanNotAssignToConstVariable, ast.check_types(&mut map).unwrap_err());
    }

}