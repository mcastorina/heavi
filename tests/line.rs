use heavi::*;

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
fn test_heavi_no_match() {
    test_heavi("test", "pattern", "");
}

#[test]
fn test_heavi_match() {
    test_heavi("a\nb\nc\n", "b", "c\n");
}

#[test]
fn test_heavi_inv_no_match() {
    test_heavi_inv("test", "pattern", "test");
}

#[test]
fn test_heavi_inv_match() {
    test_heavi_inv("a\nb\nc\n", "b", "a\n");
}
