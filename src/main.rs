//#![feature(discriminant_value)]
#![feature(trace_macros)]
#![feature(test)]

#![macro_use]
extern crate nom;
extern crate test;
extern crate clap;

use clap::{Arg, App};
use nom::IResult;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;
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

    let matches = App::new("Toy Language")
        .version("0.1.0")
        .author("Henry Zimmerman")
        .about(
            "A toy language I made in a couple of days without thinking about it too much",
        )
        .arg(
            Arg::with_name("file")
                .value_name("File")
                .help("The file that you want to interpret")
                .takes_value(true),
        )
        .get_matches();


    match matches.value_of("file") {
        Some(filename) => {
            let mut file_contents: String = String::new();
            let file: File = OpenOptions::new().read(true).open(&filename).unwrap();
            let mut buf_reader = BufReader::new(&file);
            match buf_reader.read_to_string(&mut file_contents) {
                Ok(_) => {}
                Err(e) => eprintln!("Couldn't read the file {} because {}", filename, e),
            }

            match program(file_contents.as_bytes()) {
                IResult::Done(_, ast) => {
                    let mut map: HashMap<String, Datatype> = HashMap::new();
                    let program_return_value = ast.evaluate(&mut map);
                    println!("{:?}", program_return_value);
                }
                IResult::Error(e) => eprintln!("encountered an error while parsing the file: {:?}", e),
                IResult::Incomplete(i) => eprintln!("Couldn't parse all of the file: {:?}", i),
            }
        }
        None => repl(), // If a file to run wasn't provided, drop the user into a REPL
    }
}
