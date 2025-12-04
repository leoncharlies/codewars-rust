//! Title: String ends with?
//! Link: https://www.codewars.com/kata/51f2d1cafc9c0f745c00037d
//! Kata ID: 51f2d1cafc9c0f745c00037d
//! Rank: 7kyu
//! Completed: 2025-12-04
#![allow(dead_code)]

fn solution(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
    assert_eq!(true, solution("abc", "c"));
    assert_eq!(false, solution("strawberry", "banana"));
}
