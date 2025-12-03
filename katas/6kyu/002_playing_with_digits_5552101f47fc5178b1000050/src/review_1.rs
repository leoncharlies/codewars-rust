//! Review #1
//! Date: 2025-12-03
#![allow(dead_code)]

fn dig_pow(n: i64, p: i32) -> i64 {
    let sum: i64 = n
        .to_string()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let digits = c.to_digit(10).unwrap() as i64;
            digits.pow(i as u32 + p as u32)
        })
        .sum();
    if sum % n == 0 {
        return sum / n;
    } else {
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i64, p: i32, exp: i64) -> () {
        println!(" n: {:?};", n);
        println!("p: {:?};", p);
        let ans = dig_pow(n, p);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(89, 1, 1);
        dotest(92, 1, -1);
        dotest(46288, 3, 51);
    }
}
