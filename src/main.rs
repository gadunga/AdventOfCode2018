extern crate hamming;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let path = Path::new("/Users/richardmeester/GitHub/AdventOfCode2018/src/d2input.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", 
            display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", 
            display, why.description()),
        Ok(_) => (),
    }

    //d1(&s);
    d2(&s)
}

fn d1(data: &str) {
    let freqs: Vec<i32> = data.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    println!("Part 1: {}", freqs.iter().sum::<i32>());

    let mut encountered_freqs = HashSet::new();
    let mut current_freq = 0;

    encountered_freqs.insert(current_freq);
    loop {
        for x in &freqs {
            current_freq += x;
            if encountered_freqs.contains(&current_freq) {
                println!("Part 2: {:?}", current_freq);
                return;
            }
            encountered_freqs.insert(current_freq);
        }
    }  
}

fn d2(data: &str){
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
        .map(|(c1, c2)| c1)
        .collect();
    println!("Part 2: {}", res);
}
