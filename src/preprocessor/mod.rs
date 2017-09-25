use regex::Regex;
use std::borrow::Cow;

pub fn preprocess(string: &str) -> String {
    replace_escapes(string)
}


/// This replaces \\<escape> instances with \<escape>.
/// The readline functionality sanitizes these escapes with the double backslash, this returns them to the form they were entered with.
// TODO: This uses 4 regexes to alter 4 different strings, this should be able to be accomplished in a single pass
fn replace_escapes(string: &str) -> String {
    let newline_re = Regex::new(r"\\n").unwrap();
    let newline_string = newline_re.replace_all(string, "\n").to_string();

    let tab_re = Regex::new(r"\\t").unwrap();
    let tab_string = tab_re.replace_all(newline_string.as_str(), "\t").to_string();

    let return_re = Regex::new(r"\\r").unwrap();
    let return_string = return_re.replace_all(tab_string.as_str(), "\r").to_string();

    let backslash_re = Regex::new(r"\\\\").unwrap();
    let backslash_string = backslash_re.replace_all(return_string.as_str(), "\\").to_string();

    backslash_string
}

#[test]
fn escape_escapes() {
    assert_eq!("\n".to_string(), replace_escapes("\\n"));
    assert_eq!("hello\n".to_string(), replace_escapes("hello\\n"));
    assert_eq!("hello\nworld", replace_escapes("hello\\nworld"));
    assert_eq!("hello\tworld", replace_escapes("hello\\tworld"));
    assert_eq!("hello\n\rworld", replace_escapes("hello\\n\\rworld"));
    assert_eq!("hello\\world", replace_escapes("hello\\\\world"));
}