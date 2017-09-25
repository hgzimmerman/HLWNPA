use regex::Regex;

pub fn preprocess(string: &str) -> String {
    replace_escapes(string)
}

fn replace_escapes(string: &str) -> String {
    let newline_re = Regex::new(r"\\n").unwrap();
    newline_re.replace_all(string, "\n").to_string()
}

#[test]
fn escape_escapes() {
    assert_eq!("\n".to_string(), replace_escapes("\\n"));
    assert_eq!("hello\n".to_string(), replace_escapes("hello\\n"));
    let mut to_edit: String = "hello\\nworld".to_string();
    assert_eq!("hello\nworld", replace_escapes("hello\\nworld") )
}