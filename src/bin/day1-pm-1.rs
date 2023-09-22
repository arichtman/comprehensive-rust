// TODO: remove this when you're done with your implementation.
// #![allow(unused_variables, dead_code)]

//This while implementation is so nasty it's not even funny...
fn jank(input: char) -> Option<char> {
    let double = input.to_digit(10).unwrap() * 2;
    let double = double.to_string();
    let output = double.chars().fold(0, |a, c| c.to_digit(10).unwrap() + a);
    char::from_u32(output)
}
pub fn luhn(cc_number: &str) -> bool {
    let cc_number = cc_number.replace(" ", "");
    // I wanted to do this without the else to bail out early...
    // Not indent the rest of this function...
    if cc_number.len() <= 2 {
        false
    } else {
        let cc_number = cc_number
            .char_indices()
            .fold(String::new(), |a, (i, c)| {
                format!("{a}{}", if i % 2 == 0 { jank(c).unwrap() } else { c })
            })
            .chars()
            .fold(0, |a, c| a + c.to_digit(10).unwrap());
        let final_char = cc_number.to_string().chars().last().unwrap();
        final_char == '0'
    }
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
