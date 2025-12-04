//! Title: Square(n) Sum
//! Link: https://www.codewars.com/kata/515e271a311df0350d00000f
//! Kata ID: 515e271a311df0350d00000f
//! Rank: 8kyu
//! Completed: 2025-12-04
#![allow(dead_code)]

fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|n| n.pow(2)).sum()
}

#[test]
fn returns_expected() {
    assert_eq!(square_sum(vec![1, 2]), 5);
    assert_eq!(square_sum(vec![-1, -2]), 5);
    assert_eq!(square_sum(vec![5, 3, 4]), 50);
    assert_eq!(square_sum(vec![]), 0);
}
