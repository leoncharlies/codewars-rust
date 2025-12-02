//! Title: pyramid slide down
//! Link: https://www.codewars.com/kata/551f23362ff852e2ab000037
//! Kata ID: 551f23362ff852e2ab000037
//! Rank: 4kyu
//! Completed: 2025-12-01
//!
//! TODO: 在这里写你的解法
//!
//! 把 Codewars 的测试用例复制到下面 tests 模块里即可（推荐保留原样，便于以后复习）

fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        let small = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
        assert_eq!(
            longest_slide_down(&small),
            23,
            "It should work for small pyramids"
        );
    }

    #[test]
    fn test_small_tricky() {
        let small = vec![
            vec![10],
            vec![10, 20],
            vec![10, 10, 20],
            vec![10, 90, 10, 20],
        ];
        assert_eq!(
            longest_slide_down(&small),
            130,
            "It should work for small, tricky pyramids"
        );
    }

    #[test]
    fn test_medium() {
        let medium = vec![
            vec![75],
            vec![95, 64],
            vec![17, 47, 82],
            vec![18, 35, 87, 10],
            vec![20, 4, 82, 47, 65],
            vec![19, 1, 23, 75, 3, 34],
            vec![88, 2, 77, 73, 7, 63, 67],
            vec![99, 65, 4, 28, 6, 16, 70, 92],
            vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
            vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
            vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
            vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
            vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
            vec![63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
            vec![4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
        ];
        assert_eq!(
            longest_slide_down(&medium),
            1074,
            "It should work for medium pyramids"
        );
    }
}
