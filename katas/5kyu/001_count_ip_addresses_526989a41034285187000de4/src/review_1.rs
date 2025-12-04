//! Review #1
//! Date: 2025-12-04
#![allow(dead_code)]

fn ips_between(start: &str, end: &str) -> u32 {
    let s: Vec<u32> = start
        .split('.')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let e: Vec<u32> = end.split('.').map(|x| x.parse::<u32>().unwrap()).collect();
    let s_num = (s[0] << 24) | (s[1] << 16) | (s[2] << 8) | s[3];
    let e_num = (e[0] << 24) | (e[1] << 16) | (e[2] << 8) | e[3];
    e_num - s_num
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
