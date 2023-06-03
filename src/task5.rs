use regex::Regex;
use std::io::{self, BufRead};

pub fn task5_part1_v1() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let re = Regex::new(r"\[[A-Z]\]|   ").unwrap();
    let mut canvas = Vec::new();

    for line in input.iter().rev() {
        if !line.contains("[") {
            continue;
        }
        let mut row = Vec::new();
        for capture in re.captures_iter(&line.replace(" [", "[")) {
            let matched = capture.get(0).unwrap().as_str().to_string();
            let letter = matched.chars().nth(1).unwrap();
            row.push(letter);
        }
        canvas.push(row);
    }

    let mut rotated = Vec::new();
    for i in 0..canvas[0].len() {
        let mut row = Vec::new();
        for j in 0..canvas.len() {
            if canvas[j][i] != ' ' {
                row.push(canvas[j][i]);
            }
        }
        rotated.push(row);
    }

    for line in input.iter().filter(|line| line.contains("move")) {
        let mut iter = line.split(" ");
        iter.next();
        let cnt = iter.next().unwrap().parse::<usize>().unwrap();
        iter.next();
        let from_block = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        iter.next();
        let to_block = iter.next().unwrap().parse::<usize>().unwrap() - 1;

        let mut letters = Vec::new();
        for _ in 0..cnt {
            letters.push(rotated[from_block].pop().unwrap());
        }

        for letter in letters.iter() {
            rotated[to_block].push(*letter);
        }
    }

    for row in rotated.iter() {
        if row.len() == 0 {
            print!(" ");
        } else {
            print!("{}", row[row.len() - 1]);
        }
    }
    println!();
}

pub fn task5_part2_v1() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let re = Regex::new(r"\[[A-Z]\]|   ").unwrap();
    let mut canvas = Vec::new();

    for line in input.iter().rev() {
        if !line.contains("[") {
            continue;
        }
        let mut row = Vec::new();
        for capture in re.captures_iter(&line.replace(" [", "[")) {
            let matched = capture.get(0).unwrap().as_str().to_string();
            let letter = matched.chars().nth(1).unwrap();
            row.push(letter);
        }
        canvas.push(row);
    }

    let mut rotated = Vec::new();
    for i in 0..canvas[0].len() {
        let mut row = Vec::new();
        for j in 0..canvas.len() {
            if canvas[j][i] != ' ' {
                row.push(canvas[j][i]);
            }
        }
        rotated.push(row);
    }

    for line in input.iter().filter(|line| line.contains("move")) {
        let mut iter = line.split(" ");
        iter.next();
        let cnt = iter.next().unwrap().parse::<usize>().unwrap();
        iter.next();
        let from_block = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        iter.next();
        let to_block = iter.next().unwrap().parse::<usize>().unwrap() - 1;

        let mut letters = Vec::new();
        for _ in 0..cnt {
            letters.push(rotated[from_block].pop().unwrap());
        }

        for letter in letters.iter().rev() {
            rotated[to_block].push(*letter);
        }
    }

    for row in rotated.iter() {
        if row.len() == 0 {
            print!(" ");
        } else {
            print!("{}", row[row.len() - 1]);
        }
    }
    println!();
}
