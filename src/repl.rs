
use nom::IResult;
use parser::program;
use ast::{Ast, LangResult, LangError};
use ast::datatype::{Datatype, VariableStore};
use std::collections::HashMap;

use std::io;
use std::io::Write;

use std_functions;
use std::rc::Rc;
use preprocessor::preprocess;
use ast::mutability::MutabilityMap;
use ast::type_checking::TypeStore;


/// Because the program parser will put all top level expressions in a list,
/// the mutability checker will be given a new scope for every command entered in at
/// the command line due to the way it will copy the mutability map for every new scope.
///
/// This moves the singular element out of the list and allows the mutability checker to work in
/// the REPL.
fn replace_top_level_list_with_its_constituent_element( ast: &Ast ) -> &Ast {
    if let Ast::ExpressionList(ref expressions) = *ast {
        if expressions.len() == 1 {
            expressions.get(0).unwrap()
        } else {
            ast
        }
    } else {
        ast
    }

}


/// Reads and parses
fn read<'a>(read_string: &'a str) -> IResult<&'a [u8], Ast> {
    if read_string == "" {
        return IResult::Done(b"", Ast::Literal(Datatype::None))
    }
    return program(read_string.as_bytes());
}

// Evaluates the AST
fn evaluate(
    ast: Ast,
    map: &mut VariableStore,
    mutability_map: &mut MutabilityMap,
    type_store: &mut TypeStore
) -> LangResult {

    let ast = replace_top_level_list_with_its_constituent_element(&ast);
    if let Err(error) = ast.check_mutability_semantics(mutability_map) {
        println!("{:?}", error);
        Err(LangError::MutabilityRulesViolated)
    } else {
        match ast.check_types(type_store) {
            Ok(_) => ast.evaluate(map),
            Err(type_error) => Err(LangError::NewTypeError(type_error))
        }
    }
}

/// Prints the result of the AST
fn print(possibly_evaluated_program: LangResult) {

    match possibly_evaluated_program {
        Ok(datatype) => print!("{:?}\nuser>", datatype),
        Err(err) => print!("{:?}\nuser>", err),
    }

    let _ = io::stdout().flush(); // print immediately
}

/// It is expected that the incoming map already has the std_functions added.
pub fn repl(mut map: &mut VariableStore, mut mutability_map: &mut MutabilityMap, mut type_store: &mut TypeStore) {
    use std::io;
    use std::io::prelude::*;
    let stdin = io::stdin();

    print!("user>");
    let _ = io::stdout().flush();
    for line in stdin.lock().lines() {
        prep(&mut line.unwrap().as_str(), &mut map, &mut mutability_map, type_store)
    }
}



/// Creates the map, adds standard functions to it and runs the repl with it.
pub fn create_repl() {
    let mut map: VariableStore = VariableStore::new();
    let mut mutability_map: MutabilityMap = MutabilityMap::new();
    let mut type_store: TypeStore = TypeStore::new();
    std_functions::add_std_functions(&mut map);

    repl(&mut map, &mut mutability_map, &mut type_store)
}


/// Preprocess, Parse, Evaluate, Print.
fn prep(a: &mut &str, map: &mut VariableStore, mutability_map: &mut MutabilityMap, type_store: &mut TypeStore ) {
    let preprocessed = preprocess(a);
    let parsed = read(preprocessed.as_str());

    match parsed {
        IResult::Done(_, ast) => {
            let evaled = evaluate(ast, map, mutability_map, type_store);
            print(evaled)
        },
        IResult::Error(e) => {
            print!("Invalid syntax: {}\nuser>", e);
        }
        IResult::Incomplete(i) => {
            print!("Invalid syntax. Parser returned incomplete: {:?}\nuser>", i);
        }
    }


}
