use std::collections::HashSet;

#[allow(dead_code)]
pub fn solve(data: &str) {
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