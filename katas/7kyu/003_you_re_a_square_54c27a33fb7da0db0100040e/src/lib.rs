//! Title: You're a square!
//! Link: https://www.codewars.com/kata/54c27a33fb7da0db0100040e
//! Kata ID: 54c27a33fb7da0db0100040e
//! Rank: 7kyu
//! Completed: 2025-12-04
#![allow(dead_code)]

fn is_square(n: i64) -> bool {
    if n < 0 {
        return false;
    }
    let ot = (n as f64).sqrt();
    let base = ot as i64;
    base * base == n
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::is_square;

    #[test]
    fn fixed_tests() {
        assert_eq!(
            is_square(-1),
            false,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            is_square(0),
            true,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            is_square(3),
            false,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            is_square(4),
            true,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            is_square(25),
            true,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            is_square(26),
            false,
            "\nYour answer (left) is not the expected answer (right)."
        );
    }
}
