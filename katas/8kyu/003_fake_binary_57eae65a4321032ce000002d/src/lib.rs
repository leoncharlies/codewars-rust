//! Title: Fake Binary
//! Link: https://www.codewars.com/kata/57eae65a4321032ce000002d
//! Kata ID: 57eae65a4321032ce000002d
//! Rank: 8kyu
//! Completed: 2025-12-04
#![allow(dead_code)]

fn fake_bin(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.to_digit(10).unwrap() < 5 {
                '0'
            } else {
                '1'
            }
        })
        .collect()
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::fake_bin;

    #[test]
    fn basic_tests() {
        assert_eq!(fake_bin("45385593107843568"), "01011110001100111");
        assert_eq!(fake_bin("509321967506747"), "101000111101101");
        assert_eq!(
            fake_bin("366058562030849490134388085"),
            "011011110000101010000011011"
        );
        assert_eq!(fake_bin("15889923"), "01111100");
        assert_eq!(fake_bin("800857237867"), "100111001111");
    }
}
