//! Title: so many permutations!
//! Link: https://www.codewars.com/kata/5254ca2719453dcc0b00027d
//! Kata ID: 5254ca2719453dcc0b00027d
//! Rank: 4kyu
//! Completed: 2025-12-02
//!
//! TODO: 在这里写你的解法
//!
//! 把 Codewars 的测试用例复制到下面 tests 模块里即可（推荐保留原样,便于以后复习）

pub fn permutations(s: &str) -> Vec<String> {
    todo!("Your code here")
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
