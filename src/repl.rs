
use nom::IResult;
use parser::program;
use lang_result::{LangResult, LangError};
use ast::Ast;
use datatype::Datatype;
use std::collections::HashMap;

use std::io;
use std::io::Write;

use std_functions;
use regex::Regex;

use preprocessor::preprocess;


/// Reads and parses
fn read<'a>(read_string: &'a str ) -> IResult<&'a [u8], Ast> {
    return program(read_string.as_bytes());
}

// Evaluates the AST
fn evaluate(
    possibly_parsed_ast: IResult<&[u8], Ast>,
    map: &mut HashMap<String, Datatype>,
) -> LangResult {

    match possibly_parsed_ast {
        IResult::Done(_, ast) => ast.evaluate(map),
        IResult::Error(e) => {
            print!("Invalid syntax: {}\nuser>", e);
            Err(LangError::InvalidSyntax)
        }
        IResult::Incomplete(i) => {
            print!("Invalid syntax. Parser returned incomplete: {:?}\nuser>", i);
            Err(LangError::InvalidSyntaxFailedToParse)
        }
    }
}

// Prints the result of the AST
fn print(possibly_evaluated_program: LangResult) {

    match possibly_evaluated_program {
        Ok(datatype) => print!("{:?}\nuser>", datatype),
        Err(err) => print!("{:?}\nuser>", err),
    }

    let _ = io::stdout().flush(); // print immediately
}

/// It is expected that the incoming map already has the std_functions added.
pub fn repl(mut map: &mut HashMap<String, Datatype>) {
    use std::io;
    use std::io::prelude::*;
    let stdin = io::stdin();

    print!("user>");
    let _ = io::stdout().flush();
    for line in stdin.lock().lines() {
        prep(&mut line.unwrap().as_str(), &mut map)
    }
}

/// Creates the map, adds standard functions to it and runs the repl with it.
pub fn create_repl() {
    let mut map: HashMap<String, Datatype> = HashMap::new();
    std_functions::add_std_functions(&mut map);

    repl(&mut map)
}


fn prep(a: &mut &str, map: &mut HashMap<String, Datatype>) {
    let preprocessed = preprocess(a);
    let parsed = read(preprocessed.as_str());
    let evaled = evaluate(parsed, map);
    print(evaled)
}
