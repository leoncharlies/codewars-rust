//! Title: so many permutations!
//! Link: https://www.codewars.com/kata/5254ca2719453dcc0b00027d
//! Kata ID: 5254ca2719453dcc0b00027d
//! Rank: 4kyu
//! Completed: 2025-12-02
//!
//! TODO: 在这里写你的解法
//!
//! 把 Codewars 的测试用例复制到下面 tests 模块里即可（推荐保留原样,便于以后复习）
#![allow(dead_code)]

fn permutations(s: &str) -> Vec<String> {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort(); // 先排序，方便去重

    let mut used = vec![false; chars.len()];
    let mut path = Vec::new();
    let mut res = Vec::new();

    backtrack(&chars, &mut used, &mut path, &mut res);

    res
}

fn backtrack(chars: &Vec<char>, used: &mut Vec<bool>, path: &mut Vec<char>, res: &mut Vec<String>) {
    if path.len() == chars.len() {
        res.push(path.iter().collect());
        return;
    }

    for i in 0..chars.len() {
        if used[i] {
            continue;
        }

        // 去重关键点：相同字符，只能使用第一个还没被用过的
        if i > 0 && chars[i] == chars[i - 1] && !used[i - 1] {
            continue;
        }

        used[i] = true;
        path.push(chars[i]);

        backtrack(chars, used, path, res);

        path.pop();
        used[i] = false;
    }
}

#[cfg(test)]
mod example_tests {
    use super::permutations;

    fn assert_ordered_equals(actual: &[String], expected: &[String]) {
        let mut actual: Vec<_> = actual.to_vec();
        let mut expected: Vec<_> = expected.to_vec();
        actual.sort_unstable();
        expected.sort_unstable();

        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right)"
        );
    }

    #[test]
    fn sample_tests() {
        let expected = vec!["a".to_string()];
        let actual = permutations("a");
        assert_ordered_equals(&actual, &expected);

        let expected = vec!["ab".to_string(), "ba".to_string()];
        let actual = permutations("ab");
        assert_ordered_equals(&actual, &expected);

        let expected = vec![
            "aabb".to_string(),
            "abab".to_string(),
            "abba".to_string(),
            "baab".to_string(),
            "baba".to_string(),
            "bbaa".to_string(),
        ];
        let actual = permutations("aabb");
        assert_ordered_equals(&actual, &expected);
    }
}
