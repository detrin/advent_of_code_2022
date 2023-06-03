use std::io::{self, BufRead};

pub fn task12_part1_v1() {
    let stdin = io::stdin();
    let mut map_str = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut pos_start_x = 0;
    let mut pos_start_y = 0;
    let mut pos_end_x = 0;
    let mut pos_end_y = 0;

    for (y, line) in map_str.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                pos_start_x = x;
                pos_start_y = y;
            } else if *c == 'E' {
                pos_end_x = x;
                pos_end_y = y;
            }
        }
    }
    map_str[pos_start_y][pos_start_x] = 'a';
    map_str[pos_end_y][pos_end_x] = 'z';

    let map = map_str
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| (*c as u8 - 'a' as u8) as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let path = pathfinding::directed::bfs::bfs(
        &(pos_end_y, pos_end_x),
        |(y, x)| {
            let x = *x;
            let y = *y;
            let mut neighbors = Vec::new();
            if y > 0 && map[y - 1][x] >= map[y][x].saturating_sub(1) {
                neighbors.push((y - 1, x));
            }
            if y < map.len() - 1 && map[y + 1][x] >= map[y][x].saturating_sub(1) {
                neighbors.push((y + 1, x));
            }
            if x > 0 && map[y][x - 1] >= map[y][x].saturating_sub(1) {
                neighbors.push((y, x - 1));
            }
            if x < map[0].len() - 1 && map[y][x + 1] >= map[y][x].saturating_sub(1) {
                neighbors.push((y, x + 1));
            }
            neighbors
        },
        |&p| p == (pos_start_y, pos_start_x),
    );
    for (y, x) in path.clone().unwrap() {
        map_str[y][x] = '#';
    }
    // for line in map_str {
    //     println!("{}", line.iter().collect::<String>());
    // }

    let path_length = path.clone().unwrap().len() - 1;

    println!("{}", path_length);
}

pub fn task12_part2_v1() {
    let stdin = io::stdin();
    let mut map_str = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut pos_start_x = 0;
    let mut pos_start_y = 0;
    let mut pos_end_x = 0;
    let mut pos_end_y = 0;

    for (y, line) in map_str.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                pos_start_x = x;
                pos_start_y = y;
            } else if *c == 'E' {
                pos_end_x = x;
                pos_end_y = y;
            }
        }
    }
    map_str[pos_start_y][pos_start_x] = 'a';
    map_str[pos_end_y][pos_end_x] = 'z';

    let map = map_str
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| (*c as u8 - 'a' as u8) as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let path = pathfinding::directed::bfs::bfs(
        &(pos_end_y, pos_end_x),
        |(y, x)| {
            let x = *x;
            let y = *y;
            let mut neighbors = Vec::new();
            if y > 0 && map[y - 1][x] >= map[y][x].saturating_sub(1) {
                neighbors.push((y - 1, x));
            }
            if y < map.len() - 1 && map[y + 1][x] >= map[y][x].saturating_sub(1) {
                neighbors.push((y + 1, x));
            }
            if x > 0 && map[y][x - 1] >= map[y][x].saturating_sub(1) {
                neighbors.push((y, x - 1));
            }
            if x < map[0].len() - 1 && map[y][x + 1] >= map[y][x].saturating_sub(1) {
                neighbors.push((y, x + 1));
            }
            neighbors
        },
        |&p| map[p.0][p.1] == 0,
    );
    for (y, x) in path.clone().unwrap() {
        map_str[y][x] = '#';
    }
    // for line in map_str {
    //     println!("{}", line.iter().collect::<String>());
    // }

    let path_length = path.clone().unwrap().len() - 1;

    println!("{}", path_length);
}
