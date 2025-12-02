//! Title: Not very secure
//! Link: https://www.codewars.com/kata/526dbd6c8c0eb53254000110
//! Kata ID: 526dbd6c8c0eb53254000110
//! Rank: 5kyu
//! Completed: 2025-12-01
//!
//! TODO: 在这里写你的解法
//!
//! 把 Codewars 的测试用例复制到下面 tests 模块里即可（推荐保留原样，便于以后复习）

fn alphanumeric(password: &str) -> bool {
    if password.is_empty() {
        return false;
    }
    if password.chars().all(|c| c.is_alphanumeric()) {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::alphanumeric;

    fn do_test(s: &str, expected: bool) {
        let actual = alphanumeric(s);
        assert_eq!(
            actual, expected,
            "\nInput: {s:?}\nYour result (left) did not match the expected output (right)"
        )
    }

    #[test]
    fn sample_tests() {
        do_test("hello world_", false);
        do_test("PassW0rd", true);
        do_test("     ", false);
    }
}
