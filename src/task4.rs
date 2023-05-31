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

fn parse_numbers(line: &str) -> Option<(usize, usize, usize, usize)> {
    let mut iter = line.split(",");
    
    if let (Some(range1), Some(range2)) = (iter.next(), iter.next()) {
        let (a, b) = parse_range(range1)?;
        let (c, d) = parse_range(range2)?;
        return Some((a, b, c, d));
    }
    
    None
}

fn parse_range(range: &str) -> Option<(usize, usize)> {
    let mut iter = range.split("-");
    
    if let (Some(a_str), Some(b_str)) = (iter.next(), iter.next()) {
        let a = a_str.trim().parse().ok()?;
        let b = b_str.trim().parse().ok()?;
        return Some((a, b));
    }
    
    None
}
pub fn task4_part1_v2() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut fully_contained = 0;

    while let Some(Ok(line)) = lines.next() {
        if let Some((a, b, c, d)) = parse_numbers(&line) {
            if (a <= c && b >= d) || (c <= a && d >= b) {
                fully_contained += 1;
            } 
        }
    }
    println!("{}", fully_contained);
}

pub fn task4_part2_v2() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut fully_contained = 0;

    while let Some(Ok(line)) = lines.next() {
        if let Some((a, b, c, d)) = parse_numbers(&line) {
            if ! (a < d && c > b) && ! (c < b && a > d){
                fully_contained += 1;
            } 
        }
    }
    println!("{}", fully_contained);
}