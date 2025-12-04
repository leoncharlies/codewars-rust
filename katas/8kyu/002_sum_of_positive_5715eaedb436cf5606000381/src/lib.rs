//! Title: Sum of positive
//! Link: https://www.codewars.com/kata/5715eaedb436cf5606000381
//! Kata ID: 5715eaedb436cf5606000381
//! Rank: 8kyu
//! Completed: 2025-12-04
#![allow(dead_code)]

fn positive_sum(slice: &[i32]) -> i32 {
    // your code
    slice.iter().filter(|&i| *i > 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_examples() {
        assert_eq!(positive_sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(positive_sum(&[1, -2, 3, 4, 5]), 13);
        assert_eq!(positive_sum(&[-1, 2, 3, 4, -5]), 9);
    }

    #[test]
    fn empty_list() {
        assert_eq!(positive_sum(&[]), 0);
    }

    #[test]
    fn all_negative() {
        assert_eq!(positive_sum(&[-1, -2, -3, -4, -5]), 0);
    }
}
