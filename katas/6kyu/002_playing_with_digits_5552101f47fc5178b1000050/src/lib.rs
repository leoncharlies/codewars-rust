//! Title: Playing with digits
//! Link: https://www.codewars.com/kata/5552101f47fc5178b1000050
//! Kata ID: 5552101f47fc5178b1000050
//! Rank: 6kyu
//! Completed: 2025-12-03
#![allow(dead_code)]

fn dig_pow(n: i64, p: i32) -> i64 {
    // your code
    let mut sum: i64 = 0;
    let mut counter = p;
    for num in n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
    {
        sum += num.pow(counter as u32);
        counter += 1;
    }
    if sum % n != 0 {
        return -1;
    } else {
        return sum / n;
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

#[cfg(test)]
mod review_1;
