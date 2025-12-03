//! Title: Unique In Order
//! Link: https://www.codewars.com/kata/54e6533c92449cc251001667
//! Kata ID: 54e6533c92449cc251001667
//! Rank: 6kyu
//! Completed: 2025-12-02

fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut result = Vec::new();
    for item in sequence {
        if result.last() != Some(&item) {
            result.push(item);
        }
    }
    result
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(
            unique_in_order("AAAABBBCCDAABBB".chars()),
            vec!['A', 'B', 'C', 'D', 'A', 'B']
        );
    }
}
