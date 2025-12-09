//! Title: The Millionth Fibonacci Kata
//! Link: https://www.codewars.com/kata/53d40c1e2f13e331fc000c26
//! Kata ID: 53d40c1e2f13e331fc000c26
//! Rank: 3kyu
//! Completed: 2025-12-09

#![allow(dead_code)]
use num::bigint::BigInt;
use num::traits::{One, Zero};

fn fib(n: i32) -> BigInt {
    // 处理 n = 0 的情况
    if n == 0 {
        return BigInt::zero();
    }

    // 处理负数：fib(-n) = (-1)^(n+1) * fib(n)
    if n < 0 {
        let result = fib_positive(-n);
        // 如果 n 是偶数，结果取反
        return if (-n) % 2 == 0 { -result } else { result };
    }

    fib_positive(n)
}

/// 使用矩阵快速幂计算正数的斐波那契数
fn fib_positive(n: i32) -> BigInt {
    if n == 1 {
        return BigInt::one();
    }

    // 矩阵快速幂算法
    let result = matrix_pow(n as u32);
    result[1][0].clone()
}

/// 2x2 矩阵类型
type Matrix = [[BigInt; 2]; 2];

/// 矩阵乘法
fn matrix_mul(a: &Matrix, b: &Matrix) -> Matrix {
    [
        [
            &a[0][0] * &b[0][0] + &a[0][1] * &b[1][0],
            &a[0][0] * &b[0][1] + &a[0][1] * &b[1][1],
        ],
        [
            &a[1][0] * &b[0][0] + &a[1][1] * &b[1][0],
            &a[1][0] * &b[0][1] + &a[1][1] * &b[1][1],
        ],
    ]
}

/// 计算矩阵 [[1,1],[1,0]] 的 n 次幂
fn matrix_pow(mut n: u32) -> Matrix {
    // 单位矩阵
    let mut result = [
        [BigInt::one(), BigInt::zero()],
        [BigInt::zero(), BigInt::one()],
    ];

    // 基础矩阵 [[1,1],[1,0]]
    let mut base = [
        [BigInt::one(), BigInt::one()],
        [BigInt::one(), BigInt::zero()],
    ];

    // 快速幂：二进制分解
    while n > 0 {
        if n % 2 == 1 {
            result = matrix_mul(&result, &base);
        }
        base = matrix_mul(&base, &base);
        n /= 2;
    }

    result
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod sample_tests {
    use super::fib;
    use num::bigint::BigInt;
    use num::traits::{One, Zero};
    use std::str::FromStr;

    fn dotest(n: i32, expected: BigInt) {
        let actual = fib(n);
        assert!(
            actual == expected,
            "Test failed with n = {n}\nExpected \"{expected:?}\"\nBut got \"{actual:?}\""
        )
    }

    #[test]
    fn small_positive_numbers() {
        dotest(0, BigInt::zero());
        dotest(1, BigInt::one());
        dotest(2, BigInt::one());
        dotest(3, BigInt::from(2));
        dotest(4, BigInt::from(3));
        dotest(5, BigInt::from(5));
    }

    #[test]
    fn small_negative_numbers() {
        dotest(-1, BigInt::from(1));
        dotest(-6, BigInt::from(-8));
        dotest(-96, BigInt::from_str("-51680708854858323072").unwrap());
    }

    #[test]
    fn large_numbers() {
        dotest(
            -500,
            BigInt::from_str("-139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125")
            .unwrap()
        );

        dotest(
            1000,
            BigInt::from_str("43466557686937456435688527675040625802564660517371780402481729089536555417949051890403879840079255169295922593080322634775209689623239873322471161642996440906533187938298969649928516003704476137795166849228875")
            .unwrap()
        );
    }
}
