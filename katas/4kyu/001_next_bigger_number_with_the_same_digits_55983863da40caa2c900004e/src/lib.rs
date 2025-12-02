//! Title: next bigger number with the same digits
//! Link: https://www.codewars.com/kata/55983863da40caa2c900004e
//! Kata ID: 55983863da40caa2c900004e
//! Rank: 4kyu
//! Completed: 2025-12-01
//!
//! TODO: 在这里写你的解法
//!
//! 把 Codewars 的测试用例复制到下面 tests 模块里即可（推荐保留原样,便于以后复习）

fn next_bigger_number(n: u64) -> Option<u64> {
    let mut v = n.to_string().bytes().collect::<Vec<_>>();
    (0..v.len() - 1)
        .rev()
        .find(|&i| v[i] < v[i + 1])
        .and_then(|i| {
            (0..v.len()).rev().find(|&j| v[j] > v[i]).and_then(|j| {
                v.swap(i, j);
                v[i + 1..].reverse();
                String::from_utf8(v).unwrap().parse().ok()
            })
        })
}

#[cfg(test)]

mod tests {
    use super::next_bigger_number;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected result (right).";

    #[test]
    fn sample_tests() {
        assert_eq!(next_bigger_number(9), None, "{ERR_MSG}");
        assert_eq!(next_bigger_number(12), Some(21), "{ERR_MSG}");
        assert_eq!(next_bigger_number(513), Some(531), "{ERR_MSG}");
        assert_eq!(next_bigger_number(2017), Some(2071), "{ERR_MSG}");
        assert_eq!(next_bigger_number(414), Some(441), "{ERR_MSG}");
        assert_eq!(next_bigger_number(144), Some(414), "{ERR_MSG}");
    }
}
