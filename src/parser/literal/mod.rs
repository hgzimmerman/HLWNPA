mod number;
use self::number::number_literal;

mod string;
pub use self::string::string_literal;

mod boolean;
use self::boolean::bool_literal;

mod array;
use self::array::*;

mod float;
use self::float::float_literal;

#[allow(unused_imports)]
use nom::*;
use ast::Ast;

/// put all literal types here
named!(pub literal<Ast>,
    alt_complete!(
        array_literal |
        array_range |
        float_literal |
        number_literal |
        string_literal |
        bool_literal
    )
);


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

    // This behavior should cause the consuming parser to fail.
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
