#[allow(dead_code)]
pub fn solve(data: &str) {
    println!("Part 1: {}", scan(data));

    let delta = 'a' as u8 - 'A' as u8;
    let mut smallest = std::usize::MAX;
    for a in b'a'..b'z' + 1 {
        let new_data: String = data.chars().filter(|&c| c != a as char && c != (a - delta) as char).collect();
        let len = scan(new_data.as_str());
        if len < smallest {
            smallest = len;
        }
    }

    println!("Part 2: {}", smallest);

}

fn scan(data: &str) -> usize {
    let mut polymers = Vec::new();
    let delta = 'a' as u8 - 'A' as u8;

    data.chars()
        .for_each(|c| {
        if polymers.is_empty() {
            polymers.push(c);
            return;
        }

        let last = *polymers.last().unwrap() as i32;
        let current = c as i32;
        if (last - current).abs() == delta as i32 {
            polymers.pop();
        } else {
            polymers.push(c);
        }
    });

    polymers.len()
}