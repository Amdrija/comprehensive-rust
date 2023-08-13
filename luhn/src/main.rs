pub fn luhn(cc_number: &str) -> bool {
    let cc_number = cc_number.replace(" ", "");

    let mut sum = 0;
    let mut digits_in_cc = 0;
    for (i, c) in cc_number.char_indices() {
        if c == ' ' {
            continue;
        }

        let mut digit = match c.to_digit(10) {
            Some(d) => d,
            None => return false,
        };

        digits_in_cc += 1;
        if i % 2 == cc_number.len() % 2 {
            digit *= 2;
            digit = digit / 10 + digit % 10;
        }

        sum += digit;
    }

    if digits_in_cc < 2 {
        return false;
    }

    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
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
