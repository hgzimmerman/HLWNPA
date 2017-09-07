use lang_result::*;
use datatype::{Datatype, TypeInfo};
//use std::mem::discriminant;

use std::boxed::Box;
use std::collections::HashMap;



#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Ast {
    Expression {
        operator: BinaryOperator,
        expr1: Box<Ast>,
        expr2: Box<Ast>,
    },
    UnaryExpression {
        operator: UnaryOperator,
        expr: Box<Ast>,
    },
    VecExpression { expressions: Vec<Ast> }, // uesd for structuring execution of the AST.
    Conditional {
        condition: Box<Ast>,
        true_expr: Box<Ast>,
        false_expr: Option<Box<Ast>>,
    },
    Literal(Datatype), // consider making the Literal another enum with supported default datatypes.
    Type(TypeInfo), // value in the datatype is useless, just use this to determine parameter type.
    ValueIdentifier(String), // gets the value mapped to a hashmap
}


#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Assignment,
    ExecuteFn,
    FunctionParameterAssignment,
    Loop,
    AccessArray,
    StructDeclaration,
    AccessStructField,
    CreateStruct
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum UnaryOperator {
    Print,
    Invert,
    Increment,
    Decrement,
}


impl Ast {
    pub fn evaluate(&self, map: &mut HashMap<String, Datatype>) -> LangResult {
        match *self {
            Ast::Expression {
                ref operator,
                ref expr1,
                ref expr2,
            } => {
                match *operator {
                    BinaryOperator::Plus => expr1.evaluate(map)? + expr2.evaluate(map)?,
                    BinaryOperator::Minus => expr1.evaluate(map)? - expr2.evaluate(map)?,
                    BinaryOperator::Multiply => expr1.evaluate(map)? * expr2.evaluate(map)?,
                    BinaryOperator::Divide => expr1.evaluate(map)? / expr2.evaluate(map)?,
                    BinaryOperator::Modulo => expr1.evaluate(map)? % expr2.evaluate(map)?,
                    BinaryOperator::Equals => {
                        if expr1.evaluate(map)? == expr2.evaluate(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    BinaryOperator::NotEquals => {
                        if expr1.evaluate(map)? != expr2.evaluate(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    BinaryOperator::GreaterThan => {
                        if expr1.evaluate(map)? > expr2.evaluate(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    BinaryOperator::LessThan => {
                        if expr1.evaluate(map)? < expr2.evaluate(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    BinaryOperator::GreaterThanOrEqual => {
                        if expr1.evaluate(map)? >= expr2.evaluate(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    BinaryOperator::LessThanOrEqual => {
                        if expr1.evaluate(map)? <= expr2.evaluate(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    BinaryOperator::Assignment => {
                        if let Ast::ValueIdentifier(ref ident) = **expr1 {
                            let mut cloned_map = map.clone(); // since this is a clone, the required righthand expressions will be evaluated in their own 'stack', this modified hashmap will be cleaned up post assignment.
                            let evaluated_right_hand_side = expr2.evaluate(&mut cloned_map)?;
                            let cloned_evaluated_rhs = evaluated_right_hand_side.clone();
                            map.insert(ident.clone(), evaluated_right_hand_side);
                            return Ok(cloned_evaluated_rhs);
                        } else {
                            Err(LangError::IdentifierDoesntExist)
                        }
                    }
                    BinaryOperator::FunctionParameterAssignment => {
                        // does the same thing as assignment, but I want a separate type for this.
                        if let Ast::ValueIdentifier(ref ident) = **expr1 {
                            let mut cloned_map = map.clone(); // since this is a clone, the required righthand expressions will be evaluated in their own 'stack', this modified hashmap will be cleaned up post assignment.
                            let evaluated_right_hand_side = expr2.evaluate(&mut cloned_map)?;
                            let cloned_evaluated_rhs = evaluated_right_hand_side.clone();
                            map.insert(ident.clone(), evaluated_right_hand_side);
                            return Ok(cloned_evaluated_rhs);
                        } else {
                            Err(LangError::IdentifierDoesntExist)
                        }
                    }
                    BinaryOperator::Loop => {
                        let mut evaluated_loop: Datatype = Datatype::None;
                        let cloned_expr1 = *expr1.clone();
                        loop {
                            let condition: Datatype = cloned_expr1.evaluate(map)?;
                            match condition {
                                Datatype::Bool(b) => {
                                    if b {
                                        evaluated_loop = expr2.evaluate(map)?; // This doesn't clone the map, so it can mutate the "stack" of its context
                                    } else {
                                        break;
                                    }
                                }
                                _ => return Err(LangError::ConditionalNotBoolean(TypeInfo::from(condition))),
                            }
                        }
                        return Ok(evaluated_loop); // leave block
                    }
                    BinaryOperator::AccessArray => {
                        let datatype: Datatype = expr1.evaluate(map)?;
                        match datatype {
                            Datatype::Array {value, type_} => {
                                let possible_index = expr2.evaluate(map)?;
                                match possible_index {
                                    Datatype::Number(index) => {
                                        if index >= 0 {
                                            match value.get(index as usize) {
                                                Some(indexed_result) => Ok(indexed_result.clone()), // cannot mutate the interior of the array.
                                                None => Err(LangError::OutOfBoundsArrayAccess)
                                            }
                                        } else {
                                            Err(LangError::NegativeIndex(index))
                                        }
                                    }
                                    _ => Err(LangError::InvalidIndexType(possible_index))
                                }
                            }
                            _ => return Err(LangError::ArrayAccessOnNonArry(TypeInfo::from(datatype)))
                        }
                    }
                    // Add an entry for a struct type to the current stack
                    BinaryOperator::StructDeclaration => {
                        if let Ast::ValueIdentifier(ref struct_type_identifier) = **expr1 {
                            if let Ast::VecExpression {ref expressions} = **expr2 {

                                let mut struct_map: HashMap<String, TypeInfo> = HashMap::new();

                                for assignment_expr in expressions {
                                    if let &Ast::Expression {operator: ref assignment_op, expr1: ref field_identifier_expr, expr2: ref field_type_expr} = assignment_expr {
                                        if let BinaryOperator::FunctionParameterAssignment = *assignment_op {
                                            if let Ast::ValueIdentifier(ref field_id) = **field_identifier_expr {
                                                if let Ast::Type(ref field_type) = **field_type_expr {
                                                    struct_map.insert(field_id.clone(), field_type.clone());
                                                } else {
                                                    return Err(LangError::FieldTypeNotSupplied)
                                                }
                                            } else {
                                                return Err(LangError::FieldIdentifierNotSupplied)
                                            }
                                        } else {
                                            return Err(LangError::NonAssignmentInStructDeclaration)
                                        }
                                    } else {
                                        return Err(LangError::ExpectedExpression)
                                    }
                                }
                                let new_struct_type = TypeInfo::Struct {map: struct_map};
                                let retval = Datatype::StructType(new_struct_type);
                                map.insert(struct_type_identifier.clone(), retval.clone() );
                                return Ok(retval)
                            } else {
                                return Err(LangError::StructBodyNotSupplied)
                            }
                        } else {
                            Err(LangError::StructNameNotSupplied)
                        }
                    }
                    BinaryOperator::AccessStructField => {
                        // if expr1 produces a struct when evaluated
                        if let Datatype::Struct { map: struct_map } = expr1.evaluate(map)? {
                            if let Ast::ValueIdentifier(ref field_identifier) = **expr2 {
                                match struct_map.get(field_identifier) {
                                    Some(struct_field_datatype) => return Ok(struct_field_datatype.clone()),
                                    None => return Err(LangError::StructFieldDoesntExist)
                                }
                            } else {
                                return Err(LangError::IdentifierDoesntExist)
                            }
                        } else {
                            return Err(LangError::TriedToAccessNonStruct)
                        }
                    }

                    BinaryOperator::CreateStruct => {
                        // This expects the expr1 to be an Identifier that resolves to be a struct definition, or the struct definition itself.
                        if let Datatype::StructType(TypeInfo::Struct {map: struct_type_map} ) = expr1.evaluate(map)? {
                            // It expects that the righthand side should be a series of expressions that assign values to fields (that have already been specified in the StructType)
                            if let Ast::VecExpression { expressions: ref assignment_expressions } = **expr2 {
                                let mut new_struct_map: HashMap<String, Datatype> = HashMap::new();

                                for assignment_expression in assignment_expressions {
                                    if let Ast::Expression { operator: ref assignment_operator, expr1: ref assignment_expr1, expr2: ref assignment_expr2 } = *assignment_expression {
                                        if assignment_operator == &BinaryOperator::FunctionParameterAssignment { // TODO I REALLY dont like this operator's use in this context
                                            if let Ast::ValueIdentifier(ref field_identifier) = **assignment_expr1 {
                                                // Is the identifier specified in the AST exist in the struct type? check the struct_map
                                                let expected_type = match struct_type_map.get(field_identifier) {
                                                    Some(struct_type) => struct_type,
                                                    None => return Err(LangError::IdentifierDoesntExist) // todo find better error message
                                                };
                                                let value_to_be_assigned: Datatype = assignment_expr2.evaluate(map)?;

                                                // check if the value to be assigned matches the expected type
                                                let to_be_assigned_type: TypeInfo = TypeInfo::from(value_to_be_assigned.clone());
                                                if expected_type == &to_be_assigned_type {
                                                    // now add the value to the new struct's map
                                                    new_struct_map.insert(field_identifier.clone(), value_to_be_assigned);
                                                } else {
                                                    return Err(LangError::TypeError { expected: expected_type.clone(), found: to_be_assigned_type })
                                                }

                                            } else {
                                                return Err(LangError::ExpectedIdentifier);
                                            }
                                        } else {
                                            return Err(LangError::NonAssignmentInStructInit);
                                        }
                                    } else {
                                        return Err(LangError::NonAssignmentInStructInit);
                                    }
                                }
                                return Ok(Datatype::Struct{map: new_struct_map}) // Return the new struct.
                            } else {
                                return Err(LangError::StructBodyNotSupplied) // not entirely accurate
                            }
                        } else {
                            return Err(LangError::IdentifierDoesntExist) // TODO not implemented
                        }
                    }

                    BinaryOperator::ExecuteFn => {
                        let mut cloned_map = map.clone(); // clone the map, to create a temporary new "stack" for the life of the function

                        // evaluate the parameters
                        let evaluated_parameters: Vec<Datatype> = match **expr2 {
                            Ast::VecExpression { ref expressions } => {
                                let mut evaluated_expressions: Vec<Datatype> = vec![];
                                for e in expressions {
                                    match e.evaluate(&mut cloned_map) {
                                        Ok(dt) => evaluated_expressions.push(dt),
                                        Err(err) => return Err(err),
                                    }
                                }
                                evaluated_expressions
                            }
                            _ => return Err(LangError::FunctionParametersShouldBeVecExpression),
                        };


                        // Take an existing function by (by grabbing the function using an identifier, which should resolve to a function)
                        match expr1.evaluate(&mut cloned_map)? {
                            Datatype::Function {
                                parameters,
                                body,
                                return_type,
                            } => {
                                match *parameters {
                                    // The parameters should be in the form: VecExpression(expression_with_fn_assignment, expression_with_fn_assignment, ...) This way, functions should be able to support arbitrary numbers of parameters.
                                    Ast::VecExpression { expressions } => {
                                        // zip the values of the evaluated parameters into the expected parameters for the function
                                        if evaluated_parameters.len() == expressions.len() {
                                            // Replace the right hand side of the expression (which should be an Ast::Type with a computed literal.
                                            let rhs_replaced_with_evaluated_parameters_results: Vec<Result<Ast, LangError>> = expressions
                                                .iter()
                                                .zip(evaluated_parameters)
                                                .map(|expressions_with_parameters: (&Ast, Datatype)| {
                                                    let (e, d) = expressions_with_parameters; // assign out of tuple.
                                                    if let Ast::Expression {
                                                        ref operator,
                                                        ref expr1,
                                                        ref expr2,
                                                    } = *e
                                                    {
                                                        let operator = operator.clone();
                                                        let expr1 = expr1.clone();

                                                        //do run-time type-checking, the supplied value should be of the same type as the specified value
                                                        let expected_type: &TypeInfo = match **expr2 {
                                                            Ast::Type(ref datatype) => datatype,
                                                            Ast::ValueIdentifier(ref id) => {
                                                                match map.get(id) { // get what should be a struct out of the stack
                                                                    Some(datatype) => {
                                                                        if let Datatype::StructType(ref struct_type_info) = *datatype {
                                                                            struct_type_info
                                                                        } else {
                                                                            return Err(LangError::ExpectedIdentifierToBeStructType{ found: id.clone()} )
                                                                        }
                                                                    }
                                                                    None => return Err(LangError::IdentifierDoesntExist)
                                                                }
                                                            }
                                                            _ => return Err(LangError::ExpectedDataTypeInfo),
                                                        };
                                                        if expected_type != &TypeInfo::from(d.clone()) {
                                                            return Err(LangError::TypeError {
                                                                expected: expected_type.clone(),
                                                                found: TypeInfo::from(d),
                                                            });
                                                        }

                                                        if operator == BinaryOperator::FunctionParameterAssignment {
                                                            return Ok(Ast::Expression {
                                                                operator: operator,
                                                                expr1: expr1,
                                                                expr2: Box::new(Ast::Literal(d)),
                                                            }); // return a new FunctionParameterAssignment Expression with a replaced expr2.
                                                        } else {
                                                            return Err(LangError::InvalidFunctionPrototypeFormatting);
                                                        }
                                                    } else {
                                                        return Err(LangError::InvalidFunctionPrototypeFormatting);
                                                    }
                                                })
                                                .collect();

                                            // These functions _should_ all be assignments, per the parser.
                                            // So after replacing the Types with Literals that have been derived from the expressions passed in,
                                            // they can be associated with the identifiers, and the identifiers can be used in the function body later.
                                            for rhs in rhs_replaced_with_evaluated_parameters_results {
                                                let rhs = rhs?; // return the error if present
                                                rhs.evaluate(&mut cloned_map)?; // create the assignment
                                            }
                                        } else {
                                            return Err(LangError::ParameterLengthMismatch);
                                        }

                                        // Evaluate the body of the function
                                        let output: Datatype = body.evaluate(&mut cloned_map)?;
                                        let expected_return_type: TypeInfo = match *return_type {
                                            Ast::Type(type_) => type_,
                                            Ast::ValueIdentifier(ref id) => {
                                                match map.get(id) {
                                                    Some(datatype) => {
                                                        if let Datatype::StructType(ref struct_type_info) = *datatype {
                                                            struct_type_info.clone()
                                                        } else {
                                                            return Err(LangError::ExpectedIdentifierToBeStructType { found: id.clone() })
                                                        }
                                                    }
                                                    None => return Err(LangError::IdentifierDoesntExist)
                                                }
                                            }
                                            _ => {
                                               return Err(LangError::ExpectedDataTypeInfo)
                                            }
                                        };

                                        if TypeInfo::from(output.clone()) == expected_return_type {
                                            return Ok(output);
                                        } else {
                                            return Err(LangError::ReturnTypeDoesNotMatchReturnValue);
                                        }
                                    }
                                    _ => return Err(LangError::ParserShouldHaveRejected), // The parser should have put the parameters in the form VecExpression(expression_with_assignment, expression_with_assignment, ...)
                                }
                            }
                            _ => Err(LangError::ExecuteNonFunction),
                        }
                    }
                }
            }
            Ast::UnaryExpression {
                ref operator,
                ref expr,
            } => {
                match *operator {
                    UnaryOperator::Print => {
                        print!("{:?}", expr.evaluate(map)?); // todo use: std::fmt::Display::fmt instead
                        Ok(Datatype::None)
                    }
                    UnaryOperator::Invert => {
                        match expr.evaluate(map)? {
                            Datatype::Bool(bool) => Ok(Datatype::Bool(!bool)),
                            _ => Err(LangError::InvertNonBoolean),
                        }
                    }
                    UnaryOperator::Increment => {
                        match expr.evaluate(map)? {
                            Datatype::Number(number) => Ok(Datatype::Number(number + 1)),
                            _ => Err(LangError::IncrementNonNumber),
                        }
                    }
                    UnaryOperator::Decrement => {
                        match expr.evaluate(map)? {
                            Datatype::Number(number) => Ok(Datatype::Number(number - 1)),
                            _ => Err(LangError::DecrementNonNumber),
                        }
                    }
                }
            }
            //Evaluate multiple expressions and return the result of the last one.
            Ast::VecExpression { ref expressions } => {
                let mut val: Datatype = Datatype::None; // TODO, consider maxing this return an error if the expressions vector is empty
                for e in expressions {
                    val = e.evaluate(map)?;
                }
                Ok(val) // return the last evaluated expression;
            }
            Ast::Conditional {
                ref condition,
                ref true_expr,
                ref false_expr,
            } => {
                match condition.evaluate(map)? {
                    Datatype::Bool(bool) => {
                        match bool {
                            true => Ok(true_expr.evaluate(map)?),
                            false => {
                                match *false_expr {
                                    Some(ref e) => Ok(e.evaluate(map)?),
                                    _ => Ok(Datatype::None),
                                }
                            }
                        }
                    }
                    _ => Err(LangError::ConditionOnNonBoolean),
                }
            }
            Ast::Literal(ref datatype) => Ok(datatype.clone()),
            Ast::Type(ref datatype) => Err(LangError::TriedToEvaluateTypeInfo(datatype.clone())), // you shouldn't try to evaluate the datatype,
            Ast::ValueIdentifier(ref ident) => {
                match map.get(ident) {
                    Some(value) => Ok(value.clone()),
                    None => Err(LangError::VariableDoesntExist(
                        format!("Variable `{}` hasn't been assigned yet", ident),
                    )), // identifier hasn't been assigned yet.
                }
            }
        }
    }
}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn plus_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Expression {
            operator: BinaryOperator::Plus,
            expr1: Box::new(Ast::Literal(Datatype::Number(3))),
            expr2: Box::new(Ast::Literal(Datatype::Number(6))),
        };
        assert_eq!(Datatype::Number(9), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn string_plus_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Expression {
            operator: BinaryOperator::Plus,
            expr1: Box::new(Ast::Literal(Datatype::String("Hello".to_string()))),
            expr2: Box::new(Ast::Literal(Datatype::String(" World!".to_string()))),
        };
        assert_eq!(
        Datatype::String("Hello World!".to_string()),
        ast.evaluate(&mut map).unwrap()
        )
    }

    #[test]
    fn minus_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Expression {
            operator: BinaryOperator::Minus,
            expr1: Box::new(Ast::Literal(Datatype::Number(6))),
            expr2: Box::new(Ast::Literal(Datatype::Number(3))),
        };
        assert_eq!(Datatype::Number(3), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn minus_negative_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Expression {
            operator: BinaryOperator::Minus,
            expr1: Box::new(Ast::Literal(Datatype::Number(3))),
            expr2: Box::new(Ast::Literal(Datatype::Number(6))),
        };
        assert_eq!(Datatype::Number(-3), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn multiplication_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Expression {
            operator: BinaryOperator::Multiply,
            expr1: Box::new(Ast::Literal(Datatype::Number(6))),
            expr2: Box::new(Ast::Literal(Datatype::Number(3))),
        };
        assert_eq!(Datatype::Number(18), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn division_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Expression {
            operator: BinaryOperator::Divide,
            expr1: Box::new(Ast::Literal(Datatype::Number(6))),
            expr2: Box::new(Ast::Literal(Datatype::Number(3))),
        };
        assert_eq!(Datatype::Number(2), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn integer_division_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Expression {
            operator: BinaryOperator::Divide,
            expr1: Box::new(Ast::Literal(Datatype::Number(5))),
            expr2: Box::new(Ast::Literal(Datatype::Number(3))),
        };
        assert_eq!(Datatype::Number(1), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn division_by_zero_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Expression {
            operator: BinaryOperator::Divide,
            expr1: Box::new(Ast::Literal(Datatype::Number(5))),
            expr2: Box::new(Ast::Literal(Datatype::Number(0))),
        };
        assert_eq!(
        LangError::DivideByZero,
        ast.evaluate(&mut map).err().unwrap()
        )
    }


    #[test]
    fn modulo_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Expression {
            operator: BinaryOperator::Modulo,
            expr1: Box::new(Ast::Literal(Datatype::Number(8))),
            expr2: Box::new(Ast::Literal(Datatype::Number(3))),
        };
        assert_eq!(Datatype::Number(2), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn equality_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Expression {
            operator: BinaryOperator::Equals,
            expr1: Box::new(Ast::Literal(Datatype::Number(3))),
            expr2: Box::new(Ast::Literal(Datatype::Number(3))),
        };
        assert_eq!(Datatype::Bool(true), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn greater_than_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Expression {
            operator: BinaryOperator::GreaterThan,
            expr1: Box::new(Ast::Literal(Datatype::Number(4))),
            expr2: Box::new(Ast::Literal(Datatype::Number(3))),
        };
        assert_eq!(Datatype::Bool(true), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn less_than_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Expression {
            operator: BinaryOperator::LessThan,
            expr1: Box::new(Ast::Literal(Datatype::Number(2))),
            expr2: Box::new(Ast::Literal(Datatype::Number(3))),
        };
        assert_eq!(Datatype::Bool(true), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn greater_than_or_equal_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Expression {
            operator: BinaryOperator::GreaterThanOrEqual,
            expr1: Box::new(Ast::Literal(Datatype::Number(4))),
            expr2: Box::new(Ast::Literal(Datatype::Number(3))),
        };
        assert_eq!(Datatype::Bool(true), ast.evaluate(&mut map).unwrap());

        let mut map: HashMap<String, Datatype> = HashMap::new();
        let equals_ast = Ast::Expression {
            operator: BinaryOperator::GreaterThanOrEqual,
            expr1: Box::new(Ast::Literal(Datatype::Number(5))),
            expr2: Box::new(Ast::Literal(Datatype::Number(5))),
        };
        assert_eq!(Datatype::Bool(true), equals_ast.evaluate(&mut map).unwrap());
    }

    #[test]
    fn less_than_or_equal_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Expression {
            operator: BinaryOperator::LessThanOrEqual,
            expr1: Box::new(Ast::Literal(Datatype::Number(2))),
            expr2: Box::new(Ast::Literal(Datatype::Number(3))),
        };
        assert_eq!(Datatype::Bool(true), ast.evaluate(&mut map).unwrap());

        let mut map: HashMap<String, Datatype> = HashMap::new();
        let equals_ast = Ast::Expression {
            operator: BinaryOperator::LessThanOrEqual,
            expr1: Box::new(Ast::Literal(Datatype::Number(5))),
            expr2: Box::new(Ast::Literal(Datatype::Number(5))),
        };
        assert_eq!(Datatype::Bool(true), equals_ast.evaluate(&mut map).unwrap());
    }

    /// Assign the value 6 to the identifier "a"
/// Recall that identifier and add it to 5
    #[test]
    fn assignment_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::VecExpression {
            expressions: vec![
                Ast::Expression {
                    operator: BinaryOperator::Assignment,
                    expr1: Box::new(Ast::ValueIdentifier("a".to_string())),
                    expr2: Box::new(Ast::Literal(Datatype::Number(6))),
                },
                Ast::Expression {
                    operator: BinaryOperator::Plus,
                    expr1: Box::new(Ast::ValueIdentifier("a".to_string())),
                    expr2: Box::new(Ast::Literal(Datatype::Number(5))),
                },
            ],
        };
        assert_eq!(Datatype::Number(11), ast.evaluate(&mut map).unwrap())
    }


    /// Assign the value 6 to "a".
/// Copy the value in "a" to "b".
/// Recall the value in "b" and add it to 5.
    #[test]
    fn variable_copy_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::VecExpression {
            expressions: vec![
                Ast::Expression {
                    operator: BinaryOperator::Assignment,
                    expr1: Box::new(Ast::ValueIdentifier("a".to_string())),
                    expr2: Box::new(Ast::Literal(Datatype::Number(6))),
                },
                Ast::Expression {
                    operator: BinaryOperator::Assignment,
                    expr1: Box::new(Ast::ValueIdentifier("b".to_string())),
                    expr2: Box::new(Ast::ValueIdentifier("a".to_string())),
                },
                Ast::Expression {
                    operator: BinaryOperator::Plus,
                    expr1: Box::new(Ast::ValueIdentifier("b".to_string())),
                    expr2: Box::new(Ast::Literal(Datatype::Number(5))),
                },
            ],
        };
        assert_eq!(Datatype::Number(11), ast.evaluate(&mut map).unwrap())
    }

    /// Assign the value 6 to a.
/// Assign the value 3 to a.
/// Recall the value in a and add it to 5, the value of a should be 3, equalling 8.
    #[test]
    fn reassignment_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::VecExpression {
            expressions: vec![
                Ast::Expression {
                    operator: BinaryOperator::Assignment,
                    expr1: Box::new(Ast::ValueIdentifier("a".to_string())),
                    expr2: Box::new(Ast::Literal(Datatype::Number(6))),
                },
                Ast::Expression {
                    operator: BinaryOperator::Assignment,
                    expr1: Box::new(Ast::ValueIdentifier("a".to_string())),
                    expr2: Box::new(Ast::Literal(Datatype::Number(3))),
                },
                Ast::Expression {
                    operator: BinaryOperator::Plus,
                    expr1: Box::new(Ast::ValueIdentifier("a".to_string())),
                    expr2: Box::new(Ast::Literal(Datatype::Number(5))),
                },
            ],
        };
        assert_eq!(Datatype::Number(8), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn conditional_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Conditional {
            condition: Box::new(Ast::Literal(Datatype::Bool(true))),
            true_expr: Box::new(Ast::Literal(Datatype::Number(7))),
            false_expr: None,
        };
        assert_eq!(Datatype::Number(7), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn conditional_with_else_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::Conditional {
            condition: Box::new(Ast::Literal(Datatype::Bool(false))),
            true_expr: Box::new(Ast::Literal(Datatype::Number(7))),
            false_expr: Some(Box::new(Ast::Literal(Datatype::Number(2)))),
        };
        assert_eq!(Datatype::Number(2), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn basic_function_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::VecExpression {
            expressions: vec![
                Ast::Expression {
                    operator: BinaryOperator::Assignment,
                    expr1: Box::new(Ast::ValueIdentifier("a".to_string())),
                    expr2: Box::new(Ast::Literal(Datatype::Function {
                        parameters: Box::new(Ast::VecExpression { expressions: vec![] }),
                        // empty parameters
                        body: (Box::new(Ast::Literal(Datatype::Number(32)))),
                        // just return a number
                        return_type: Box::new(Ast::Type(TypeInfo::Number)),
                        // expect a number
                    })),
                },
                Ast::Expression {
                    operator: BinaryOperator::ExecuteFn,
                    expr1: Box::new(Ast::ValueIdentifier("a".to_string())),
                    // get the identifier for a
                    expr2: Box::new(Ast::VecExpression { expressions: vec![] }),
                    // provide the function parameters
                },
            ],
        };
        assert_eq!(Datatype::Number(32), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn function_with_parameter_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::VecExpression {
            expressions: vec![
                Ast::Expression {
                    operator: BinaryOperator::Assignment,
                    expr1: Box::new(Ast::ValueIdentifier("a".to_string())),
                    expr2: Box::new(Ast::Literal(Datatype::Function {
                        parameters: Box::new(Ast::VecExpression {
                            expressions: vec![
                                Ast::Expression {
                                    operator: BinaryOperator::FunctionParameterAssignment,
                                    expr1: Box::new(Ast::ValueIdentifier("b".to_string())),
                                    // the value's name is b
                                    expr2: Box::new(Ast::Type(TypeInfo::Number)),
                                    // fn takes a number
                                },
                            ],
                        }),
                        body: (Box::new(Ast::ValueIdentifier("b".to_string()))),
                        // just return the number passed in.
                        return_type: Box::new(Ast::Type(TypeInfo::Number)),
                        // expect a number to be returned
                    })),
                },
                Ast::Expression {
                    operator: BinaryOperator::ExecuteFn,
                    expr1: Box::new(Ast::ValueIdentifier("a".to_string())),
                    // get the identifier for a
                    expr2: Box::new(Ast::VecExpression {
                        expressions: vec![Ast::Literal(Datatype::Number(7))],
                    }),
                    // provide the function parameters
                },
            ],
        };
        assert_eq!(Datatype::Number(7), ast.evaluate(&mut map).unwrap())
    }


    #[test]
    fn function_with_two_parameters_addition_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::VecExpression {
            expressions: vec![
                Ast::Expression {
                    operator: BinaryOperator::Assignment,
                    expr1: Box::new(Ast::ValueIdentifier("add_two_numbers".to_string())),
                    expr2: Box::new(Ast::Literal(Datatype::Function {
                        parameters: Box::new(Ast::VecExpression {
                            expressions: vec![
                                Ast::Expression {
                                    operator: BinaryOperator::FunctionParameterAssignment,
                                    expr1: Box::new(Ast::ValueIdentifier("b".to_string())),
                                    // the value's name is b
                                    expr2: Box::new(Ast::Type(TypeInfo::Number)),
                                    // fn takes a number
                                },
                                Ast::Expression {
                                    operator: BinaryOperator::FunctionParameterAssignment,
                                    expr1: Box::new(Ast::ValueIdentifier("c".to_string())),
                                    // the value's name is b
                                    expr2: Box::new(Ast::Type(TypeInfo::Number)),
                                    // fn takes a number
                                },
                            ],
                        }),
                        body: (Box::new(Ast::Expression {
                            // the body of the function will add the two passed in values together
                            operator: BinaryOperator::Plus,
                            expr1: Box::new(Ast::ValueIdentifier("b".to_string())),
                            expr2: Box::new(Ast::ValueIdentifier("c".to_string())),
                        })),

                        return_type: Box::new(Ast::Type(TypeInfo::Number)),
                        // expect a number to be returned
                    })),
                },
                Ast::Expression {
                    operator: BinaryOperator::ExecuteFn,
                    expr1: Box::new(Ast::ValueIdentifier("add_two_numbers".to_string())),
                    // get the identifier for a
                    expr2: Box::new(Ast::VecExpression {
                        expressions: vec![
                            Ast::Literal(Datatype::Number(7)),
                            Ast::Literal(Datatype::Number(5)),
                        ],
                    }),
                    // provide the function parameters
                },
            ],
        };
        assert_eq!(Datatype::Number(12), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn array_access_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast: Ast = Ast::Expression {
            operator: BinaryOperator::AccessArray,
            expr1: Box::new(Ast::Literal(Datatype::Array {
                value: vec![
                    Datatype::Number(12),
                    Datatype::Number(14),
                    Datatype::Number(16)
                ],
                type_: TypeInfo::Number
            })),
            expr2: Box::new(Ast::Literal(Datatype::Number(0))) // get the first element
        };
        assert_eq!(Datatype::Number(12), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn array_incorrect_access_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast: Ast = Ast::Expression {
            operator: BinaryOperator::AccessArray,
            expr1: Box::new(Ast::Literal(Datatype::Array {
                value: vec![
                    Datatype::Number(12),
                    Datatype::Number(14),
                    Datatype::Number(16)
                ],
                type_: TypeInfo::Number
            })),
            expr2: Box::new(Ast::Literal(Datatype::Number(3))) // Array size 3. 0, 1, 2 hold elements. Index 3 doesn't.
        };
        assert_eq!(LangError::OutOfBoundsArrayAccess, ast.evaluate(&mut map).unwrap_err())
    }

    #[test]
    fn struct_declaration_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast: Ast = Ast::Expression {
            operator: BinaryOperator::StructDeclaration,
            expr1: Box::new(Ast::ValueIdentifier("MyStruct".to_string())),
            expr2: Box::new(Ast::VecExpression {
                expressions: vec![
                    Ast::Expression {
                        operator: BinaryOperator::FunctionParameterAssignment,
                        expr1: Box::new(Ast::ValueIdentifier("Field1".to_string())),
                        expr2: Box::new(Ast::Type(TypeInfo::Number))
                    }
                ]
            })
        };

        let _ = ast.evaluate(&mut map); // execute the ast to add the struct entry to the global stack map.
        let mut expected_map = HashMap::new();
        let mut inner_struct_hash_map = HashMap::new();
        inner_struct_hash_map.insert("Field1".to_string(), TypeInfo::Number);
        expected_map.insert("MyStruct".to_string(), Datatype::StructType(TypeInfo::Struct { map: inner_struct_hash_map }));
        assert_eq!(expected_map, map)
    }


    #[test]
    fn struct_creation_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let declaration_ast: Ast = Ast::Expression {
            operator: BinaryOperator::StructDeclaration,
            expr1: Box::new(Ast::ValueIdentifier("MyStruct".to_string())),
            expr2: Box::new(Ast::VecExpression {
                expressions: vec![
                    Ast::Expression {
                        operator: BinaryOperator::FunctionParameterAssignment,
                        expr1: Box::new(Ast::ValueIdentifier("Field1".to_string())),
                        expr2: Box::new(Ast::Type(TypeInfo::Number))
                    }
                ]
            })
        };
        let _ = declaration_ast.evaluate(&mut map); // execute the ast to add the struct entry to the global stack map.

        let creation_ast: Ast = Ast::Expression {
            operator: BinaryOperator::CreateStruct,
            expr1: Box::new(Ast::ValueIdentifier("MyStruct".to_string())),
            expr2: Box::new( Ast::VecExpression {
                expressions: vec![
                    Ast::Expression {
                        operator: BinaryOperator::FunctionParameterAssignment,
                        expr1: Box::new(Ast::ValueIdentifier("Field1".to_string())),
                        expr2: Box::new(Ast::Literal(Datatype::Number(8))) // assign 8 to field Field1
                    }
                ]
            })
        };

        let struct_instance = creation_ast.evaluate(&mut map).unwrap();

        let mut inner_struct_hash_map = HashMap::new();
        inner_struct_hash_map.insert("Field1".to_string(), Datatype::Number(8));

        assert_eq!(Datatype::Struct{ map: inner_struct_hash_map}, struct_instance)
    }
}