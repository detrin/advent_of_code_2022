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

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn update(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn update_with_point(&mut self, other: &Point) {
        self.x = other.x;
        self.y = other.y;
    }

    fn Clone(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    fn Copy(&mut self, other: &Point) {
        self.x = other.x;
        self.y = other.y;
    }
}

fn move_pt(pt: &mut Point, pt_former: &Point) {
    let diff_x: i32 = pt.x - pt_former.x;
    let diff_y: i32 = pt.y - pt_former.y;

    if diff_x.abs() > 1 || diff_y.abs() > 1 {
        let diff_x: i32 = pt.x - pt_former.x;
        let diff_y: i32 = pt.y - pt_former.y;
        let max_diff: i32 = diff_x.abs().max(diff_y.abs());
        let diff_scaled_x = diff_x / max_diff;
        let diff_scaled_y = diff_y / max_diff;
        pt.update(pt_former.x + diff_scaled_x, pt_former.y + diff_scaled_y);
    }
}

pub fn task9_part1_v2() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut visited = std::collections::HashMap::new();
    let points_num = 2;
    let mut points = Vec::new();
    for _ in 0..points_num {
        points.push(Point::new(0, 0));
    }

    for line in input {
        let (direction, number) = line.split_at(line.find(" ").unwrap());
        let number = number[1..].parse::<usize>().unwrap();

        for _ in 0..number {
            let pt_head = &mut points[0];
            match direction {
                "R" => {
                    pt_head.x += 1;
                }
                "L" => {
                    pt_head.x -= 1;
                }
                "U" => {
                    pt_head.y += 1;
                }
                "D" => {
                    pt_head.y -= 1;
                }
                _ => {}
            }
            for i in 1..points_num {
                let (former_slice, current_slice) = points.split_at_mut(i);
                let pt_former = &mut former_slice[i - 1];
                let pt = &mut current_slice[0];
                move_pt(pt, pt_former);
            }
            let pt_tail = &points[points_num - 1];
            if !visited.contains_key(&(pt_tail.x, pt_tail.y)) {
                visited.insert((pt_tail.x, pt_tail.y), true);
            }
        }
    }

    println!("{}", visited.len());
}

pub fn task9_part2_v1() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut visited = std::collections::HashMap::new();
    let points_num = 10;
    let mut points = Vec::new();
    for _ in 0..points_num {
        points.push(Point::new(0, 0));
    }

    for line in input {
        let (direction, number) = line.split_at(line.find(" ").unwrap());
        let number = number[1..].parse::<usize>().unwrap();

        for _ in 0..number {
            let pt_head = &mut points[0];
            match direction {
                "R" => {
                    pt_head.x += 1;
                }
                "L" => {
                    pt_head.x -= 1;
                }
                "U" => {
                    pt_head.y += 1;
                }
                "D" => {
                    pt_head.y -= 1;
                }
                _ => {}
            }
            for i in 1..points_num {
                let (former_slice, current_slice) = points.split_at_mut(i);
                let pt_former = &mut former_slice[i - 1];
                let pt = &mut current_slice[0];
                move_pt(pt, pt_former);
            }
            let pt_tail = &points[points_num - 1];
            if !visited.contains_key(&(pt_tail.x, pt_tail.y)) {
                visited.insert((pt_tail.x, pt_tail.y), true);
            }
        }
    }

    println!("{}", visited.len());
}
