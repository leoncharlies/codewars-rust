//! Title: Fateful Findings
//! Link: https://www.codewars.com/kata/67330f514a2d47c019c67340
//! Kata ID: 67330f514a2d47c019c67340
//! Rank: 5kyu
//! Completed: 2025-12-04
#![allow(dead_code)]

struct State {
    home: i32,
    gifts: Vec<(i32, char)>,
    result: String,
    last_dir: Option<i32>,
}

impl State {
    fn new(s: &str) -> Self {
        let home = s.chars().position(|c| c == '@').unwrap() as i32;
        let gifts = s
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_alphabetic())
            .map(|(i, c)| (i as i32, c))
            .collect();

        State {
            home,
            gifts,
            result: String::new(),
            last_dir: None,
        }
    }

    fn is_large(c: char) -> bool {
        c.is_uppercase()
    }

    fn weight(c: char) -> i32 {
        if Self::is_large(c) {
            2
        } else {
            1
        }
    }

    fn fanciness(c: char) -> i32 {
        -(c as i32)
    }

    fn collect_gifts(&mut self) -> String {
        while !self.gifts.is_empty() {
            if let Some((first, second)) = self.choose_gifts() {
                // 先收集第二个礼物(如果存在)
                if let Some(second_gift) = second {
                    self.collect_gift(second_gift);
                }
                // 再收集第一个礼物
                self.collect_gift(first);
            } else {
                // 无法决定
                return "?".to_string();
            }
        }
        self.result.clone()
    }

    fn choose_gifts(&mut self) -> Option<((i32, char), Option<(i32, char)>)> {
        if self.gifts.is_empty() {
            return None;
        }

        // 找到最佳的第一个礼物
        let first = self.choose_first_gift()?;

        // 如果是小礼物,尝试找第二个
        let second = if !Self::is_large(first.1) {
            self.choose_second_gift(first)
        } else {
            None
        };

        Some((first, second))
    }

    fn choose_first_gift(&self) -> Option<(i32, char)> {
        let mut candidates = self.gifts.clone();

        // 按距离排序
        candidates.sort_by_key(|(pos, _)| (self.home - pos).abs());
        let min_dist = (self.home - candidates[0].0).abs();
        candidates.retain(|(pos, _)| (self.home - pos).abs() == min_dist);

        if candidates.len() == 1 {
            return Some(candidates[0]);
        }

        // 按重量排序(降序)
        candidates.sort_by_key(|(_, c)| -Self::weight(*c));
        let max_weight = Self::weight(candidates[0].1);
        candidates.retain(|(_, c)| Self::weight(*c) == max_weight);

        if candidates.len() == 1 {
            return Some(candidates[0]);
        }

        // 按精美度排序(降序)
        candidates.sort_by_key(|(_, c)| Self::fanciness(*c));
        let max_fancy = Self::fanciness(candidates[0].1);
        candidates.retain(|(_, c)| Self::fanciness(*c) == max_fancy);

        if candidates.len() == 1 {
            return Some(candidates[0]);
        }

        // 需要根据last_dir决定
        if let Some(last_dir) = self.last_dir {
            // 选择相反方向
            let opposite_dir = -last_dir;
            for &(pos, c) in &candidates {
                let dir = (pos - self.home).signum();
                if dir == opposite_dir {
                    return Some((pos, c));
                }
            }
        }

        // 无法决定
        None
    }

    fn choose_second_gift(&self, first: (i32, char)) -> Option<(i32, char)> {
        let (first_pos, _) = first;
        let dir = (first_pos - self.home).signum();
        let dist_to_home = (first_pos - self.home).abs();

        // 找同方向的小礼物
        let mut same_dir_gifts: Vec<_> = self
            .gifts
            .iter()
            .filter(|&&(pos, c)| {
                pos != first_pos
                    && !Self::is_large(c)
                    && (pos - self.home).signum() == dir
                    && (pos - self.home).abs() > (first_pos - self.home).abs()
            })
            .copied()
            .collect();

        same_dir_gifts.sort_by_key(|(pos, _)| (pos - first_pos).abs());

        // 规则1: 距离 <= 到家距离
        if let Some(&second) = same_dir_gifts.first() {
            if (second.0 - first_pos).abs() <= dist_to_home {
                return Some(second);
            }
        }

        // 规则2: 不比对面最近的礼物远
        let opposite_gifts: Vec<_> = self
            .gifts
            .iter()
            .filter(|&&(pos, _)| (pos - self.home).signum() == -dir)
            .copied()
            .collect();

        if let (Some(&second), Some(opposite)) = (
            same_dir_gifts.first(),
            opposite_gifts
                .iter()
                .min_by_key(|(pos, _)| (pos - self.home).abs()),
        ) {
            if (second.0 - self.home).abs() <= (opposite.0 - self.home).abs() {
                return Some(second);
            }
        }

        None
    }

    fn collect_gift(&mut self, gift: (i32, char)) {
        self.result.push(gift.1);
        self.gifts.retain(|&g| g != gift);
        self.last_dir = Some((gift.0 - self.home).signum());
    }
}

fn f(s: &str) -> String {
    let mut state = State::new(s);
    state.collect_gifts()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::f;

    macro_rules! fixed_test {
        ($name:ident, $input:literal, $expected:literal) => {
            #[test]
            fn $name() {
                let actual = f($input);
                let expected = $expected;
                assert!(
                    actual == expected,
                    "Test failed.\nInput:    {}\nActual:   {actual}\nExpected: {expected}\n",
                    $input
                );
            }
        };
    }

    mod _1_sample_cases {
        use super::*;

        fixed_test!(zig_zag, "a..Z..b..@..c..Y..d", "bcYZad");
        fixed_test!(
            long_distance_runner,
            "z....y....x....@....a....b....c",
            "bayxcz"
        );
        fixed_test!(cluster_conundrum, "ABCdef@abcDEF", "baefcdCDBEAF");
        fixed_test!(double_trouble, "..ab..cd..@..ef..gh..", "cdfeabhg");
        fixed_test!(mixed_bag, "z.Y.x.@.A.b.C.d", "AxYdbCz");
        fixed_test!(fanciness_factor, "Z.y.x.@.a.B.c", "ayxBZc");
    }

    mod _2_basic_rules {
        use super::*;

        fixed_test!(no_gifts, "......@.....", "");
        fixed_test!(one_gift, "...z..@.....", "z");
        fixed_test!(closest_gift_first, "...z..@...A.", "zA");
        fixed_test!(larger_gift_first_when_equidistant, "...a..@..Z..", "Za");
        fixed_test!(
            fancier_gift_first_when_equal_distance_and_weight,
            "...a..@..z..",
            "az"
        );
        fixed_test!(
            tie_breaker_picks_opposite_of_last,
            "...a.....@.Z...a..",
            "Zaa"
        );
        fixed_test!(no_choice_possible, ".Z.a..@..a..", "?");
    }

    mod _3_advanced_rules {
        use super::*;

        fixed_test!(second_gift_close_enough, "...b..a..@........", "ba");
        fixed_test!(
            second_gift_close_enough_bypassing_large_gift,
            "...b.Za..@........",
            "baZ"
        );
        fixed_test!(
            second_gift_closer_than_opposite,
            "..z...a..@......b.",
            "zab"
        );
        fixed_test!(
            second_gift_close_than_opposite_bypassing_large_gift,
            "..zA..a..@......b.",
            "zaAb"
        );
        fixed_test!(second_gift_too_large, "...B..a..@..z.....", "azB");
        fixed_test!(second_gift_too_far, "..b...a..@..z.....", "azb");
        fixed_test!(
            tie_breaker_picks_second_gift_opposite_last,
            ".b.a.....@.Z...a..",
            "Zbaa"
        );
    }
}
