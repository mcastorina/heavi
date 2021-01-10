use heavi::*;

fn test_heavi_line(input: &str, pattern: &str, expected: &str) {
    let mut output: Vec<u8> = vec![];
    assert!(heavi_line(input.as_bytes(), &mut output, pattern).is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), expected);
}
fn test_heavi_line_inv(input: &str, pattern: &str, expected: &str) {
    let mut output: Vec<u8> = vec![];
    assert!(heavi_line_inv(input.as_bytes(), &mut output, pattern).is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), expected);
}
fn test_heavi(input: &str, pattern: &str, expected: &str) {
    let mut output: Vec<u8> = vec![];
    assert!(heavi(input.as_bytes(), &mut output, pattern).is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), expected);
}
fn test_heavi_inv(input: &str, pattern: &str, expected: &str) {
    let mut output: Vec<u8> = vec![];
    assert!(heavi_inv(input.as_bytes(), &mut output, pattern).is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), expected);
}

#[test]
fn test_heavi_line_no_match() {
    test_heavi_line("test", "pattern", "");
}

#[test]
fn test_heavi_line_match() {
    test_heavi_line("a\nb\nc\nd\ne\n", "c", "d\ne\n");
}

#[test]
fn test_heavi_line_inv_no_match() {
    test_heavi_line_inv("test", "pattern", "test");
}

#[test]
fn test_heavi_line_inv_match() {
    test_heavi_line_inv("a\nb\nc\n", "b", "a\n");
}

#[test]
fn test_heavi_no_match() {
    test_heavi("test", "pattern", "");
}

#[test]
fn test_heavi_match() {
    test_heavi("abcde", "c", "de");
}

#[test]
fn test_heavi_inv_no_match() {
    test_heavi_inv("test", "pattern", "test");
}

#[test]
fn test_heavi_inv_match() {
    test_heavi_inv("abcde", "c", "ab");
}
