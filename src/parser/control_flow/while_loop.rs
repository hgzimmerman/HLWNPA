#[allow(unused_imports)]
use nom::*;
use ast::Ast;
use s_expression::SExpression;
use parser::body::body;
use std::boxed::Box;
use parser::expressions::sexpr;




named!(pub while_loop<Ast>,
    do_parse!(
        ws!(tag!("while")) >>
        while_conditional: ws!(sexpr) >>
        while_body: ws!(body) >>

        (Ast::SExpr(SExpression::Loop{
            conditional: Box::new(while_conditional),
            body: Box::new(while_body)
        }))
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
            Ast::SExpr(SExpression::Loop {
                conditional: Box::new(Ast::Literal(Datatype::Bool(true))),
                body: Box::new(Ast::ExpressionList(
                    vec![Ast::Literal(Datatype::Bool(true))],
                )),
            }),
            value
        )
    }

    #[test]
    fn parse_while_loop_2() {
        let input_string = "while (x > 5) { true }";
        let (_, value) = match while_loop(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("Error in parsing: {}", e),
            IResult::Incomplete(i) => panic!("Incomplete parse: {:?}", i),
        };

        assert_eq!(
            Ast::SExpr(SExpression::Loop {
                conditional: Box::new(Ast::SExpr(SExpression::GreaterThan(
                    Box::new(Ast::ValueIdentifier("x".to_string())),
                    Box::new(Ast::Literal(Datatype::Number(5))),
                ))),
                body: Box::new(Ast::ExpressionList(
                    vec![Ast::Literal(Datatype::Bool(true))],
                )),
            }),
            value
        )
    }
}
