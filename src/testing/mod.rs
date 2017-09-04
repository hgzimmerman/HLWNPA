mod integration_tests;
#[allow(dead_code)]
pub const SIMPLE_PROGRAM_INPUT_1: &'static str = "
     let x := ( 3 + 4 )
     fn test_function ( a : Number ) -> Number { ( a + 8 ) }
     test_function(x)";
