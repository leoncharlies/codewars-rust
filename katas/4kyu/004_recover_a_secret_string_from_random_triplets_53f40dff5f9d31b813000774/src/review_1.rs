//! Review #1
//! Date: 2025-12-09
#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut map: HashMap<char, HashSet<char>> = HashMap::new();
    for t in triplets {
        map.entry(t[2]).or_default().insert(t[1]);
        map.entry(t[1]).or_default().insert(t[0]);
        map.entry(t[0]).or_default();
    }
    let mut result = String::with_capacity(map.len());
    while let Some(&c) = map.keys().find(|c| map[c].is_empty()) {
        result.push(c);
        map.remove(&c);
        for i in map.values_mut() {
            i.remove(&c);
        }
    }
    result
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn example_test() {
    assert_eq!(
        recover_secret(vec![
            ['t', 'u', 'p'],
            ['w', 'h', 'i'],
            ['t', 's', 'u'],
            ['a', 't', 's'],
            ['h', 'a', 'p'],
            ['t', 'i', 's'],
            ['w', 'h', 's']
        ]),
        "whatisup"
    );
}
