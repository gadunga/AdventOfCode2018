extern crate hamming;

use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve(data: &str) {
    let mut double_count = 0;
    let mut triple_count = 0;
    data.lines().for_each(|s| {
            let mut map = HashMap::new();
            s.chars().for_each(|c| {
                let count = map.entry(c).or_insert(0);
                *count += 1;
            });
            if map.iter().any(|(_, count)| *count == 2) {
                double_count += 1;
            }

            if map.iter().any(|(_, count)| *count == 3) {
                triple_count += 1;
            }
        });

    println!("Part 1: {}", double_count * triple_count);
    let mut min_distance = std::u64::MAX;
    let mut pair = ("", "");

    data.lines().for_each(|s| {
        data.lines().for_each(|s2| {
            if s2 == s {
                return;
            }
            let distance = hamming::distance(s.as_bytes(), s2.as_bytes());
            if  distance < min_distance {
                min_distance = distance;
                pair = (s, s2);
            }
        })
    });

    let res: String = pair.0
        .chars()
        .zip( pair.1.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _)| c1)
        .collect();
    println!("Part 2: {}", res);
}