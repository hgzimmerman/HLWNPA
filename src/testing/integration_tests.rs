#[cfg(test)]
mod test {
    use nom::IResult;
    use parser::program;
    use std::collections::HashMap;
    use datatype::{Datatype, TypeInfo};
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
    //        expr2: Box::new(Ast::ExpressionList {
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
     fn test_function ( a : Number ) -> Number {  a + 8  }
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
     fn test_function ( a : Number ) -> Number { a + 8  }
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
     fn test_function ( a : Number ) -> Number { a + 8 }
     test_function( 6 + 2 )";
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
         a + 8
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
     fn add_two_numbers ( a : Number, b : Number) -> Number {
         a + b
     }
     add_two_numbers(8, 3)";
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
     fn add_two_numbers ( a : Number, b : Number) -> Number {
        let a := a +  b
        a
     }
     add_two_numbers(8, 3)
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
     fn test_function ( a : String ) -> String {  a + 5  }
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
     if x == 3 {
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
     while x == 3 {
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
        while x == 3 {
           let x := x + 1
        }
        x"##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(42), ast.evaluate(&mut map).unwrap());
    }

    #[test]
    fn program_parse_literal_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        // the while body should not execute
        let input_string = r##"
        32
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(32), ast.evaluate(&mut map).unwrap());
    }

    #[test]
    fn program_parse_and_verify_array_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = r##"
        [23, 43, 11]
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(
            Datatype::Array {
                value: vec![
                    Datatype::Number(23),
                    Datatype::Number(43),
                    Datatype::Number(11),
                ],
                type_: TypeInfo::Number,
            },
            ast.evaluate(&mut map).unwrap()
        );
    }

    #[test]
    fn program_parse_struct_and_something_after_it() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = r##"
        struct MyStruct {
            a : Number
        }
        3 + 3

        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(6), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn program_parse_struct_and_access_field() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = r##"
        struct MyStruct {
            a : Number
        }
        let instance := new MyStruct {
            a: 8
        }

        instance.a

        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(8), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn program_parse_struct_and_access_field_via_assignment() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = r##"
        struct MyStruct {
            a : Number
        }
        let instance := new MyStruct {
            a: 8
        }

        let value_from_struct := instance.a
        value_from_struct

        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(8), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn program_parse_struct_with_multiple_fields_and_access_fields_in_expression() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = r##"
        struct MyStruct {
            a : Number
            b : Number
        }
        let instance := new MyStruct {
            a: 8
            b: 10
        }

        instance.a + instance.b * instance.b

        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(180), ast.evaluate(&mut map).unwrap())
    }


    #[test]
    fn program_parse_struct_with_multiple_fields_and_access_fields_in_function() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = r##"
        struct MyStruct {
            a : Number
            b : Number
        }
        let instance := new MyStruct {
            a: 8
            b: 10
        }

        fn addContents( s: MyStruct ) -> Number {
            s.a + s.b
        }

        addContents( instance )

        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

        assert_eq!(Datatype::Number(18), ast.evaluate(&mut map).unwrap())
    }

    #[test]
fn program_parse_struct_with_multiple_fields_and_return_struct_from_function_with_internal_assignment(){
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = r##"
        struct MyStruct {
            a : Number
            b : Number
        }

        fn create_new_MyStruct( value: Number ) -> MyStruct {
            let c := new MyStruct {
                a: 8
                b: value
            }
            c
        }
        create_new_MyStruct( 3 )
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };
        let mut struct_map: HashMap<String, Datatype> = HashMap::new();
        struct_map.insert("a".to_string(), Datatype::Number(8));
        struct_map.insert("b".to_string(), Datatype::Number(3));
        assert_eq!(
            Datatype::Struct { map: struct_map },
            ast.evaluate(&mut map).unwrap()
        )
    }

    #[test]
    fn program_parse_struct_with_multiple_fields_and_return_struct_from_function() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = r##"
        struct MyStruct {
            a : Number
            b : Number
        }

        fn create_new_MyStruct( value: Number ) -> MyStruct {
            new MyStruct {
                a: 8
                b: value
            }
        }
        create_new_MyStruct( 3 )
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };
        let mut struct_map: HashMap<String, Datatype> = HashMap::new();
        struct_map.insert("a".to_string(), Datatype::Number(8));
        struct_map.insert("b".to_string(), Datatype::Number(3));
        assert_eq!(
            Datatype::Struct { map: struct_map },
            ast.evaluate(&mut map).unwrap()
        )
    }

    #[test]
    fn program_verify_that_struct_maps_dont_interfere_with_global_stack_map() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = r##"
        struct MyStruct {
            a : Number
            b : Number
        }

        let a := 3

        fn create_new_MyStruct( value: Number ) -> MyStruct {
            new MyStruct {
                a: 8
                b: value
            }
        }
        create_new_MyStruct( a )
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };
        let mut struct_map: HashMap<String, Datatype> = HashMap::new();
        struct_map.insert("a".to_string(), Datatype::Number(8));
        struct_map.insert("b".to_string(), Datatype::Number(3));
        assert_eq!(
            Datatype::Struct { map: struct_map },
            ast.evaluate(&mut map).unwrap()
        )
    }

    #[test]
    fn program_verify_that_struct_maps_dont_interfere_with_function_maps() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = r##"
        struct MyStruct {
            a : Number
            b : Number
        }

        let a := 3

        fn create_new_MyStruct( a: Number ) -> MyStruct {
            new MyStruct {
                a: 8
                b: a
            }
        }
        create_new_MyStruct( a )
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };
        let mut struct_map: HashMap<String, Datatype> = HashMap::new();
        struct_map.insert("a".to_string(), Datatype::Number(8));
        struct_map.insert("b".to_string(), Datatype::Number(3));
        assert_eq!(
            Datatype::Struct { map: struct_map },
            ast.evaluate(&mut map).unwrap()
        )
    }



    #[test]
    fn program_with_struct_functions_integration_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = r##"
        struct MyStruct {
            a : Number
            b : Number
        }

        let a := 3

        fn create_new_MyStruct( value: Number ) -> MyStruct {
            new MyStruct {
                a: 8
                b: value
            }
        }

        fn addContents( s: MyStruct ) -> Number {
            s.a + s.b
        }

        let instance := create_new_MyStruct( a )

        addContents( instance )

        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };


        assert_eq!(Datatype::Number(11), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn program_with_a_conditional_in_a_function_integration_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = r##"
        let a := 3

        fn check_if_three( x: Number ) -> Number {
            if x == 3 {
                3
            } else {
                0
            }
        }

        check_if_three( a )
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };


        assert_eq!(Datatype::Number(3), ast.evaluate(&mut map).unwrap())
    }



    #[test]
    fn program_with_a_conditional_in_a_function_2_integration_test() {
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = r##"
        let a := 2

        fn check_if_three( x: Number ) -> Number {
            if x == 3 {
                3
            } else {
                0
            }
        }

        check_if_three( a )
        "##;
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };


        assert_eq!(Datatype::Number(0), ast.evaluate(&mut map).unwrap())
    }

    #[test]
    fn for_loop_eval() {
        use std::collections::HashMap;
        let mut map: HashMap<String, Datatype> = HashMap::new();
        let input_string = r#"
        let b := 0
        for i in [1,2,3] {
           let b := b + i
        }
        b
         "#;
        use std_functions::add_std_functions;
        use parser::program;
        add_std_functions(&mut map);
        let (_, ast) = match program(input_string.as_bytes()) {
            IResult::Done(rest, v) => (rest, v),
            IResult::Error(e) => panic!("{}", e),
            _ => panic!(),
        };

       assert_eq!(Datatype::Number(6), ast.evaluate(&mut map).unwrap());
    }


    mod benches {
        use super::*;

        #[bench]
        fn simple_program_execute_bench(b: &mut Bencher) {
            use super::super::super::test_constants::SIMPLE_PROGRAM_INPUT_1;
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
        fn simple_program_parse_and_execute_bench(b: &mut Bencher) {
            use super::super::super::test_constants::SIMPLE_PROGRAM_INPUT_1;

            b.iter(|| {
                let mut map: HashMap<String, Datatype> = HashMap::new();
                let (_, ast) = match program(SIMPLE_PROGRAM_INPUT_1.as_bytes()) {
                    IResult::Done(rest, v) => (rest, v),
                    IResult::Error(e) => panic!("{}", e),
                    _ => panic!(),
                };
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
                while x < 1000 {
                   let x := x + 1
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
                while x < 1000 {
                    1 * 3
                    1 * 40000
                    34234 % 7
                    let x := x + 1
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
        fn for_loop_parse_and_execute_program_bench(b: &mut Bencher) {
            fn loop_1000_times_program() {
                use nom::IResult;
                let mut map: HashMap<String, Datatype> = HashMap::new();
                let input_string = r##"
                let c := 0
                let x := [1..1001]
                for a in x {
                    let c := c + a
                }
                c
                "##;
                let (_, ast) = match program(input_string.as_bytes()) {
                    IResult::Done(rest, v) => (rest, v),
                    IResult::Error(e) => panic!("{}", e),
                    _ => panic!(),
                };

                assert_eq!(Datatype::Number(500500), ast.evaluate(&mut map).unwrap());
            }

            b.iter(|| loop_1000_times_program());
        }
        #[bench]
        fn for_loop_alt_parse_and_execute_program_bench(b: &mut Bencher) {
            fn loop_1000_times_program() {
                use nom::IResult;
                let mut map: HashMap<String, Datatype> = HashMap::new();
                let input_string = r##"
                let c := 0
                for a in [1..1001] {
                    let c := c + a
                }
                c
                "##;
                let (_, ast) = match program(input_string.as_bytes()) {
                    IResult::Done(rest, v) => (rest, v),
                    IResult::Error(e) => panic!("{}", e),
                    _ => panic!(),
                };

                assert_eq!(Datatype::Number(500500), ast.evaluate(&mut map).unwrap());
            }

            b.iter(|| loop_1000_times_program());
        }

        #[bench]
        fn while_loop_similar_to_for_parse_and_execute_program_bench(b: &mut Bencher) {
            fn loop_1000_times_program() {
                use nom::IResult;
                let mut map: HashMap<String, Datatype> = HashMap::new();
                let input_string = r##"
                let c := 0
                let index := 0
                let array := [1..1001]
                while index < 1000 {
                    let part := array[index]
                    let c := c + part
                    let index := index + 1
                }
                c
                "##;
                let (_, ast) = match program(input_string.as_bytes()) {
                    IResult::Done(rest, v) => (rest, v),
                    IResult::Error(e) => panic!("{}", e),
                    _ => panic!(),
                };

                assert_eq!(Datatype::Number(500500), ast.evaluate(&mut map).unwrap());
            }

            b.iter(|| loop_1000_times_program());
        }

        #[bench]
        fn while_loop_similar_to_for_no_array_access_parse_and_execute_program_bench(b: &mut Bencher) {
            fn loop_1000_times_program() {
                use nom::IResult;
                let mut map: HashMap<String, Datatype> = HashMap::new();
                let input_string = r##"
                let c := 0
                let index := 0
                let array := [1..1001]
                while index < 1000 {
                    let c := c + 1
                    let index := index + 1
                }
                c
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

    #[bench]
    fn array_range_then_access_parse_and_execute_program_bench(b: &mut Bencher) {
        fn loop_1000_times_program() {
            use nom::IResult;
            let mut map: HashMap<String, Datatype> = HashMap::new();
            let input_string = r##"
                let a := [1..1001]
                a[0]
                "##;
            let (_, ast) = match program(input_string.as_bytes()) {
                IResult::Done(rest, v) => (rest, v),
                IResult::Error(e) => panic!("{}", e),
                _ => panic!(),
            };

            assert_eq!(Datatype::Number(1), ast.evaluate(&mut map).unwrap());
        }

        b.iter(|| loop_1000_times_program());
    }
    #[bench]
    fn array_range_parse_and_execute_program_bench(b: &mut Bencher) {
        fn loop_1000_times_program() {
            use nom::IResult;
            let mut map: HashMap<String, Datatype> = HashMap::new();
            let input_string = r##"
                let a := [1..1001]
                a
                "##;
            let (_, ast) = match program(input_string.as_bytes()) {
                IResult::Done(rest, v) => (rest, v),
                IResult::Error(e) => panic!("{}", e),
                _ => panic!(),
            };
            ast.evaluate(&mut map).unwrap();
            //assert_eq!(Datatype::Number(1), ast.evaluate(&mut map).unwrap());
        }

        b.iter(|| loop_1000_times_program());
    }


}
