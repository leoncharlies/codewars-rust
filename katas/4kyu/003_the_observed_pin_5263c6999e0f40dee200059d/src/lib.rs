//! Title: The observed PIN
//! Link: https://www.codewars.com/kata/5263c6999e0f40dee200059d
//! Kata ID: 5263c6999e0f40dee200059d
//! Rank: 4kyu
//! Completed: 2025-12-06
#![allow(dead_code)]

use itertools::Itertools;

fn get_pins(observed: &str) -> Vec<String> {
    let mut v = Vec::new();
    for c in observed.chars() {
        let vn = match c {
            '1' => vec![1, 2, 4],
            '2' => vec![1, 2, 3, 5],
            '3' => vec![2, 3, 6],
            '4' => vec![1, 4, 5, 7],
            '5' => vec![2, 4, 5, 6, 8],
            '6' => vec![3, 5, 6, 9],
            '7' => vec![4, 7, 8],
            '8' => vec![5, 7, 8, 9, 0],
            '9' => vec![6, 8, 9],
            '0' => vec![8, 0],
            _ => vec![],
        };
        v.push(vn);
    }
    // let mut result = vec![vec![]];
    // for arr in v {
    //     let mut newresult = Vec::new();
    //     for prefix in &result {
    //         for &item in &arr {
    //             let mut new_prefix = prefix.clone();
    //             new_prefix.push(item);
    //             newresult.push(new_prefix);
    //         }
    //     }
    //     result = newresult;
    // }
    let result: Vec<Vec<i32>> = v
        .iter()
        .map(|v| v.iter().copied())
        .multi_cartesian_product()
        .collect();
    result
        .iter()
        .map(|s| s.iter().map(|v| v.to_string()).collect::<String>())
        .collect::<Vec<String>>()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::get_pins;
    use itertools::Itertools;

    #[test]
    fn sample_tests() {
        assert_eq!(
            get_pins("8").iter().sorted().collect::<Vec<&String>>(),
            vec!["0", "5", "7", "8", "9"]
        );
        assert_eq!(
            get_pins("11").iter().sorted().collect::<Vec<&String>>(),
            vec!["11", "12", "14", "21", "22", "24", "41", "42", "44"]
        );
        assert_eq!(
            get_pins("369").iter().sorted().collect::<Vec<&String>>(),
            vec![
                "236", "238", "239", "256", "258", "259", "266", "268", "269", "296", "298", "299",
                "336", "338", "339", "356", "358", "359", "366", "368", "369", "396", "398", "399",
                "636", "638", "639", "656", "658", "659", "666", "668", "669", "696", "698", "699"
            ]
        );
    }
}
