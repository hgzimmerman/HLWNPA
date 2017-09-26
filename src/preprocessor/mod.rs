
pub fn preprocess(string: &str) -> String {
    replace_escapes(string)
}


/// This replaces \\<escape> instances with \<escape>.
/// The readline functionality sanitizes these escapes with the double backslash, this returns them to the form they were entered with.
fn replace_escapes(string: &str) -> String {
    let string = string.to_string();
    string.replace("\\n","\n")
        .replace("\\t", "\t")
        .replace("\\r", "\r")
        .replace("\\\\","\\")
        .replace(r#"\""#, "\"")
}

#[cfg(test)]
mod test {
    use super::*;
    use testing::test_constants;
    use test::Bencher;

    #[test]
    fn escape_escapes() {
        assert_eq!("\n".to_string(), replace_escapes("\\n"));
        assert_eq!("hello\n".to_string(), replace_escapes("hello\\n"));
        assert_eq!("hello\nworld", replace_escapes("hello\\nworld"));
        assert_eq!("hello\tworld", replace_escapes("hello\\tworld"));
        assert_eq!("hello\n\rworld", replace_escapes("hello\\n\\rworld"));
        assert_eq!("hello\\world", replace_escapes("hello\\\\world"));
        assert_eq!("hello\"world", replace_escapes(r#"hello\\"world"#));
    }

    #[bench]
    fn preprocess_simple_program_bench(b: &mut Bencher) {
        b.iter(|| preprocess(test_constants::SIMPLE_PROGRAM_INPUT_1));
    }
}