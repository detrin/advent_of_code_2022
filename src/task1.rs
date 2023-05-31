use std::io::{self, BufRead};

pub fn task1_part1_v1() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut current_sum = 0;
    let mut max_value = 0;

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            max_value = max_value.max(current_sum);
            current_sum = 0;
        } else if let Ok(value) = line.parse::<u32>() {
            current_sum += value;
        }
    }

    max_value = max_value.max(current_sum);
    println!("{}", max_value);
}

pub fn task1_part1_v2() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!(
        "{}",
        input
            .join("\n")
            .split("\n\n")
            .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
            .max()
            .unwrap(),
    );
}

pub fn task1_part2_v1() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut summed_calories = input
        .join("\n")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();
    summed_calories.sort_unstable_by(|a, b| b.cmp(a));
    let top_three = summed_calories.iter().take(3).sum::<u32>();

    println!("{}", top_three);
}
