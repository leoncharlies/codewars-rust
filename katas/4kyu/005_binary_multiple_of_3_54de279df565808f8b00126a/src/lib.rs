//! Title: Binary multiple of 3
//! Link: https://www.codewars.com/kata/54de279df565808f8b00126a
//! Kata ID: 54de279df565808f8b00126a
//! Rank: 4kyu
//! Completed: 2025-12-09
#![allow(dead_code)]

use regex::Regex;

fn binary_multiple_of_3() -> Regex {
    return Regex::new(r"^(0|(1(01*0)*1))*$").unwrap();
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::binary_multiple_of_3;

    fn dotest(s: &str, expected: bool) {
        let actual = binary_multiple_of_3().is_match(s);
        assert!(
            actual == expected,
            "Test failed with s = \"{s}\"\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest(" 0", false);
        dotest("abc", false);
        dotest("000", true);
        dotest("110", true);
        dotest("110a", false);
        dotest("111", false);
        dotest(&format!("{:b}", 12345678), true);
    }
}
