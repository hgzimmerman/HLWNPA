#[allow(unused_imports)]
use nom::*;
use ast::{Ast, SExpression};
use parser::body::body;
use parser::utilities::expression_or_literal_or_identifier_or_struct_or_array;
use std::boxed::Box;




named!(pub while_loop<Ast>,
    do_parse!(
        ws!(tag!("while")) >>
        while_conditional: ws!(expression_or_literal_or_identifier_or_struct_or_array) >>
        while_body: ws!(body) >>

        (Ast::SExpr(Box::new(SExpression::Loop{
            conditional: Box::new(while_conditional),
            body: Box::new(while_body)
        })))
    )
);

#[cfg(test)]
mod test {
    use super::*;
    use datatype::Datatype;

    #[test]
    fn parse_while_loop_test() {
        let input_string = "while true { true }";
        let (_, value) = match while_loop(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };

        assert_eq!(
            Ast::SExpr(Box::new(
                SExpression::Loop {
                   conditional: Box::new(Ast::Literal(Datatype::Bool(true))),
                   body: Box::new(Ast::VecExpression {
                       expressions: vec![
                           Ast::Literal(Datatype::Bool(true))
                       ]
                   }),
                }
            )),
            value
        )
    }
}
