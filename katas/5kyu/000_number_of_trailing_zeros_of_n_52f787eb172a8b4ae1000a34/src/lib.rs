//! Title: Number of trailing zeros of N!
//! Link: https://www.codewars.com/kata/52f787eb172a8b4ae1000a34
//! Kata ID: 52f787eb172a8b4ae1000a34
//! Rank: 5kyu
//! Completed: 2025-12-01
//!
//! TODO: 在这里写你的解法
//!
//! 把 Codewars 的测试用例复制到下面 tests 模块里即可（推荐保留原样，便于以后复习）

fn zeros(n: u64) -> u64 {
    let mut zeros = 0;
    let mut five = 5;
    if n == 0 {
        zeros = 0
    }
    while five <= n {
        zeros += n / five;
        if five > n / 5 {
            break;
        }
        five *= 5;
    }
    zeros
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(zeros(0), 0);
        assert_eq!(zeros(6), 1);
        assert_eq!(zeros(14), 2);
        assert_eq!(zeros(30), 7);
        assert_eq!(zeros(1000), 249);
        assert_eq!(zeros(100000), 24999);
        assert_eq!(zeros(1000000000), 249999998);
    }
}
