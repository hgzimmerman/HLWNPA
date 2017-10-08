
use std::collections::HashMap;
use ast::{Ast, SExpression, Datatype, TypeInfo, VariableStore};
use parser::program;
use std::rc::Rc;

use nom::IResult;

pub fn add_std_functions(map: &mut HashMap<String, Rc<Datatype>>) {
    add_print_function(map);
    add_println_function(map);
}


fn add_print_function(map: &mut VariableStore) {
    let ast: Ast = Ast::SExpr(SExpression::CreateFunction {
        identifier: Box::new(Ast::ValueIdentifier("print".to_string())),
        function_datatype: Box::new(Ast::Literal(Datatype::Function {
            parameters: Box::new(Ast::ExpressionList(vec![
                Ast::SExpr(SExpression::TypeAssignment {
                    identifier: Box::new(Ast::ValueIdentifier("to_print".to_string())),
                    type_info: Box::new(Ast::Type(TypeInfo::String)),
                }),
            ])),
            body: Box::new(Ast::ExpressionList(vec![
                Ast::SExpr(SExpression::Print(
                    Box::new(Ast::ValueIdentifier("to_print".to_string())),
                )),
            ])),
            return_type: Box::new(Ast::Type(TypeInfo::String)),
        })),
    });
    ast.evaluate(map).expect("Couldn't add print()");
}

fn add_println_function(map: &mut HashMap<String, Rc<Datatype>>) {

    // implement the println using the print function.
    let input_function = "
        fn println(x: String) -> String {
            let str := x + \"\n\"
            print(str)
        }
    ";
    match program(input_function.as_bytes()) {
        IResult::Done(_, ast) => {
            ast.evaluate(map).expect("Couldn't add println()");
        }
        IResult::Error(e) => {
            panic!(
                "Language internals do not support the syntax used to define the function. Error: {}",
                e
            )
        }
        IResult::Incomplete(i) => {
            panic!(
                "Parser does not support the syntax used to define the function. Incomplete Parse: {:?}",
                i
            )
        }
    }
}


#[test]
fn expect_print_function_to_be_added_to_global_map() {
    let mut map: VariableStore = VariableStore::new();
    add_print_function(&mut map);
    let mut expected_map: VariableStore = VariableStore::new();
    let print_fn: Datatype = Datatype::Function {
        parameters: (Box::new(Ast::ExpressionList(vec![
            Ast::SExpr(SExpression::TypeAssignment {
                identifier: Box::new(Ast::ValueIdentifier("to_print".to_string())),
                type_info: Box::new(Ast::Type(TypeInfo::String)),
            }),
        ]))),
        body: (Box::new(Ast::ExpressionList(vec![
            Ast::SExpr(SExpression::Print(
                Box::new(Ast::ValueIdentifier("to_print".to_string())),
            )),
        ]))),
        return_type: (Box::new(Ast::Type(TypeInfo::String))),
    };
    expected_map.insert("print".to_string(), Rc::new(print_fn));
    assert_eq!(expected_map, map);
}
