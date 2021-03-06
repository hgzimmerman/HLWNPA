use ast::Ast;
use ast::lang_result::LangError;
use nom::IResult;
use parser::program;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::OpenOptions;
use std::error::Error;

pub fn read_file_into_ast(filename: String) -> Result<Ast, LangError> {
    match OpenOptions::new().read(true).open(&filename) {
        Ok(file) => {
            let mut file_contents: String = String::new();
            let mut buf_reader = BufReader::new(&file);
            match buf_reader.read_to_string(&mut file_contents) {
                Ok(_) => {}
                Err(e) => {
                    return Err(LangError::CouldNotReadFile {
                        filename: filename,
                        reason: e.description().to_string(),
                    })
                }
            }

            match program(file_contents.as_bytes()) {
                IResult::Done(_, ast) => return Ok(ast),
                IResult::Error(e) => Err(LangError::CouldNotParseFile {
                    filename: filename,
                    reason: e.description().to_string(),
                }),
                IResult::Incomplete(_) => Err(LangError::CouldNotParseFile {
                    filename: filename,
                    reason: "Incomplete parse".to_string(),
                }),
            }
        }
        Err(e) => {
            return Err(LangError::CouldNotReadFile {
                filename: filename,
                reason: e.description().to_string(),
            })
        }
    }
}
