use std::io::{self, BufRead};

pub fn task2_part1_v1() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut total_score = 0;
    while let Some(Ok(line)) = lines.next() {
        let opponet_choice = line.chars().nth(0).unwrap();
        let my_choice = line.chars().nth(2).unwrap();
        let opponet_num_choice = match opponet_choice {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => panic!("Invalid choice"),
        };
        let my_num_choice = match my_choice {
            'X' => 0,
            'Y' => 1,
            'Z' => 2,
            _ => panic!("Invalid choice"),
        };
        total_score += my_num_choice + 1;
        if opponet_num_choice == my_num_choice {
            total_score += 3;
        } else if opponet_num_choice == (my_num_choice + 2) % 3 {
            total_score += 6;
        }
    }
    println!("{}", total_score);
}

pub fn task2_part1_v2() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut total_score = 0;
    while let Some(Ok(line)) = lines.next() {
        let opponet_choice = line.chars().nth(0).unwrap();
        let my_choice = line.chars().nth(2).unwrap();
        let opponet_num_choice = match opponet_choice {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => panic!("Invalid choice"),
        };
        let my_num_choice = match my_choice {
            'X' => 0,
            'Y' => 1,
            'Z' => 2,
            _ => panic!("Invalid choice"),
        };
        total_score += my_num_choice + 1;
        let difference = (opponet_num_choice + 3 - my_num_choice) % 3;
        total_score += match difference {
            0 => 3,
            2 => 6,
            _ => 0,
        };
    }
    println!("{}", total_score);
}

pub fn task2_part2_v1() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut total_score = 0;
    while let Some(Ok(line)) = lines.next() {
        let opponet_choice = line.chars().nth(0).unwrap();
        let my_choice = line.chars().nth(2).unwrap();
        let opponet_num_choice = match opponet_choice {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => panic!("Invalid choice"),
        };
        let round_outcome = match my_choice {
            'X' => 0,
            'Y' => 1,
            'Z' => 2,
            _ => panic!("Invalid choice"),
        };
        total_score += 3*round_outcome;
        let my_choice = (opponet_num_choice + round_outcome + 2) % 3;
        total_score += my_choice + 1;
    }
    println!("{}", total_score);
}
