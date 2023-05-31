use std::io::{self, BufRead};

pub fn task3_part1_v1() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut total_score = 0;
    while let Some(Ok(line)) = lines.next() {
        let (first, second) = line.split_at(line.len() / 2);
        for ch in second.chars() {
            if let Some(_pos) = first.find(ch) {
                total_score += (ch.to_ascii_lowercase() as u32 - 'a' as u32 + 1)
                    + if ch.is_ascii_lowercase() { 0 } else { 26 };
                break;
            }
        }
    }
    println!("{}", total_score);
}

pub fn task3_part2_v1() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut total_score = 0;
    // iterate by 3
    for i in 0..input.len() / 3 {
        let (first, second) = (&input[i * 3], &input[i * 3 + 1]);
        let mut common = String::new();
        for ch in second.chars() {
            if let Some(_pos) = first.find(ch) {
                common.push(ch);
            }
        }
        let (second, third) = (&common, &input[i * 3 + 2]);
        let mut common = String::new();
        for ch in third.chars() {
            if let Some(_pos) = second.find(ch) {
                common.push(ch);
                break;
            }
        }
        let ch = common.chars().nth(0).unwrap();
        total_score += (ch.to_ascii_lowercase() as u32 - 'a' as u32 + 1)
            + if ch.is_ascii_lowercase() { 0 } else { 26 };
    }
    println!("{}", total_score);
}
