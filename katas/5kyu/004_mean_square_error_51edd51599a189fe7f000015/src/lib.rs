//! Title: mean square error
//! Link: https://www.codewars.com/kata/51edd51599a189fe7f000015
//! Kata ID: 51edd51599a189fe7f000015
//! Rank: 5kyu
//! Completed: 2025-12-02
//!
//! TODO: 在这里写你的解法
//!
//! 把 Codewars 的测试用例复制到下面 tests 模块里即可（推荐保留原样,便于以后复习）

pub fn solution(array_a: &[i64], array_b: &[i64]) -> f64 {
    let mut sum = 0;
    for i in 0..array_a.len() {
        sum += (array_a[i] - array_b[i]).pow(2);
    }
    sum as f64 / array_a.len() as f64
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod sample_tests {
    use super::solution;

    macro_rules! assert_approx_eq {
        ($sol:expr, $exp:expr, $epsilon:expr) => {
            assert!(
                ($exp - $sol).abs() <= $epsilon,
                "Expected {}, got {}. Allowed error margin: {}",
                $exp,
                $sol,
                $epsilon
            )
        };
        ($sol:expr, $exp:expr) => {
            assert_approx_eq!($sol, $exp, 1e-9)
        };
    }

    #[test]
    fn test_fixed() {
        assert_approx_eq!(solution(&[1, 2, 3], &[4, 5, 6]), 9.);
        assert_approx_eq!(solution(&[10, 20, 10, 2], &[10, 25, 5, -2]), 16.5);
        assert_approx_eq!(solution(&[0, -1], &[-1, 0]), 1.);
        assert_approx_eq!(solution(&[10, 10], &[10, 10]), 0.);
    }
}
