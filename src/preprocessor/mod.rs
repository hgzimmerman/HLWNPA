use regex::Regex;
use std::borrow::Cow;

pub fn preprocess(string: &str) -> String {
    replace_escapes(string)
}


// Todo: this replaces instances of \\n \\t and \\r with only \n. This should replace the relevant escape sequence.
fn replace_escapes(string: &str) -> String {
    let mut edit = Cow::from("");
    {
        let newline_re = Regex::new(r"\\(?P<s>[ntr])").unwrap();
        edit = newline_re.replace_all(string, "\n");
    }
    edit.to_string()
}

#[test]
fn escape_escapes() {
    assert_eq!("\n".to_string(), replace_escapes("\\n"));
    assert_eq!("hello\n".to_string(), replace_escapes("hello\\n"));
    assert_eq!("hello\nworld", replace_escapes("hello\\nworld") )
}