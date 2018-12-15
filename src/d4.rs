use chrono::{NaiveDateTime, Timelike};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Message<'a> {
    date_time: NaiveDateTime,
    message: &'a str
}

#[allow(dead_code)]
pub fn solve(data: &str) {
    let date_regex = Regex::new(r"\[([\w\s\d\-:]*)\]").unwrap();
    let guard_regex = Regex::new(r"Guard #(\d+)").unwrap();
    let mut messages = Vec::new();

    data.lines().for_each(|s| {
        for cap in date_regex.captures_iter(s) {
            let date  = NaiveDateTime::parse_from_str(&cap[1], "%F %R").unwrap();
            
            messages.push(Message { date_time: date, message: s });
        }
    });

    let mut map = HashMap::new();
    let mut current_guard: usize = 0;
    let mut start: usize = 0;
    let mut end: usize = 0;

    messages.sort_by(|a, b| a.date_time.cmp(&b.date_time));
    messages.iter()
        .for_each(|x| {
            if x.message.contains("Guard") {
                for cap in guard_regex.captures_iter(x.message) {
                    current_guard = cap[1].parse::<usize>().unwrap();
                }
            }
            if x.message.contains("falls asleep") {
               start = x.date_time.minute() as usize;
            }
            if x.message.contains("wakes up") {
                end = x.date_time.minute() as usize;
                let counts = map.entry(current_guard).or_insert(vec![0; 60]);
                for t in start..end {
                    counts[t] += 1;
                }
            }
        });
    
    let mut candidate_guard: usize = 0;
    let mut total_time_asleep = 0;
    map.iter().for_each(|(id, times)| {
        let sum = times.iter().sum::<i32>();
        if sum > total_time_asleep {
            total_time_asleep = sum;
            candidate_guard = *id;
        }
    });
    
    let mut candidate_time = 0;
    let mut largest_count = 0;
    for (i, count) in map[&candidate_guard].iter().enumerate() {
        if *count > largest_count {
            largest_count = *count;
            candidate_time = i;
        }
    }

    println!("Part 1: {}", candidate_time * candidate_guard);

    candidate_guard = 0;
    candidate_time = 0;
    largest_count = 0;
    map.iter()
        .for_each(|(id, times)| {
            for (i, count) in times.iter().enumerate() {
                if *count > largest_count {
                    largest_count = *count;
                    candidate_time = i;
                    candidate_guard = *id;
                }
            }
        });

    println!("Part 2: {}", candidate_time * candidate_guard);
}