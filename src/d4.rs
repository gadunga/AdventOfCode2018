use regex::Regex;
use chrono::NaiveDateTime;

#[allow(dead_code)]
pub fn solve(data: &str) {
    let date_regex = Regex::new(r"\[([\w\s\d\-:]*)\]").unwrap();
    //let guard_regex = Regex::new(r"Guard #(\d+)").unwrap();
    let mut dates = Vec::new();

    data.lines().for_each(|s| {
        for cap in date_regex.captures_iter(s) {
            let custom = NaiveDateTime::parse_from_str(&cap[1], "%F %R").unwrap();
            dates.push(custom);
        }
    });

    println!("{:?}", dates);
    dates.sort_by(|a, b| a.cmp(b));
    println!("{:?}", dates);
}