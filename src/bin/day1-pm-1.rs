// TODO: remove this when you're done with your implementation.
// #![allow(unused_variables, dead_code)]

// Overall I'm happy with this.
// There's quite a bit of boilerplate but that's from munging strings and integers
// There may be a more elegant solution using arrays or vectors directly

fn double_or_nothing(index_in: usize, digit_in: u32) -> Option<char> {
    if index_in % 2 == 0 {
        return char::from_digit(digit_in, 10);
    };
    let doubled_number = digit_in * 2;
    // These couple of unwraps _should_ be safe, as we've already constrained the input
    let sum_of_digits: u32 = doubled_number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum();
    char::from_digit(sum_of_digits, 10)
}

pub fn luhn(cc_number: &str) -> bool {
    // Strip spaces
    let cc_number = cc_number.replace(" ", "");
    // Check all valid numbers
    if !cc_number.chars().map(|c| c.is_digit(10)).all(|c| c) {
        return false;
    };
    // Check minimum length
    if cc_number.len() < 2 {
        return false;
    }
    // We must flip the string first to make sure the indexes are right-to-left
    let reversed_string = cc_number.chars().rev().collect::<String>();
    // Iterate over tuples enumerating the string characters
    //  processing each character and collecting them cumulatively into a final String
    let processed_string = reversed_string
        .char_indices()
        .fold(String::new(), |a, (i, c)| {
            format!(
                "{a}{}",
                double_or_nothing(i, c.to_digit(10).unwrap()).unwrap()
            )
        });
    processed_string
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>()
        % 10
        == 0
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
