//! Title: Primes in the Last Digits of Huge Numbers
//! Link: https://www.codewars.com/kata/55e61967663140aafb0001d6
//! Kata ID: 55e61967663140aafb0001d6
//! Rank: 5kyu
//! Completed: 2025-12-03
#![allow(dead_code)]

fn kth_last_digit_prime(k: u32) -> (u32, u32) {
    const MOD: i64 = 1_000_000_000; // 10^9，用于提取后九位

    // 初始化序列的前5项
    // f(0)=0, f(1)=1, f(2)=1, f(3)=2, f(4)=4
    // 滑动窗口: [f(n-4), f(n-3), f(n-2), f(n-1), f(n)]
    let mut f = [0i64, 1, 1, 2, 4];

    let mut count = 0u32;
    let mut n = 5u32; // 从 n = 5 开始计算

    loop {
        // 计算 f(n) = f(n-1) + f(n-2) - f(n-3) + f(n-4) - f(n-5)
        // f[4]=f(n-1), f[3]=f(n-2), f[2]=f(n-3), f[1]=f(n-4), f[0]=f(n-5)
        let next = (f[4] + f[3] - f[2] + f[1] - f[0]).rem_euclid(MOD);

        // 更新滑动窗口
        f[0] = f[1];
        f[1] = f[2];
        f[2] = f[3];
        f[3] = f[4];
        f[4] = next;

        // 检查后九位是否为素数（必须是9位数）
        if next >= 100_000_000 && is_prime(next as u64) {
            count += 1;
            if count == k {
                return (n, next as u32);
            }
        }

        n += 1;
    }
}

// Miller-Rabin 素性测试（确定性版本）
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // 将 n-1 写成 2^r * d 的形式
    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    // 对于 u64 范围的数，这些见证数足以确定性判断
    let witnesses = [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    'witness_loop: for &a in &witnesses {
        if a >= n {
            continue;
        }

        let mut x = mod_pow(a, d, n);

        if x == 1 || x == n - 1 {
            continue;
        }

        for _ in 0..r - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                continue 'witness_loop;
            }
        }

        return false;
    }

    true
}

// 快速模幂运算: (base^exp) % modulus
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        exp >>= 1;
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
    }
    result
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::kth_last_digit_prime;

    fn dotest(k: u32, expected: (u32, u32)) {
        let actual = kth_last_digit_prime(k);
        assert!(
            actual == expected,
            "With k = {k}\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest(1, (92, 480150779));
        dotest(2, (98, 922495169));
        dotest(3, (110, 603021049));
        dotest(4, (122, 931883761));
        dotest(5, (134, 170886697));
    }
}
