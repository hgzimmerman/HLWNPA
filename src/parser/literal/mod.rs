mod number;
use self::number::number_literal;

mod string;
pub use self::string::string_literal;

mod boolean;
use self::boolean::bool_literal;

mod array;
use self::array::array_literal;

mod float;
use self::float::float_literal;

#[allow(unused_imports)]
use nom::*;
use ast::Ast;

/// put all literal types here
named!(pub literal<Ast>,
    alt!(
        complete!(array_literal) |
        complete!(float_literal) |
        complete!(number_literal) |
        complete!(string_literal) |
        complete!(bool_literal)
    )
);

// TODO create more tests for the literal combinator

#[cfg(test)]
mod test {
    use super::*;
    use datatype::Datatype;

    #[test]
    fn verify_literal_with_escapes_in_strings() {
        let input_string = "
        \"\nHello\nWorld\n\"
        ";
        let (_, ast) = match literal(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };

        assert_eq!(
            Ast::Literal(Datatype::String("\nHello\nWorld\n".to_string())),
            ast
        )
    }


    #[test]
    fn literal_captures_float_before_number() {
        let input_string = "40.5";
        let (_, ast) = match literal(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };

        assert_eq!(
            Ast::Literal(Datatype::Float(40.5)),
            ast
        )
    }

    #[test]
    fn literal_can_capture_number() {
        let input_string = "40";
        let (_, ast) = match literal(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };

        assert_eq!(
            Ast::Literal(Datatype::Number(40)),
            ast
        )
    }

    #[test]
    fn space_in_float_causes_number_parse_instead() {
        let input_string = "40. 5";
        let (_, ast) = match literal(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };
        assert_eq!(
            Ast::Literal(Datatype::Number(40)),
            ast
        )
    }
}
