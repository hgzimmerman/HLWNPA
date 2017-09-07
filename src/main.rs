//#![feature(discriminant_value)]
#![feature(trace_macros)]
#![feature(test)]
#![recursion_limit="100"]

#![macro_use]
extern crate nom;
extern crate test;
extern crate clap;

use clap::{Arg, App};
use nom::IResult;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::OpenOptions;

mod datatype;
mod lang_result;
mod ast;
mod parser;
mod repl;
#[cfg(test)]
mod testing;

use datatype::{Datatype};
use ast::*;
use repl::repl;

use parser::program;


fn main() {

    let matches = App::new("HLWNPA - Henry's Language With No Pronounceable Acronym")
        .version("0.1.1")
        .author("Henry Zimmerman")
        .about(
            "A toy language I made in a couple of days without thinking about it too much.",
        )
        .arg(
            Arg::with_name("file")
                .value_name("File")
                .help("The file that you want to interpret. If nothing is provided, you will be dropped into a REPL.")
                .takes_value(true),
        )
        .get_matches();


    match matches.value_of("file") {
        Some(filename) => {
            // read the file into a string, parse it, and execute the resulting AST

            match OpenOptions::new().read(true).open(&filename) {
                Ok(file) => {
                    let mut file_contents: String = String::new();
                    let mut buf_reader = BufReader::new(&file);
                    match buf_reader.read_to_string(&mut file_contents) {
                        Ok(_) => {}
                        Err(e) => eprintln!("Couldn't read the file {} because: {}", filename, e),
                    }

                    match program(file_contents.as_bytes()) {
                        IResult::Done(_, ast) => {
                            let mut map: HashMap<String, Datatype> = HashMap::new();
                            let program_return_value = ast.evaluate(&mut map);
                            match program_return_value {
                                Ok(ok_value) => println!("{:?}", ok_value),
                                Err(e) => println!("{:?}", e)
                            }

                        }
                        IResult::Error(e) => eprintln!("encountered an error while parsing the file: {:?}", e),
                        IResult::Incomplete(i) => eprintln!("Couldn't parse all of the file: {:?}", i),
                    }
                }
                Err(e) => eprintln!("Couldn't open file because: {}", e)
            }
        }
        None => repl(), // If a file to run wasn't provided, drop the user into a REPL
    }
}
