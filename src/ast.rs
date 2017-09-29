use lang_result::*;
use datatype::{Datatype, TypeInfo};
//use std::mem::discriminant;

use std::boxed::Box;
use std::collections::HashMap;
use include::read_file_into_ast;

use s_expression::SExpression;

/// Used for finding the main function.
const MAIN_FUNCTION_NAME: &'static str = "main";


/// Abstract Syntax Tree
/// A recursive data structure that holds instances of other ASTs.
/// It encodes the operations that are defined by the language's syntax.
/// Evaluating an Ast will produce either a Datatype or a LangError
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Ast {
    SExpr(SExpression), // Operators that store their operands
    ExpressionList(Vec<Ast>), // uesd for structuring execution of the AST.
    Conditional {
        condition: Box<Ast>, // Resolves to a literal->bool
        true_expr: Box<Ast>, // Will execute if the condition is true
        false_expr: Option<Box<Ast>>, // Will execute if the condition is false
    },
    Literal(Datatype), // consider making the Literal another enum with supported default datatypes.
    Type(TypeInfo), // value in the datatype is useless, just use this to determine parameter type.
    ValueIdentifier(String), // gets the value mapped to a hashmap
}



impl Ast {
    /// Moves functions and structs to the top of the Ast's top level ExpressionList.
    /// This is done because regardless of where a function is declared, a datatype representing it
    /// will be encoded in the program map before it is used.
    /// If the AST doesn't have an Expression list, this will throw an error.
    pub fn hoist_functions_and_structs(&self) -> Ast {
        match *self {
            Ast::ExpressionList(ref expressions) => {
                // Capture struct declarations and hoist them to the top of the AST
                let mut struct_declarations: Vec<Ast> = vec![];
                // Capture the functions as well and move them to the top of the evaluation list.
                let mut function_declarations: Vec<Ast> = vec![];
                // Keep the ordering of everything else, likely constant assignments.
                let mut everything_else: Vec<Ast> = vec![];

                for ast in expressions {

                    match *ast {
                        Ast::SExpr(ref sexpr) => {
                            match *sexpr {
                                SExpression::CreateStruct { .. } => {
                                    let ast = ast.clone();
                                    struct_declarations.push(ast);
                                }
                                SExpression::CreateFunction { .. } => {
                                    let ast = ast.clone();
                                    function_declarations.push(ast);
                                }
                                _ => {
                                    //If it isn't a creation of a struct or function, put it in the everything else bucket.
                                    let ast = ast.clone();
                                    everything_else.push(ast)
                                }
                            }
                        }
                        _ => {
                            // If it isn't one of the desired expression types, put it in the everything else bucket.
                            let ast = ast.clone();
                            everything_else.push(ast)
                        }
                    }
                }
                // Rearrange the order in which the top level AST node is evaluated
                let mut eval_list_with_hoists: Vec<Ast> = vec![];
                eval_list_with_hoists.append(&mut struct_declarations);
                eval_list_with_hoists.append(&mut function_declarations);
                eval_list_with_hoists.append(&mut everything_else);

                Ast::ExpressionList(eval_list_with_hoists)
            }
            _ => panic!("Tried to hoist a non list of expressions."),
        }
    }

    /// Determines if a main function exists.
    /// This should be used to determine if calling the main function after evaluating the AST is necessary.
    pub fn main_fn_exists(&self) -> bool {
        match *self {
            Ast::ExpressionList(ref expressions) => {
                for ast in expressions {
                    match *ast {
                        Ast::SExpr(ref sexpr) => {
                            if let SExpression::CreateFunction {
                                ref identifier,
                                function_datatype: ref _function_datatype,
                            } = *sexpr
                            {
                                if let Ast::ValueIdentifier(ref fn_name) = **identifier {
                                    if fn_name.as_str() == MAIN_FUNCTION_NAME {
                                        return true;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        false
    }

    /// Calls the main function.
    /// This should be used if a main function has been determined to exist.
    pub fn execute_main(&self, map: &mut HashMap<String, Datatype>) -> LangResult {
        let executing_ast = Ast::SExpr(SExpression::ExecuteFn {
            identifier: Box::new(Ast::ValueIdentifier(MAIN_FUNCTION_NAME.to_string())),
            parameters: Box::new(Ast::ExpressionList(vec![])),
        });

        executing_ast.evaluate(map)
    }


    /// Recurse down the AST, evaluating expressions where possible, turning them into Literals that contain Datatypes.
    /// If no errors are encountered, the whole AST should resolve to become a single Datatype, which is then returned.
    pub fn evaluate(&self, map: &mut HashMap<String, Datatype>) -> LangResult {
        match *self {
            Ast::SExpr(ref sexpr) => {
                match *sexpr {
                    SExpression::Add(ref lhs, ref rhs) => lhs.evaluate(map)? + rhs.evaluate(map)?,
                    SExpression::Subtract(ref lhs, ref rhs) => {
                        lhs.evaluate(map)? - rhs.evaluate(map)?
                    }
                    SExpression::Multiply(ref lhs, ref rhs) => {
                        lhs.evaluate(map)? * rhs.evaluate(map)?
                    }
                    SExpression::Divide(ref lhs, ref rhs) => {
                        lhs.evaluate(map)? / rhs.evaluate(map)?
                    }
                    SExpression::Modulo(ref lhs, ref rhs) => {
                        lhs.evaluate(map)? % rhs.evaluate(map)?
                    }
                    SExpression::Equals(ref lhs, ref rhs) => {
                        if lhs.evaluate(map)? == rhs.evaluate(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    SExpression::NotEquals(ref lhs, ref rhs) => {
                        if lhs.evaluate(map)? != rhs.evaluate(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    SExpression::GreaterThan(ref lhs, ref rhs) => {
                        if lhs.evaluate(map)? > rhs.evaluate(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    SExpression::LessThan(ref lhs, ref rhs) => {
                        if lhs.evaluate(map)? < rhs.evaluate(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    SExpression::GreaterThanOrEqual(ref lhs, ref rhs) => {
                        if lhs.evaluate(map)? >= rhs.evaluate(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    SExpression::LessThanOrEqual(ref lhs, ref rhs) => {
                        if lhs.evaluate(map)? <= rhs.evaluate(map)? {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    SExpression::LogicalAnd(ref lhs, ref rhs) => {
                        let lhs_bool: bool = match rhs.evaluate(map)? {
                            Datatype::Bool(b) => b,
                            _ => {
                                return Err(LangError::TypeError {
                                    expected: (TypeInfo::Bool),
                                    found: (TypeInfo::from(lhs.evaluate(&mut map.clone())?)),
                                })
                            }
                        };
                        let rhs_bool: bool = match rhs.evaluate(map)? {
                            Datatype::Bool(b) => b,
                            _ => {
                                return Err(LangError::TypeError {
                                    expected: (TypeInfo::Bool),
                                    found: (TypeInfo::from(rhs.evaluate(&mut map.clone())?)),
                                })
                            }
                        };

                        if lhs_bool && rhs_bool {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    SExpression::LogicalOr(ref lhs, ref rhs) => {
                        let lhs_bool: bool = match rhs.evaluate(map)? {
                            Datatype::Bool(b) => b,
                            _ => {
                                return Err(LangError::TypeError {
                                    expected: (TypeInfo::Bool),
                                    found: (TypeInfo::from(lhs.evaluate(&mut map.clone())?)),
                                })
                            }
                        };
                        let rhs_bool: bool = match rhs.evaluate(map)? {
                            Datatype::Bool(b) => b,
                            _ => {
                                return Err(LangError::TypeError {
                                    expected: (TypeInfo::Bool),
                                    found: (TypeInfo::from(rhs.evaluate(&mut map.clone())?)),
                                })
                            }
                        };

                        if lhs_bool || rhs_bool {
                            return Ok(Datatype::Bool(true));
                        } else {
                            return Ok(Datatype::Bool(false));
                        }
                    }
                    SExpression::Assignment {
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
                    SExpression::CreateFunction {
                        identifier: ref lhs,
                        function_datatype: ref rhs,
                    } => {
                        if let Ast::ValueIdentifier(ref ident) = **lhs {
                            let mut cloned_map = map.clone(); // since this is a clone, the required righthand expressions will be evaluated in their own 'stack', this modified hashmap will be cleaned up post assignment.
                            let evaluated_right_hand_side = rhs.evaluate(&mut cloned_map)?;
                            let cloned_evaluated_rhs = evaluated_right_hand_side.clone();
                            map.insert(ident.clone(), evaluated_right_hand_side);
                            return Ok(cloned_evaluated_rhs);
                        } else {
                            Err(LangError::IdentifierDoesntExist)
                        }
                    }
                    SExpression::Loop {
                        ref conditional,
                        ref body,
                    } => {
                        let mut evaluated_loop: Datatype = Datatype::None;
                        let cloned_conditional = *conditional.clone();
                        loop {
                            let condition: Datatype = cloned_conditional.evaluate(map)?;
                            match condition {
                                Datatype::Bool(b) => {
                                    if b {
                                        evaluated_loop = body.evaluate(map)?; // This doesn't clone the map, so it can mutate the "stack" of its context
                                    } else {
                                        break;
                                    }
                                }
                                _ => {
                                    return Err(LangError::ConditionalNotBoolean(
                                        TypeInfo::from(condition),
                                    ))
                                }
                            }
                        }
                        return Ok(evaluated_loop); // leave block
                    }
                    SExpression::AccessArray {
                        ref identifier,
                        ref index,
                    } => {
                        let datatype: Datatype = identifier.evaluate(map)?;
                        match datatype {
                            Datatype::Array { value, .. } => {
                                let possible_index = index.evaluate(map)?;
                                match possible_index {
                                    Datatype::Number(resolved_index) => {
                                        if resolved_index >= 0 {
                                            match value.get(resolved_index as usize) {
                                                Some(indexed_result) => Ok(indexed_result.clone()), // cannot mutate the interior of the array.
                                                None => Err(LangError::OutOfBoundsArrayAccess),
                                            }
                                        } else {
                                            Err(LangError::NegativeIndex(resolved_index))
                                        }
                                    }
                                    _ => Err(LangError::InvalidIndexType(possible_index)),
                                }
                            }
                            _ => {
                                return Err(
                                    LangError::ArrayAccessOnNonArry(TypeInfo::from(datatype)),
                                )
                            }
                        }
                    }

                    SExpression::StructDeclaration {
                        identifier: ref lhs,
                        struct_type_info: ref rhs,
                    } => return declare_struct(lhs, rhs, map),
                    SExpression::AccessStructField {
                        identifier: ref lhs,
                        field_identifier: ref rhs,
                    } => return access_struct_field(lhs, rhs, map),
                    SExpression::CreateStruct {
                        identifier: ref lhs,
                        struct_datatype: ref rhs,
                    } => return create_struct(lhs, rhs, map),
                    SExpression::ExecuteFn {
                        ref identifier,
                        ref parameters,
                    } => return execute_function(identifier, parameters, map),
                    SExpression::Print(ref expr) => {
                        let datatype_to_print = expr.evaluate(map)?;
                        if let Ast::ValueIdentifier(ref identifier) = **expr {
                            if let Datatype::Struct { .. } = datatype_to_print {
                                print!("{}{}", identifier, datatype_to_print)
                            } else {
                                print!("{}", datatype_to_print);
                            }
                        } else {
                            print!("{}", datatype_to_print);
                        }
                        Ok(datatype_to_print)
                    }
                    SExpression::Include(ref expr) => {
                        match expr.evaluate(map)? {
                            Datatype::String(filename) => {
                                let new_ast: Ast = read_file_into_ast(filename)?;
                                new_ast.evaluate(map) // move the new AST into the current AST
                            }
                            _ => Err(LangError::CouldNotReadFile {
                                filename: "Not provided".to_string(),
                                reason: "File name was not a string.".to_string(),
                            }),
                        }
                    }
                    SExpression::Invert(ref expr) => {
                        match expr.evaluate(map)? {
                            Datatype::Bool(bool) => Ok(Datatype::Bool(!bool)),
                            _ => Err(LangError::InvertNonBoolean),
                        }
                    }
                    SExpression::Increment(ref expr) => {
                        match expr.evaluate(map)? {
                            Datatype::Number(number) => Ok(Datatype::Number(number + 1)),
                            _ => Err(LangError::IncrementNonNumber),
                        }
                    }
                    SExpression::Decrement(ref expr) => {
                        match expr.evaluate(map)? {
                            Datatype::Number(number) => Ok(Datatype::Number(number - 1)),
                            _ => Err(LangError::DecrementNonNumber),
                        }
                    }
                }
            }

            //Evaluate multiple expressions and return the result of the last one.
            Ast::ExpressionList(ref expressions) => {
                let mut val: Datatype = Datatype::None; // TODO, consider making this return an error if the expressions vector is empty
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
                    )),
                }
            }
        }
    }
}

/// Resolve the first expression to a struct.
/// Resolve the second expression to an identifier.
/// Check if the second expression's identifier is in the struct's map.
/// If it is, the get the value associated with that identifier and return it.
fn access_struct_field(
    struct_identifier: &Ast,
    field_identifier: &Ast,
    map: &mut HashMap<String, Datatype>,
) -> LangResult {
    // if struct_identifier produces a struct when evaluated
    if let Datatype::Struct { map: struct_map } = struct_identifier.evaluate(map)? {
        if let Ast::ValueIdentifier(ref field_identifier) = *field_identifier {
            match struct_map.get(field_identifier) {
                Some(struct_field_datatype) => return Ok(struct_field_datatype.clone()),
                None => return Err(LangError::StructFieldDoesntExist),
            }
        } else {
            return Err(LangError::IdentifierDoesntExist);
        }
    } else {
        return Err(LangError::TriedToAccessNonStruct);
    }
}

/// Given an Identifier,
/// and a vector of expressions tha contains only TypeAssignments.
/// Create a new map that will represent the binding between the fields in the struct and the types they should have when instansiated.
/// Insert into the map these field-type pairs.
/// Create a new StructType from the new map and return it.
fn declare_struct(
    identifier: &Ast,
    struct_type_assignments: &Ast,
    map: &mut HashMap<String, Datatype>,
) -> LangResult {
    if let Ast::ValueIdentifier(ref struct_type_identifier) = *identifier {
        if let Ast::ExpressionList(ref expressions) = *struct_type_assignments {

            let mut struct_map: HashMap<String, TypeInfo> = HashMap::new();

            for assignment_expr in expressions {
                if let &Ast::SExpr(ref sexpr) = assignment_expr {
                    if let SExpression::TypeAssignment {
                        identifier: ref field_identifier_expr,
                        type_info: ref field_type_expr,
                    } = *sexpr
                    {
                        if let Ast::ValueIdentifier(ref field_id) = **field_identifier_expr {
                            if let Ast::Type(ref field_type) = **field_type_expr {
                                struct_map.insert(field_id.clone(), field_type.clone());
                            } else {
                                return Err(LangError::FieldTypeNotSupplied);
                            }
                        } else {
                            return Err(LangError::FieldIdentifierNotSupplied);
                        }
                    } else {
                        return Err(LangError::NonAssignmentInStructDeclaration);
                    }
                } else {
                    return Err(LangError::ExpectedExpression);
                }
            }
            let new_struct_type = TypeInfo::Struct { map: struct_map };
            let retval = Datatype::StructType(new_struct_type);
            map.insert(struct_type_identifier.clone(), retval.clone());
            return Ok(retval);
        } else {
            return Err(LangError::StructBodyNotSupplied);
        }
    } else {
        Err(LangError::StructNameNotSupplied)
    }
}

/// Given an Ast that resolves to a struct type,
/// and a vector of expressions that contains only FieldAssignments.
/// Grab the map of identifiers to expected Types for fields.
/// Create a new map of identifiers and Datatypes
/// Get the expected type for each supplied assignment from the first map and check it against the type of data provided.
/// Insert the data into the new map.
/// Create a new struct instance from the new map, and return it.
fn create_struct(expr1: &Ast, expr2: &Ast, map: &mut HashMap<String, Datatype>) -> LangResult {
    // This expects the expr1 to be an Identifier that resolves to be a struct definition, or the struct definition itself.
    if let Datatype::StructType(TypeInfo::Struct { map: struct_type_map }) = expr1.evaluate(map)? {
        // It expects that the righthand side should be a series of expressions that assign values to fields (that have already been specified in the StructType)
        if let Ast::ExpressionList(ref assignment_expressions) = *expr2 {
            let mut new_struct_map: HashMap<String, Datatype> = HashMap::new();

            for assignment_expression in assignment_expressions {
                if let Ast::SExpr(ref sexpr) = *assignment_expression {
                    if let SExpression::FieldAssignment {
                        identifier: ref assignment_expr1,
                        ast: ref assignment_expr2,
                    } = *sexpr
                    {
                        if let Ast::ValueIdentifier(ref field_identifier) = **assignment_expr1 {
                            // Is the identifier specified in the AST exist in the struct type? check the struct_map
                            let expected_type = match struct_type_map.get(field_identifier) {
                                Some(struct_type) => struct_type,
                                None => return Err(LangError::IdentifierDoesntExist), // todo find better error message
                            };
                            let value_to_be_assigned: Datatype = assignment_expr2.evaluate(map)?;

                            // check if the value to be assigned matches the expected type
                            let to_be_assigned_type: TypeInfo =
                                TypeInfo::from(value_to_be_assigned.clone());
                            if expected_type == &to_be_assigned_type {
                                // now add the value to the new struct's map
                                new_struct_map.insert(
                                    field_identifier.clone(),
                                    value_to_be_assigned,
                                );
                            } else {
                                return Err(LangError::TypeError {
                                    expected: expected_type.clone(),
                                    found: to_be_assigned_type,
                                });
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
            return Ok(Datatype::Struct { map: new_struct_map }); // Return the new struct.
        } else {
            return Err(LangError::StructBodyNotSupplied); // not entirely accurate
        }
    } else {
        return Err(LangError::ExpectedIdentifier);
    }
}

/// Given an identifier that resolves to a function (with expected parameters, the function body, and return type) and a set of input expressions that resolve to Datatypes,
/// Resolve the input input expressions to Datatypes.
/// Resolve the identifier to a function.
/// Check if the input Datatypes have the same types as the supplied function.
/// Map the input datatypes onto the identifiers expected in the function prototype.
/// Evaluate the function with the substituted datatypes as input.
/// Check if the return type is the same as was expected by the function.
/// Return the return value.
fn execute_function(
    identifier: &Ast,
    function: &Ast,
    map: &HashMap<String, Datatype>,
) -> LangResult {
    let mut cloned_map = map.clone(); // clone the map, to create a temporary new "stack" for the life of the function

    // evaluate the parameters
    let evaluated_parameters: Vec<Datatype> = match *function {
        Ast::ExpressionList(ref expressions) => {
            let mut evaluated_expressions: Vec<Datatype> = vec![];
            for e in expressions {
                match e.evaluate(&mut cloned_map) {
                    Ok(dt) => evaluated_expressions.push(dt),
                    Err(err) => return Err(err),
                }
            }
            evaluated_expressions
        }
        _ => return Err(LangError::FunctionParametersShouldBeExpressionList),
    };


    // Take an existing function by (by grabbing the function using an identifier, which should resolve to a function)
    match identifier.evaluate(&mut cloned_map)? {
        Datatype::Function {
            parameters,
            body,
            return_type,
        } => {
            match *parameters {
                // The parameters should be in the form: ExpressionList(expression_with_fn_assignment, expression_with_fn_assignment, ...) This way, functions should be able to support arbitrary numbers of parameters.
                Ast::ExpressionList(expressions) => {
                    // zip the values of the evaluated parameters into the expected parameters for the function
                    if evaluated_parameters.len() == expressions.len() {
                        // Replace the right hand side of the expression (which should be an Ast::Type with a computed literal.
                        let rhs_replaced_with_evaluated_parameters_results: Vec<Result<Ast, LangError>> = expressions
                            .iter()
                            .zip(evaluated_parameters)
                            .map(|expressions_with_parameters: (&Ast, Datatype)| {
                                let (expression, datatype) = expressions_with_parameters; // assign out of tuple.
                                if let Ast::SExpr(
                                    ref sexpr
                                ) = *expression
                                    {
                                        if let SExpression::TypeAssignment {
                                            ref identifier,
                                            ref type_info
                                        } = *sexpr {
                                            let identifier: Box<Ast> = identifier.clone();

                                            //do run-time type-checking, the supplied value should be of the same type as the specified value
                                            let expected_type: &TypeInfo = match **type_info {
                                                Ast::Type(ref datatype) => datatype,
                                                Ast::ValueIdentifier(ref id) => {
                                                    match map.get(id) {
                                                        // get what should be a struct out of the stack
                                                        Some(datatype) => {
                                                            if let Datatype::StructType(ref struct_type_info) = *datatype {
                                                                struct_type_info
                                                            } else {
                                                                return Err(LangError::ExpectedIdentifierToBeStructType {
                                                                    found: id.clone(),
                                                                });
                                                            }
                                                        }
                                                        None => return Err(LangError::IdentifierDoesntExist),
                                                    }
                                                }
                                                _ => return Err(LangError::ExpectedDataTypeInfo),
                                            };
                                            // Convert the datatype into a TypeInfo and check it against the expected type
                                            if expected_type != &TypeInfo::from(datatype.clone()) {
                                                return Err(LangError::TypeError {
                                                    expected: expected_type.clone(),
                                                    found: TypeInfo::from(datatype),
                                                });
                                            }
                                            // Return a new FunctionParameterAssignment Expression with the same identifier
                                            // pointing to a literal that was reduced from the expression passed in as a parameter.
                                            return Ok(Ast::SExpr(SExpression::FieldAssignment {
                                                identifier: identifier,
                                                ast: Box::new(Ast::Literal(datatype))
                                            }))
                                        } else {
                                            return Err(LangError::InvalidFunctionPrototypeFormatting);
                                        }
                                    } else {
                                    return Err(LangError::InvalidFunctionPrototypeFormatting)
                                }
                            }).collect();

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
                                        return Err(LangError::ExpectedIdentifierToBeStructType {
                                            found: id.clone(),
                                        });
                                    }
                                }
                                None => return Err(LangError::IdentifierDoesntExist),
                            }
                        }
                        _ => return Err(LangError::ExpectedDataTypeInfo),
                    };

                    if TypeInfo::from(output.clone()) == expected_return_type {
                        return Ok(output);
                    } else {
                        return Err(LangError::ReturnTypeDoesNotMatchReturnValue);
                    }
                }
                _ => return Err(LangError::ParserShouldHaveRejected), // The parser should have put the parameters in the form ExpressionList(expression_with_assignment, expression_with_assignment, ...)
            }
        }
        _ => Err(LangError::ExecuteNonFunction),
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn plus_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::SExpr(SExpression::Add(
            Box::new(Ast::Literal(Datatype::Number(3))),
            Box::new(Ast::Literal(Datatype::Number(6))),
        ));
        assert_eq!(Datatype::Number(9), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn string_plus_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::SExpr(SExpression::Add(
            Box::new(
                Ast::Literal(Datatype::String("Hello".to_string())),
            ),
            Box::new(
                Ast::Literal(Datatype::String(" World!".to_string())),
            ),
        ));
        assert_eq!(
            Datatype::String("Hello World!".to_string()),
            ast.evaluate(&mut map).unwrap()
        )
    }

    #[test]
    fn minus_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::SExpr(SExpression::Subtract(
            Box::new(Ast::Literal(Datatype::Number(6))),
            Box::new(Ast::Literal(Datatype::Number(3))),
        ));
        assert_eq!(Datatype::Number(3), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn minus_negative_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::SExpr(SExpression::Subtract(
            Box::new(Ast::Literal(Datatype::Number(3))),
            Box::new(Ast::Literal(Datatype::Number(6))),
        ));
        assert_eq!(Datatype::Number(-3), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn multiplication_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::SExpr(SExpression::Multiply(
            Box::new(Ast::Literal(Datatype::Number(6))),
            Box::new(Ast::Literal(Datatype::Number(3))),
        ));
        assert_eq!(Datatype::Number(18), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn division_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::SExpr(SExpression::Divide(
            Box::new(Ast::Literal(Datatype::Number(6))),
            Box::new(Ast::Literal(Datatype::Number(3))),
        ));
        assert_eq!(Datatype::Number(2), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn integer_division_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::SExpr(SExpression::Divide(
            Box::new(Ast::Literal(Datatype::Number(5))),
            Box::new(Ast::Literal(Datatype::Number(3))),
        ));
        assert_eq!(Datatype::Number(1), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn division_by_zero_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::SExpr(SExpression::Divide(
            Box::new(Ast::Literal(Datatype::Number(5))),
            Box::new(Ast::Literal(Datatype::Number(0))),
        ));
        assert_eq!(
            LangError::DivideByZero,
            ast.evaluate(&mut map).err().unwrap()
        )
    }

    #[test]
    fn modulo_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::SExpr(SExpression::Modulo(
            Box::new(Ast::Literal(Datatype::Number(8))),
            Box::new(Ast::Literal(Datatype::Number(3))),
        ));
        assert_eq!(Datatype::Number(2), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn equality_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::SExpr(SExpression::Equals(
            Box::new(Ast::Literal(Datatype::Number(3))),
            Box::new(Ast::Literal(Datatype::Number(3))),
        ));
        assert_eq!(Datatype::Bool(true), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn greater_than_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::SExpr(SExpression::GreaterThan(
            Box::new(Ast::Literal(Datatype::Number(4))),
            Box::new(Ast::Literal(Datatype::Number(3))),
        ));
        assert_eq!(Datatype::Bool(true), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn less_than_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::SExpr(SExpression::LessThan(
            Box::new(Ast::Literal(Datatype::Number(2))),
            Box::new(Ast::Literal(Datatype::Number(3))),
        ));
        assert_eq!(Datatype::Bool(true), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn greater_than_or_equal_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::SExpr(SExpression::GreaterThanOrEqual(
            Box::new(Ast::Literal(Datatype::Number(4))),
            Box::new(Ast::Literal(Datatype::Number(3))),
        ));
        assert_eq!(Datatype::Bool(true), ast.evaluate(&mut map).unwrap());

        let mut map: HashMap<String, Datatype> = HashMap::new();
        let equals_ast = Ast::SExpr(SExpression::GreaterThanOrEqual(
            Box::new(Ast::Literal(Datatype::Number(5))),
            Box::new(Ast::Literal(Datatype::Number(5))),
        ));
        assert_eq!(Datatype::Bool(true), equals_ast.evaluate(&mut map).unwrap());
    }

    #[test]
    fn less_than_or_equal_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::SExpr(SExpression::LessThanOrEqual(
            Box::new(Ast::Literal(Datatype::Number(2))),
            Box::new(Ast::Literal(Datatype::Number(3))),
        ));
        assert_eq!(Datatype::Bool(true), ast.evaluate(&mut map).unwrap());

        let mut map: HashMap<String, Datatype> = HashMap::new();
        let equals_ast = Ast::SExpr(SExpression::LessThanOrEqual(
            Box::new(Ast::Literal(Datatype::Number(5))),
            Box::new(Ast::Literal(Datatype::Number(5))),
        ));
        assert_eq!(Datatype::Bool(true), equals_ast.evaluate(&mut map).unwrap());
    }

    /// Assign the value 6 to the identifier "a"
    /// Recall that identifier and add it to 5
    #[test]
    fn assignment_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::ExpressionList(vec![
            Ast::SExpr(SExpression::Assignment {
                identifier: Box::new(Ast::ValueIdentifier("a".to_string())),
                ast: Box::new(Ast::Literal(Datatype::Number(6))),
            }),
            Ast::SExpr(SExpression::Add(
                Box::new(Ast::ValueIdentifier("a".to_string())),
                Box::new(Ast::Literal(Datatype::Number(5))),
            )),
        ]);
        assert_eq!(Datatype::Number(11), ast.evaluate(&mut map).unwrap())
    }


    /// Assign the value 6 to "a".
    /// Copy the value in "a" to "b".
    /// Recall the value in "b" and add it to 5.
    #[test]
    fn variable_copy_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::ExpressionList(vec![
            Ast::SExpr(SExpression::Assignment {
                identifier: Box::new(Ast::ValueIdentifier("a".to_string())),
                ast: Box::new(Ast::Literal(Datatype::Number(6))),
            }),
            Ast::SExpr(SExpression::Assignment {
                identifier: Box::new(Ast::ValueIdentifier("b".to_string())),
                ast: Box::new(Ast::ValueIdentifier("a".to_string())),
            }),
            Ast::SExpr(SExpression::Add(
                Box::new(Ast::ValueIdentifier("b".to_string())),
                Box::new(Ast::Literal(Datatype::Number(5))),
            )),
        ]);
        assert_eq!(Datatype::Number(11), ast.evaluate(&mut map).unwrap())
    }

    /// Assign the value 6 to a.
    /// Assign the value 3 to a.
    /// Recall the value in a and add it to 5, the value of a should be 3, equalling 8.
    #[test]
    fn reassignment_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::ExpressionList(vec![
            Ast::SExpr(SExpression::Assignment {
                identifier: Box::new(Ast::ValueIdentifier("a".to_string())),
                ast: Box::new(Ast::Literal(Datatype::Number(6))),
            }),
            Ast::SExpr(SExpression::Assignment {
                identifier: Box::new(Ast::ValueIdentifier("a".to_string())),
                ast: Box::new(Ast::Literal(Datatype::Number(3))),
            }),
            Ast::SExpr(SExpression::Add(
                Box::new(Ast::ValueIdentifier("a".to_string())),
                Box::new(Ast::Literal(Datatype::Number(5))),
            )),
        ]);
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
        let ast = Ast::ExpressionList(vec![
            Ast::SExpr(SExpression::Assignment {
                identifier: Box::new(Ast::ValueIdentifier("a".to_string())),
                ast: Box::new(Ast::Literal(Datatype::Function {
                    parameters: Box::new(Ast::ExpressionList(vec![])),
                    // empty parameters
                    body: (Box::new(Ast::Literal(Datatype::Number(32)))),
                    // just return a number
                    return_type: Box::new(Ast::Type(TypeInfo::Number)),
                        // expect a number
                })),
            }),
            Ast::SExpr(SExpression::ExecuteFn {
                identifier: Box::new(Ast::ValueIdentifier("a".to_string())),
                // get the identifier for a
                parameters: Box::new(Ast::ExpressionList(vec![])),
                    // provide the function parameters
            }),
        ]);
        assert_eq!(Datatype::Number(32), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn function_with_parameter_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::ExpressionList(vec![
            Ast::SExpr(SExpression::Assignment {
                identifier: Box::new(Ast::ValueIdentifier("a".to_string())),
                ast: Box::new(Ast::Literal(Datatype::Function {
                    parameters: Box::new(Ast::ExpressionList(vec![
                        Ast::SExpr(SExpression::TypeAssignment {
                            identifier: Box::new(Ast::ValueIdentifier("b".to_string())),
                            type_info: Box::new(Ast::Type(TypeInfo::Number)),
                        }),
                    ])),
                    body: (Box::new(Ast::ValueIdentifier("b".to_string()))), // just return the number passed in.
                    return_type: Box::new(Ast::Type(TypeInfo::Number)), // expect a number to be returned
                })),
            }),
            Ast::SExpr(SExpression::ExecuteFn {
                identifier: Box::new(Ast::ValueIdentifier("a".to_string())),
                // get the identifier for a
                parameters: Box::new(Ast::ExpressionList(vec![Ast::Literal(Datatype::Number(7))])),
            }),
        ]);
        assert_eq!(Datatype::Number(7), ast.evaluate(&mut map).unwrap())
    }


    #[test]
    fn function_with_two_parameters_addition_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::ExpressionList(vec![
            Ast::SExpr(SExpression::Assignment {
                identifier: Box::new(Ast::ValueIdentifier("add_two_numbers".to_string())),
                ast: Box::new(Ast::Literal(Datatype::Function {
                    parameters: Box::new(Ast::ExpressionList(vec![
                        Ast::SExpr(SExpression::TypeAssignment {
                            identifier: Box::new(Ast::ValueIdentifier("b".to_string())),
                            // the value's name is b
                            type_info: Box::new(Ast::Type(TypeInfo::Number)),
                                    // fn takes a number
                        }),
                        Ast::SExpr(SExpression::TypeAssignment {
                            identifier: Box::new(Ast::ValueIdentifier("c".to_string())),
                            // the value's name is b
                            type_info: Box::new(Ast::Type(TypeInfo::Number)),
                                    // fn takes a number
                        }),
                    ])),
                    body: (Box::new(Ast::SExpr(SExpression::Add(
                        Box::new(Ast::ValueIdentifier("b".to_string())),
                        Box::new(Ast::ValueIdentifier("c".to_string())),
                    )))),
                    // just return the number passed in.
                    return_type: Box::new(Ast::Type(TypeInfo::Number)),
                        // expect a number to be returned
                })),
            }),
            Ast::SExpr(SExpression::ExecuteFn {
                identifier: Box::new(Ast::ValueIdentifier("add_two_numbers".to_string())),
                // get the identifier for a
                parameters: Box::new(Ast::ExpressionList(vec![
                    Ast::Literal(Datatype::Number(7)),
                    Ast::Literal(Datatype::Number(5)),
                ])),
            }),
        ]);
        assert_eq!(Datatype::Number(12), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn array_access_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast: Ast = Ast::SExpr(SExpression::AccessArray {
            identifier: Box::new(Ast::Literal(Datatype::Array {
                value: vec![
                    Datatype::Number(12),
                    Datatype::Number(14),
                    Datatype::Number(16),
                ],
                type_: TypeInfo::Number,
            })),
            index: Box::new(Ast::Literal(Datatype::Number(0))), // get the first element
        });
        assert_eq!(Datatype::Number(12), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn array_incorrect_access_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast: Ast = Ast::SExpr(SExpression::AccessArray {
            identifier: Box::new(Ast::Literal(Datatype::Array {
                value: vec![
                    Datatype::Number(12),
                    Datatype::Number(14),
                    Datatype::Number(16),
                ],
                type_: TypeInfo::Number,
            })),
            index: Box::new(Ast::Literal(Datatype::Number(3))), // Array size 3. 0, 1, 2 hold elements. Index 3 doesn't.
        });
        assert_eq!(
            LangError::OutOfBoundsArrayAccess,
            ast.evaluate(&mut map).unwrap_err()
        )
    }

    #[test]
    fn struct_declaration_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast: Ast = Ast::SExpr(SExpression::StructDeclaration {
            identifier: Box::new(Ast::ValueIdentifier("MyStruct".to_string())),
            struct_type_info: Box::new(Ast::ExpressionList(vec![
                Ast::SExpr(SExpression::TypeAssignment {
                    identifier: Box::new(Ast::ValueIdentifier("Field1".to_string())),
                    type_info: Box::new(Ast::Type(TypeInfo::Number)),
                }),
            ])),
        });

        let _ = ast.evaluate(&mut map); // execute the ast to add the struct entry to the global stack map.
        let mut expected_map = HashMap::new();
        let mut inner_struct_hash_map = HashMap::new();
        inner_struct_hash_map.insert("Field1".to_string(), TypeInfo::Number);
        expected_map.insert(
            "MyStruct".to_string(),
            Datatype::StructType(TypeInfo::Struct { map: inner_struct_hash_map }),
        );
        assert_eq!(expected_map, map)
    }


    #[test]
    fn struct_creation_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let declaration_ast: Ast = Ast::SExpr(SExpression::StructDeclaration {
            identifier: Box::new(Ast::ValueIdentifier("MyStruct".to_string())),
            struct_type_info: Box::new(Ast::ExpressionList(vec![
                Ast::SExpr(SExpression::TypeAssignment {
                    identifier: Box::new(Ast::ValueIdentifier("Field1".to_string())),
                    type_info: Box::new(Ast::Type(TypeInfo::Number)),
                }),
            ])),
        });
        let _ = declaration_ast.evaluate(&mut map); // execute the ast to add the struct entry to the global stack map.

        let creation_ast: Ast = Ast::SExpr(SExpression::CreateStruct {
            identifier: Box::new(Ast::ValueIdentifier("MyStruct".to_string())),
            struct_datatype: Box::new(Ast::ExpressionList(vec![
                Ast::SExpr(SExpression::FieldAssignment {
                    identifier: Box::new(Ast::ValueIdentifier("Field1".to_string())),
                    ast: Box::new(Ast::Literal(Datatype::Number(8))),
                        // assign 8 to field Field1
                }),
            ])),
        });

        let struct_instance = creation_ast.evaluate(&mut map).unwrap();

        let mut inner_struct_hash_map = HashMap::new();
        inner_struct_hash_map.insert("Field1".to_string(), Datatype::Number(8));

        assert_eq!(
            Datatype::Struct { map: inner_struct_hash_map },
            struct_instance
        )
    }


    #[test]
    fn function_hoisting_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let ast = Ast::ExpressionList(vec![
            Ast::SExpr(SExpression::Assignment {
                identifier: Box::new(Ast::ValueIdentifier("a".to_string())),
                ast: Box::new(Ast::Literal(Datatype::Number(6))),
            }),
            Ast::SExpr(SExpression::CreateFunction {
                identifier: Box::new(Ast::ValueIdentifier("fn".to_string())),
                function_datatype: Box::new(Ast::Literal(Datatype::Function {
                    parameters: Box::new(Ast::ExpressionList(vec![])),
                    // empty parameters
                    body: (Box::new(Ast::Literal(Datatype::Number(32)))),
                    // just return a number
                    return_type: Box::new(Ast::Type(TypeInfo::Number)),
                        // expect a number
                })),
            }),
            Ast::SExpr(SExpression::ExecuteFn {
                identifier: Box::new(Ast::ValueIdentifier("fn".to_string())),
                // get the identifier for a
                parameters: Box::new(Ast::ExpressionList(vec![])),
                    // provide the function parameters
            }),
        ]);

        let hoisted_ast: Ast = ast.hoist_functions_and_structs();

        let expected_hoisted_ast: Ast = Ast::ExpressionList(vec![
            Ast::SExpr(SExpression::CreateFunction {
                identifier: Box::new(Ast::ValueIdentifier("fn".to_string())),
                function_datatype: Box::new(Ast::Literal(Datatype::Function {
                    parameters: Box::new(Ast::ExpressionList(vec![])),
                    // empty parameters
                    body: (Box::new(Ast::Literal(Datatype::Number(32)))),
                    // just return a number
                    return_type: Box::new(Ast::Type(TypeInfo::Number)),
                        // expect a number
                })),
            }),
            Ast::SExpr(SExpression::Assignment {
                identifier: Box::new(Ast::ValueIdentifier("a".to_string())),
                ast: Box::new(Ast::Literal(Datatype::Number(6))),
            }),
            Ast::SExpr(SExpression::ExecuteFn {
                identifier: Box::new(Ast::ValueIdentifier("fn".to_string())),
                // get the identifier for a
                parameters: Box::new(Ast::ExpressionList(vec![])),
                    // provide the function parameters
            }),
        ]);

        assert_eq!(hoisted_ast, expected_hoisted_ast);
        assert_eq!(
            Datatype::Number(32),
            hoisted_ast.evaluate(&mut map).unwrap()
        );
    }


}
