//! Title: interger:recreation one
//! Link: https://www.codewars.com/kata/55aa075506463dac6600010d
//! Kata ID: 55aa075506463dac6600010d
//! Rank: 5kyu
//! Completed: 2025-12-02
//!
//! TODO: 在这里写你的解法
//!
//! 把 Codewars 的测试用例复制到下面 tests 模块里即可（推荐保留原样,便于以后复习）

fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    let mut res = Vec::new();

    for i in m..=n {
        let mut sum: u128 = 0;
        let sqrt_i = (i as f64).sqrt() as u64;
        for d in 1..=sqrt_i {
            if i % d == 0 {
                let other = i / d;
                let d_sq = (d as u128) * (d as u128);
                sum += d_sq;
                if other != d {
                    let o_sq = (other as u128) * (other as u128);
                    sum += o_sq;
                }
            }
        }

        if sum > 0 {
            let r = (sum as f64).sqrt() as u128;
            if r * r == sum || (r + 1) * (r + 1) == sum {
                res.push((i, sum as u64));
            }
        }
    }

    res
}

fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(list_squared(m, n), exp)
}

#[test]
fn basics_list_squared() {
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(42, 250, vec![(42, 2500), (246, 84100)]);
    testing(300, 600, vec![]);
}
