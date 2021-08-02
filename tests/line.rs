use heavi::*;

fn test_heavi_line(input: &str, pattern: &str, expected: &str) {
    let mut output: Vec<u8> = vec![];
    assert!(Heavi {
        line_mode: true,
        invert: false,
        output: &mut output,
        inclusive: false,
    }
    .parse(input.as_bytes(), pattern)
    .is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), expected);
}
fn test_heavi_line_inv(input: &str, pattern: &str, expected: &str) {
    let mut output: Vec<u8> = vec![];
    assert!(Heavi {
        line_mode: true,
        invert: true,
        output: &mut output,
        inclusive: false,
    }
    .parse(input.as_bytes(), pattern)
    .is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), expected);
}
fn test_heavi(input: &str, pattern: &str, expected: &str) {
    let mut output: Vec<u8> = vec![];
    assert!(Heavi {
        line_mode: false,
        invert: false,
        output: &mut output,
        inclusive: false,
    }
    .parse(input.as_bytes(), pattern)
    .is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), expected);
}
fn test_heavi_inv(input: &str, pattern: &str, expected: &str) {
    let mut output: Vec<u8> = vec![];
    assert!(Heavi {
        line_mode: false,
        invert: true,
        output: &mut output,
        inclusive: false,
    }
    .parse(input.as_bytes(), pattern)
    .is_ok());
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

#[test]
fn test_heavi_multiline() {
    test_heavi("ab\ncd\ne", "b\nc", "d\ne");
}

#[test]
fn test_blank() {
    test_heavi("ab\ncd\n\nef", "^$", "\nef");
    test_heavi("\ncd", "^$", "\ncd");
}

#[test]
fn test_blank_line() {
    test_heavi_line("ab\ncd\n\nef", "^$", "ef");
    test_heavi_line("\ncd\n", "^$", "cd\n");
}
