use std::io::{self, BufRead};

pub fn task4_part1_v1() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut fully_contained = 0;
    while let Some(Ok(line)) = lines.next() {
        let (first, second) = line.split_at(line.find(",").unwrap());

        let (a, b) = first.split_at(first.find("-").unwrap());
        let (c, d) = second[1..].split_at(second[1..].find("-").unwrap());

        let a = a.parse::<usize>().unwrap();
        let b = b[1..].parse::<usize>().unwrap();
        let c = c.parse::<usize>().unwrap();
        let d = d[1..].parse::<usize>().unwrap();
        
        if a <= c && b >= d {
            fully_contained += 1;
        } else if c <= a && d >= b {
            fully_contained += 1;
        }
    }
    println!("{}", fully_contained);
}

pub fn task4_part2_v1() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut fully_contained = 0;
    while let Some(Ok(line)) = lines.next() {
        let (first, second) = line.split_at(line.find(",").unwrap());

        let (a, b) = first.split_at(first.find("-").unwrap());
        let (c, d) = second[1..].split_at(second[1..].find("-").unwrap());

        let a = a.parse::<usize>().unwrap();
        let b = b[1..].parse::<usize>().unwrap();
        let c = c.parse::<usize>().unwrap();
        let d = d[1..].parse::<usize>().unwrap();
        
        if ! (a < d && c > b) && ! (c < b && a > d){
            fully_contained += 1;
        } 
    }
    println!("{}", fully_contained);
}