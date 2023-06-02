use std::io::{self, BufRead};

pub fn task9_part1_v1() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut head_x = 0;
    let mut head_y = 0;
    let mut head_x_prev = 0;
    let mut head_y_prev = 0;
    let mut tail_x = 0;
    let mut tail_y = 0;
    let mut visited = std::collections::HashMap::new();

    for line in input {
        let (direction, number) = line.split_at(line.find(" ").unwrap());
        let number = number[1..].parse::<usize>().unwrap();

        for _ in 0..number {
            head_x_prev = head_x;
            head_y_prev = head_y;
            match direction {
                "R" => {
                    head_x += 1;
                }
                "L" => {
                    head_x -= 1;
                }
                "U" => {
                    head_y += 1;
                }
                "D" => {
                    head_y -= 1;
                }
                _ => {}
            }
            let diff_x: i32 = head_x - tail_x;
            let diff_y: i32 = head_y - tail_y;

            if diff_x.abs() > 1 || diff_y.abs() > 1 {
                tail_x = head_x_prev;
                tail_y = head_y_prev;
            }
            if !visited.contains_key(&(tail_x, tail_y)) {
                visited.insert((tail_x, tail_y), true);
            }
        }
    
    }

    println!("{}", visited.len());
}