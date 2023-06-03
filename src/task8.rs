use std::io::{self, BufRead};

pub fn task8_part1_v1() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        map.push(row);
    }
    // println!("{}", input);
    // println!("{:?}", map);

    let mut visible = Vec::new();
    for row in &map {
        let mut visible_row = Vec::new();
        for _ in row {
            visible_row.push(false);
        }
        visible.push(visible_row);
    }

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut highest_x1 = true;
            for k in 0..i {
                if map[k][j] >= map[i][j] {
                    highest_x1 = false;
                    break;
                }
            }
            let mut highest_x2 = true;
            for k in i + 1..map.len() {
                if map[k][j] >= map[i][j] {
                    highest_x2 = false;
                    break;
                }
            }
            let mut highest_y1 = true;
            for k in 0..j {
                if map[i][k] >= map[i][j] {
                    highest_y1 = false;
                    break;
                }
            }
            let mut highest_y2 = true;
            for k in j + 1..map[i].len() {
                if map[i][k] >= map[i][j] {
                    highest_y2 = false;
                    break;
                }
            }

            if highest_x1 || highest_x2 || highest_y1 || highest_y2 {
                visible[i][j] = true;
            }
        }
    }

    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if visible[i][j] {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

pub fn task8_part2_v1() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        map.push(row);
    }
    // println!("{}", input);
    // println!("{:?}", map);

    let mut visible = Vec::new();
    for row in &map {
        let mut visible_row = Vec::new();
        for _ in row {
            visible_row.push(false);
        }
        visible.push(visible_row);
    }
    let mut score_max = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut score_x1 = 0;
            for k in (0..i).rev() {
                score_x1 += 1;
                if map[k][j] >= map[i][j] {
                    break;
                }
            }
            let mut score_x2 = 0;
            for k in i + 1..map.len() {
                score_x2 += 1;
                if map[k][j] >= map[i][j] {
                    break;
                }
            }
            let mut score_y1 = 0;
            for k in (0..j).rev() {
                score_y1 += 1;
                if map[i][k] >= map[i][j] {
                    break;
                }
            }
            let mut score_y2 = 0;
            for k in j + 1..map[i].len() {
                score_y2 += 1;
                if map[i][k] >= map[i][j] {
                    break;
                }
            }

            let score = score_x1 * score_x2 * score_y1 * score_y2;
            if score > score_max {
                score_max = score;
            }
        }
    }

    println!("{}", score_max);
}
