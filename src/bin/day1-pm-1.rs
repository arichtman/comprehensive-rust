// TODO: remove this when you're done with your implementation.
// #![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let cc_number = cc_number.replace(" ", "");
    println!("Trimmed input {cc_number}");
    // I wanted to do this without the else to bail out early...
    // Not indent the rest of this function...
    // I hear side effects in pattern matching are bad anyways...
    // match cc_number.len() {
    //     ..=3 => return false,
    //     _ => (),
    // };
    if cc_number.len() < 2 {
        false
    } else {
        let mut accumulator = String::new();
        let cc_number: String = cc_number.chars().rev().collect();
        for (index, character) in cc_number.char_indices() {
            println!("Processing {index}, {character}");
            println!("Whole accumulator: {accumulator}");
            if index % 2 == 0 {
                println!("Pushing {character}");
                accumulator.push(character);
                continue;
            };
            let digit = match character.to_digit(10) {
                Some(int) => int,
                None => return false,
            };
            let digit = digit * 2;
            println!("Doubled to {digit}");
            // These couple of unwraps _should_ be safe, as we've already checked the input?
            let sum_of_digits: u32 = digit
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .sum();
            let result_character = char::from_digit(sum_of_digits, 10).unwrap();
            println!("Pushing {result_character}");
            accumulator.push(result_character);
        }
        println!("Final accumulator: {accumulator:?}");
        accumulator
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum::<u32>()
            % 10
            == 0
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
