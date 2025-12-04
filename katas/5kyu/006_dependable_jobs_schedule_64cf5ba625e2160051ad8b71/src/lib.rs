//! Title: Dependable Jobs Schedule
//! Link: https://www.codewars.com/kata/64cf5ba625e2160051ad8b71
//! Kata ID: 64cf5ba625e2160051ad8b71
//! Rank: 5kyu
//! Completed: 2025-12-03
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

struct Graph {
    adj: HashMap<u32, Vec<u32>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adj: HashMap::new(),
        }
    }
    fn add_edge(&mut self, u: u32, v: u32) {
        self.adj.entry(u).or_insert(Vec::new()).push(v);
    }
    fn has_cycle(&self) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        for &node in self.adj.keys() {
            if !visited.contains(&node) {
                if self.dfs_cycle(node, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }
        false
    }
    fn dfs_cycle(
        &self,
        node: u32,
        visited: &mut HashSet<u32>,
        rec_stack: &mut HashSet<u32>,
    ) -> bool {
        visited.insert(node);
        rec_stack.insert(node);
        if let Some(neighbors) = self.adj.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    if self.dfs_cycle(neighbor, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(&neighbor) {
                    return true;
                }
            }
        }
        rec_stack.remove(&node);
        false
    }
}
fn finish_all(prerequisites: &[(u32, u32)]) -> bool {
    let mut gh = Graph::new();
    for pre in prerequisites {
        gh.add_edge(pre.0, pre.1);
    }
    if gh.has_cycle() {
        return false;
    } else {
        return true;
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::finish_all;

    fn dotest(prerequisites: &[(u32, u32)], expected: bool) {
        let actual = finish_all(prerequisites);
        assert!(
            actual == expected,
            "With prerequisites = {prerequisites:?}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        for (lst, expected) in [
            // (&vec![(1, 0)], true),
            // (&vec![(1, 0), (0, 1)], false),
            // (&vec![(1, 0), (2, 1)], true),
            // (&vec![(0, 1), (1, 2), (2, 0)], false),
            // (&vec![], true),
            (&vec![(1, 2), (1, 0), (2, 0)], true),
            (&vec![(1, 0), (1, 2), (2, 0)], true),
            (
                &vec![
                    (6, 10),
                    (4, 3),
                    (9, 2),
                    (2, 3),
                    (6, 1),
                    (2, 8),
                    (10, 1),
                    (10, 2),
                    (5, 3),
                    (0, 10),
                    (7, 4),
                ],
                true,
            ),
            (
                &vec![
                    (7, 17),
                    (4, 0),
                    (20, 14),
                    (12, 10),
                    (4, 12),
                    (14, 10),
                    (19, 0),
                    (11, 5),
                    (8, 18),
                    (0, 17),
                    (1, 12),
                    (14, 6),
                    (14, 18),
                    (4, 17),
                    (9, 7),
                    (19, 11),
                    (9, 10),
                    (1, 5),
                    (16, 6),
                    (2, 13),
                    (7, 9),
                    (18, 18),
                    (6, 16),
                    (3, 14),
                    (0, 0),
                    (11, 9),
                    (7, 2),
                    (13, 15),
                    (16, 11),
                ],
                false,
            ),
            (
                &vec![
                    (3, 7),
                    (18, 29),
                    (27, 7),
                    (28, 12),
                    (9, 14),
                    (2, 5),
                    (5, 28),
                    (8, 27),
                    (6, 5),
                    (1, 24),
                    (14, 3),
                    (4, 8),
                    (14, 12),
                    (17, 11),
                    (3, 21),
                    (5, 15),
                    (10, 5),
                    (29, 6),
                    (11, 22),
                    (13, 19),
                    (29, 9),
                    (7, 6),
                    (20, 12),
                    (29, 21),
                    (23, 14),
                    (20, 21),
                    (14, 5),
                    (22, 21),
                    (17, 19),
                    (27, 23),
                    (13, 3),
                    (27, 29),
                    (10, 19),
                    (10, 22),
                ],
                true,
            ),
        ] {
            dotest(lst, expected)
        }
    }
}
