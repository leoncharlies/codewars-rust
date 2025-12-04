//! Title: Counting sheep...
//! Link: https://www.codewars.com/kata/54edbc7200b811e956000556
//! Kata ID: 54edbc7200b811e956000556
//! Rank: 8kyu
//! Completed: 2025-12-04
#![allow(dead_code)]

fn count_sheep(sheep: &[bool]) -> u8 {
    sheep
        .iter()
        .map(|&c| {
            if c {
                return 1;
            } else {
                return 0;
            }
        })
        .sum()
}

#[test]
fn returns_correct_sheep_count() {
    assert_eq!(count_sheep(&[false]), 0);
    assert_eq!(count_sheep(&[true]), 1);
    assert_eq!(count_sheep(&[true, false]), 1);
}
