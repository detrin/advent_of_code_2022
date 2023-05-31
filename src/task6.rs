use std::io::{self, BufRead};
use std::collections::HashSet;

pub fn task6_part1_v1() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let msg_len = 4;

    for i in 0..line.len()-msg_len {

        let snippet = line[i..i + msg_len].chars().collect::<Vec<char>>();
        let unique_chars = snippet.iter().copied().collect::<HashSet<char>>().len();
        if unique_chars == msg_len {
            println!("{}", i + msg_len);
            break;
        }
    }
}

pub fn task6_part2_v1() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let msg_len = 14;

    for i in 0..line.len()-msg_len {

        let snippet = line[i..i + msg_len].chars().collect::<Vec<char>>();
        let unique_chars = snippet.iter().copied().collect::<HashSet<char>>().len();
        if unique_chars == msg_len {
            println!("{}", i + msg_len);
            break;
        }
    }
}