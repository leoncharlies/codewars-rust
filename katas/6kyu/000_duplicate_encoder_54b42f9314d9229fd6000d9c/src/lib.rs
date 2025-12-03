//! Title: Duplicate Encoder
//! Link: https://www.codewars.com/kata/54b42f9314d9229fd6000d9c
//! Kata ID: 54b42f9314d9229fd6000d9c
//! Rank: 6kyu
//! Completed: 2025-12-02
#![allow(dead_code)]

use std::collections::HashMap;
fn duplicate_encode(word: &str) -> String {
    let mut hm: HashMap<char, u32> = HashMap::new();
    let word = word.to_lowercase();
    for c in word.chars() {
        *hm.entry(c).or_default() += 1;
    }
    let mut result = String::new();
    for c in word.chars() {
        if *hm.get(&c).unwrap() == 1 {
            result.push('(');
        } else {
            result.push(')');
        }
    }
    result
}

#[test]
fn run_tests() {
    assert_eq!(duplicate_encode("din"), "(((");
    assert_eq!(duplicate_encode("recede"), "()()()");
    assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
    assert_eq!(duplicate_encode("(( @"), "))((");
}

#[cfg(test)]
mod review_1;
