#[allow(unused_imports)]
use nom::*;
use ast::Ast;
use s_expression::SExpression;
use parser::body::body;
use std::boxed::Box;
use parser::expressions::sexpr;
use parser::identifier::identifier;
use datatype::Datatype;
use uuid::Uuid;
use uuid::UuidVersion;

named!(pub for_loop<Ast>,
    do_parse!(
        ws!(tag!("for")) >>
        variable: ws!(identifier) >>
        ws!(tag!("in")) >>
        array: ws!(sexpr) >>
        for_body: ws!(body) >>

        ( create_for_loop(variable, array, for_body) )
    )
);

fn create_for_loop(identifier: Ast, array: Ast, for_body: Ast) -> Ast {
    // Create a unique value to hold the index that should never collide if this is called repeatedly.
    let index_uuid: String = Uuid::new(UuidVersion::Random).unwrap().hyphenated().to_string();
    let length_uuid: String = Uuid::new(UuidVersion::Random).unwrap().hyphenated().to_string();
    //Depending if the array is an identifier, or if some significant amount of computation is required to produce an array,
    // optimize the AST to not not require a double-identifier lookup.
    match array {
        Ast::ValueIdentifier(array_id) => {
            Ast::ExpressionList(vec![
                Ast::SExpr(SExpression::Assignment {
                    identifier: Box::new(Ast::ValueIdentifier(index_uuid.clone())),
                    ast: Box::new(Ast::Literal(Datatype::Number(0))) // 0 index
                }),


                Ast::SExpr(SExpression::Assignment {
                    identifier: Box::new(Ast::ValueIdentifier(length_uuid.clone())),
                    ast: Box::new(Ast::SExpr(SExpression::GetArrayLength(Box::new(Ast::ValueIdentifier(array_id.clone())))))
                }),
                Ast::SExpr(SExpression::Loop {
                    conditional: Box::new(Ast::SExpr(SExpression::LessThan (
                        Box::new(Ast::ValueIdentifier(index_uuid.clone())),
                        Box::new( Ast::ValueIdentifier(length_uuid))
                    ) )),
                    body: Box::new(Ast::ExpressionList(vec![
                        Ast::SExpr(SExpression::Assignment {
                            identifier: Box::new(identifier),
                            ast: Box::new(Ast::SExpr(SExpression::AccessArray {
                                identifier: Box::new(Ast::ValueIdentifier(array_id)), // Because we already have an identifier, we can just use it here. No need to declare another variable earlier
                                index: Box::new(Ast::ValueIdentifier(index_uuid.clone())),
                            }))
                        }), // Assign the value at the index to the given identifier
                        for_body, // execute the specified body
                        Ast::SExpr(
                            SExpression::Assignment {
                                identifier: Box::new(Ast::ValueIdentifier(index_uuid.clone())),
                                ast: Box::new(Ast::SExpr(SExpression::Increment(Box::new(Ast::ValueIdentifier(index_uuid)))))
                            }
                        ) // increment the index
                    ]))
                })
            ])
        }
        _ => {
            // Use this uuid to store the creation of the array. This way, the array can be created once, instead of being created in the conditional
            let array_uuid: String = Uuid::new(UuidVersion::Random).unwrap().hyphenated().to_string();

            let length_uuid: String = Uuid::new(UuidVersion::Random).unwrap().hyphenated().to_string();
            Ast::ExpressionList(vec![
                Ast::SExpr(SExpression::Assignment {
                    identifier: Box::new(Ast::ValueIdentifier(index_uuid.clone())),
                    ast: Box::new(Ast::Literal(Datatype::Number(0))) // 0 index
                }),
                // Hide the Array behind this variable, so accessing it incurs a constant cost
                // (if we initialize an array then iterate through it, we only create it once,
                // instead of creating a new array for every loop iteration like a prior implementation in all cases except
                // you guessed it, by accessing it through an id.)
                Ast::SExpr(SExpression::Assignment {
                    identifier: Box::new(Ast::ValueIdentifier(array_uuid.clone())),
                    ast: Box::new(array)
                }),

                Ast::SExpr(SExpression::Assignment {
                    identifier: Box::new(Ast::ValueIdentifier(length_uuid.clone())),
                    ast: Box::new(Ast::SExpr(SExpression::GetArrayLength(Box::new(Ast::ValueIdentifier(array_uuid.clone())))))
                }),

                Ast::SExpr(SExpression::Loop {
                    conditional: Box::new(Ast::SExpr(SExpression::LessThan (
                        Box::new(Ast::ValueIdentifier(index_uuid.clone())),
                        Box::new(Ast::ValueIdentifier(length_uuid))
                    ) )),
                    body: Box::new(Ast::ExpressionList(vec![
                        Ast::SExpr(SExpression::Assignment {
                            identifier: Box::new(identifier),
                            ast: Box::new(Ast::SExpr(SExpression::AccessArray {
                                identifier: Box::new(Ast::ValueIdentifier(array_uuid)), // Use the identifier to the initialized array that was created earlier
                                index: Box::new(Ast::ValueIdentifier(index_uuid.clone())),
                            }))
                        }), // Assign the value at the index to the given identifier
                        for_body, // execute the specified body
                        Ast::SExpr(
                            SExpression::Assignment {
                                identifier: Box::new(Ast::ValueIdentifier(index_uuid.clone())),
                                ast: Box::new(Ast::SExpr(SExpression::Increment(Box::new(Ast::ValueIdentifier(index_uuid)))))
                            }
                        ) // increment the index
                    ]))
                })
            ])
        }
    }
}

#[cfg(test)]
mod test {
//    use std::rc::Rc;
    use super::*;
//    use datatype::TypeInfo;

    #[test]
    fn for_loop_parse() {
        let input_string = r#"
        for i in [0,2] {
            3
        }
         "#;
        let (_, /*ast*/_) = match for_loop(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };



//        let expected_ast = create_for_loop(
//            Ast::ValueIdentifier("i".to_string()),
//            Ast::Literal(Datatype::Array {
//                value: vec![Rc::new(Datatype::Number(0)), Rc::new(Datatype::Number(2))],
//                type_: TypeInfo::Number
//            }),
//            Ast::ExpressionList(vec![Ast::Literal(Datatype::Number(3))])
//        );
        // Can't test this because of the random uuids used for the value identifiers.

        //        assert_eq!(expected_ast, ast);
    }
}
