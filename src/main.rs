use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let path = Path::new("C:\\Users\\richard\\GitHub\\AdventOfCode2018\\src\\d1input.txt");
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

    d1(&s); 
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
