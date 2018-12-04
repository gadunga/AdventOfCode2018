use std::collections::HashSet;

use regex::Regex;

pub fn solve(data: &str) {
    let re = Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut cloth = vec![vec![Vec::<usize>::new(); 1024]; 1024];
    data.lines().for_each(|s| {
        for cap in re.captures_iter(s) {
            let id = cap[1].parse::<usize>().unwrap();
            let (x, y) = (cap[2].parse::<usize>().unwrap(), cap[3].parse::<usize>().unwrap());
            let (w, h) = (cap[4].parse::<usize>().unwrap(), cap[5].parse::<usize>().unwrap());
            for step_x in 0..w {
                for step_y in 0..h {
                    cloth[x + step_x][y + step_y].push(id);
                }
            }
        }
    });
    let mut count = 0;
    cloth.iter().for_each(
        |r| r.iter().for_each(|c| {
        if c.len() >= 2 {
            count += 1;
        }
    }));

    println!("Part 1: {}", count);
    let mut candidates = HashSet::new();

    cloth.iter_mut().for_each(
        |r| r.iter_mut().for_each(|c| {
        if c.len() == 1 {
            candidates.insert(c[0]);
        }
    }));

    cloth.iter().for_each(
        |r| r.iter().for_each(|c| {
        if c.len() > 1 {
            c.iter().for_each(|i| {candidates.remove(i);});
        }
    }));

    println!("Part 2: {:?}", candidates);
}