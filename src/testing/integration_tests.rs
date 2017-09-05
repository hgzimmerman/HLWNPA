#[cfg(test)]
mod test {
    use nom::IResult;
    use parser::program;
    //use parser::function::function;
//    use ast::*;
//    use std::boxed::Box;
    use std::collections::HashMap;
    use datatype::{Datatype};
    use test::Bencher;

    //#[test]
    //fn function_parse_and_execute_separately_integration_test() {
    //    use nom::IResult;
    //    let mut map: HashMap<String, Datatype> = HashMap::new();
    //
    //    let input_string = "fn add8ToValue ( a : Number ) -> Number { ( a + 8 ) }";
    //    let (_, ast_with_function) = match function(input_string.as_bytes()) {
    //        IResult::Done(rest, v) => (rest, v),
    //        IResult::Error(e) => panic!("{}", e),
    //        _ => panic!(),
    //    };
    //
    //    let _ = ast_with_function.evaluate(&mut map); // insert the function into the hashmap
    //
    //    let executor_ast: Ast = Ast::Expression {
    //        operator: BinaryOperator::ExecuteFn,
    //        expr1: Box::new(Ast::ValueIdentifier("add8ToValue".to_string())),
    //        expr2: Box::new(Ast::VecExpression {
    //            expressions: vec![Ast::Literal(Datatype::Number(7))],
    //        }),
    //    };
    //
    //    assert_eq!(
    //        Datatype::Number(15),
    //        executor_ast.evaluate(&mut map).unwrap()
    //    ); // find the test function and pass the value 7 into it
    //}


    #[test]
    fn program_parse_and_execute_integration_test_1() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = "
     let x := 7
     fn test_function ( a : Number ) -> Number { ( a + 8 ) }
     test_function(x)";
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(15), ast.evaluate(&mut map).unwrap());
    }


    #[test]
    fn program_parse_and_execute_integration_test_2() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = "
     fn test_function ( a : Number ) -> Number { ( a + 8 ) }
     test_function(8)";
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(16), ast.evaluate(&mut map).unwrap());
    }

    #[test]
    fn program_parse_and_execute_integration_test_3() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = "
     fn test_function ( a : Number ) -> Number { ( a + 8 ) }
     test_function( ( 6 + 2) )";
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(16), ast.evaluate(&mut map).unwrap());
    }

    /// Test multiple line functions
    #[test]
    fn program_parse_and_execute_integration_test_4() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = "
     fn test_function ( a : Number ) -> Number {
        ( a + 8 )
     }
     test_function(8)";
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(16), ast.evaluate(&mut map).unwrap());
    }

    #[test]
    fn program_multiple_parameter_function_integration_test() {
        use nom::IResult;
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = "
     fn add_two_numbers ( a : Number b : Number) -> Number {
        ( a + b )
     }
     add_two_numbers(8 3)";
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(11), ast.evaluate(&mut map).unwrap());
    }


    #[test]
    fn program_function_internals_does_not_clobber_outer_stack_integration_test() {
        use nom::IResult;
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = "
     let a := 2
     fn add_two_numbers ( a : Number b : Number) -> Number {
        ( a +  b )
     }
     add_two_numbers(8 3)
     a
     ";
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(2), ast.evaluate(&mut map).unwrap());
    }


    /// Test the assignment of a string, then passing it into a function that takes a string.
/// The function should then add a number to the string, creating a new string.
    #[test]
    fn program_string_coercion_integration_test() {
        use nom::IResult;
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = r##"
     let x := "Hi "
     fn test_function ( a : String ) -> String { ( a + 5 ) }
     test_function(x)"##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(
            Datatype::String("Hi 5".to_string()),
            ast.evaluate(&mut map).unwrap()
        );
    }


    #[test]
    fn program_if_test() {
        use nom::IResult;
        let mut map: HashMap<String, Datatype> = HashMap::new();
        // the while loop should increment the x once
        let input_string = r##"
     let x := 3
     if ( x == 3) {
        let x := 40
     }
     x"##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(40), ast.evaluate(&mut map).unwrap());
    }

    #[test]
    fn program_while_loop_test() {
        use nom::IResult;
        let mut map: HashMap<String, Datatype> = HashMap::new();
        // the while loop should reassign x to be something else;
        let input_string = r##"
     let x := 3
     while (x == 3) {
        let x := 40
     }
     x"##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(40), ast.evaluate(&mut map).unwrap());
    }

    #[test]
    fn program_while_loop_false_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        // the while body should not execute
        let input_string = r##"
     let x := 42
     while (x == 3) {
        let x := (x + 1)
     }
     x"##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(42), ast.evaluate(&mut map).unwrap());
    }

    #[bench]
    fn simple_program_bench(b: &mut Bencher) {
        use super::super::test_constants::SIMPLE_PROGRAM_INPUT_1;
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let (_, ast) = match program(SIMPLE_PROGRAM_INPUT_1.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        b.iter(|| {
            assert_eq!(Datatype::Number(15), ast.evaluate(&mut map).unwrap())
        })
    }

    #[bench]
    fn while_loop_parse_and_execute_program_bench(b: &mut Bencher) {
        fn loop_1000_times_program() {
            use nom::IResult;
            let mut map: HashMap<String, Datatype> = HashMap::new();
            let input_string = r##"
        let x := 0
        while (x < 1000) {
           let x := (x + 1)
        }
        x
        "##;
            let (_, ast) = match program(input_string.as_bytes()) {
                IResult::Done(rest, v) => (rest, v),
                IResult::Error(e) => panic!("{}", e),
                _ => panic!(),
            };

            assert_eq!(Datatype::Number(1000), ast.evaluate(&mut map).unwrap());
        }

        b.iter(|| loop_1000_times_program());
    }


    #[bench]
    fn while_loop_with_useless_conditionals_parse_and_execute_program_bench(b: &mut Bencher) {
        fn loop_1000_times_program() {
            use nom::IResult;
            let mut map: HashMap<String, Datatype> = HashMap::new();
            let input_string = r##"
        let x := 0
        while (x < 1000) {
            ( 1 * 3 )
            ( 1 * 40000 )
            ( 34234 % 7 )
            let x := (x + 1)
        }
        x
        "##;
            let (_, ast) = match program(input_string.as_bytes()) {
                IResult::Done(rest, v) => (rest, v),
                IResult::Error(e) => panic!("{}", e),
                _ => panic!(),
            };

            assert_eq!(Datatype::Number(1000), ast.evaluate(&mut map).unwrap());
        }

        b.iter(|| loop_1000_times_program());
    }
}