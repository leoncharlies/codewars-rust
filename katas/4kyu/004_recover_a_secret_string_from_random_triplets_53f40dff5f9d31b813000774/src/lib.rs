//! Title: Recover a secret string from random triplets
//! Link: https://www.codewars.com/kata/53f40dff5f9d31b813000774
//! Kata ID: 53f40dff5f9d31b813000774
//! Rank: 4kyu
//! Completed: 2025-12-06
#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut graph: HashMap<char, HashSet<char>> = HashMap::new();
    let mut indegree: HashMap<char, usize> = HashMap::new();

    // 初始化字符
    for t in &triplets {
        for &c in t {
            graph.entry(c).or_default();
            indegree.entry(c).or_insert(0);
        }
    }

    // 建边
    for [a, b, c] in triplets {
        for (x, y) in [(a, b), (b, c)] {
            if graph.get_mut(&x).unwrap().insert(y) {
                *indegree.get_mut(&y).unwrap() += 1;
            }
        }
    }

    // 拓扑排序
    let mut q: VecDeque<char> = indegree
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(&c, _)| c)
        .collect();

    let mut ans = String::new();

    while let Some(c) = q.pop_front() {
        ans.push(c);
        for &n in &graph[&c] {
            let deg = indegree.get_mut(&n).unwrap();
            *deg -= 1;
            if *deg == 0 {
                q.push_back(n);
            }
        }
    }

    ans
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

#[cfg(test)]
mod review_1;
