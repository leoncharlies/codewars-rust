//! Title: Abbreviate a Two Word Name
//! Link: https://www.codewars.com/kata/57eadb7ecd143f4c9c0000a3
//! Kata ID: 57eadb7ecd143f4c9c0000a3
//! Rank: 8kyu
//! Completed: 2025-12-03
#![allow(dead_code)]

fn abbrev_name(name: &str) -> String {
    let mut result = String::new();
    for n in name.split(' ').collect::<Vec<&str>>() {
        result.push(n.to_uppercase().chars().nth(0).unwrap());
    }
    result.insert(1, '.');
    result
}

// Rust test example:
#[test]
fn sample_tests() {
    assert_eq!(abbrev_name("Sam Harris"), "S.H");
    assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
    assert_eq!(abbrev_name("Evan Cole"), "E.C");
    assert_eq!(abbrev_name("P Favuzzi"), "P.F");
    assert_eq!(abbrev_name("David Mendieta"), "D.M");
}
