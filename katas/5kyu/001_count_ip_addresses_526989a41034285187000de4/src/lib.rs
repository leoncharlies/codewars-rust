//! Title: Count IP Addresses
//! Link: https://www.codewars.com/kata/526989a41034285187000de4
//! Kata ID: 526989a41034285187000de4
//! Rank: 5kyu
//! Completed: 2025-12-01
//!
//! TODO: 在这里写你的解法
//!
//! 把 Codewars 的测试用例复制到下面 tests 模块里即可（推荐保留原样，便于以后复习）
#![allow(dead_code)]

fn ips_between(start: &str, end: &str) -> u32 {
    fn convert2num(input: &str) -> u32 {
        let default_num: u32 = 256;
        let nums: Vec<&str> = input.split(".").collect();
        if nums.len() != 4 {
            return 0;
        }
        let mut result = 0;
        for (i, num) in nums.iter().enumerate() {
            let n: u32 = num.parse().unwrap();
            let exp = 3 - i;
            result += n * default_num.pow(exp as u32);
        }
        result
    }
    let start_address = convert2num(start);
    let end_address = convert2num(end);
    start_address.abs_diff(end_address)
}

#[cfg(test)]
mod tests {
    use super::ips_between;

    #[test]
    fn basic() {
        assert_eq!(
            ips_between("10.0.0.0", "10.0.0.50"),
            50,
            "\nYour result (left) did not match the expected result (right)"
        );
        assert_eq!(
            ips_between("20.0.0.10", "20.0.1.0"),
            246,
            "\nYour result (left) did not match the expected result (right)"
        );
    }
}

#[cfg(test)]
mod review_1;
