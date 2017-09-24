pub mod number; // TODO move number_raw to other file/module so this can remove the "pub"
use self::number::number_literal;

mod string;
pub use self::string::string_literal;

mod boolean;
use self::boolean::bool_literal;

mod array;
use self::array::array_literal;

#[allow(unused_imports)]
use nom::*;
use ast::Ast;

/// put all literal types here
named!(pub literal<Ast>,
    alt!(
        complete!(array_literal) |
        complete!(number_literal) |
        complete!(string_literal) |
        complete!(bool_literal) |
        complete!(array_literal)
    )
);

// TODO create more tests for the literal combinator


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

        assert_eq!(Ast::Literal(Datatype::String("\nHello\nWorld\n".to_string())), ast)

    }