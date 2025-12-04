//! Title: Take a Ten Minutes Walk
//! Link: https://www.codewars.com/kata/54da539698b8a2ad76000228
//! Kata ID: 54da539698b8a2ad76000228
//! Rank: 6kyu
//! Completed: 2025-12-04
#![allow(dead_code)]
use std::collections::HashMap;
fn is_valid_walk(walk: &[char]) -> bool {
    if walk.len() != 10 {
        return false;
    }
    let mut map = HashMap::new();
    for &w in walk {
        match w {
            'n' => *map.entry("high").or_insert(0) += 1,
            's' => *map.entry("high").or_insert(0) -= 1,
            'e' => *map.entry("wide").or_insert(0) += 1,
            'w' => *map.entry("wide").or_insert(0) -= 1,
            _ => (),
        }
    }
    let mut height = 0;
    let mut width = 0;
    if let Some(h) = map.get("high") {
        height = *h
    }
    if let Some(w) = map.get("wide") {
        width = *w
    }
    if height == 0 && width == 0 {
        return true;
    } else {
        return false;
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::is_valid_walk;

    #[test]
    fn sample_tests() {
        assert!(is_valid_walk(&[
            'n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
        ]));
        assert!(!is_valid_walk(&[
            'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e'
        ]));
        assert!(!is_valid_walk(&['w']));
        assert!(!is_valid_walk(&[
            'n', 'n', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
        ]));
        assert!(!is_valid_walk(&[
            'e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's'
        ]))
    }
}
