use std::io::{self, BufRead};

fn noop() {
    // empty
}

fn addx(value: i32, registry: &mut i32) {
    *registry += value as i32;
}

fn debug(registry: i32, index: usize) -> i32 {
    let value = match index {
        20 => registry,
        60 => registry,
        100 => registry,
        140 => registry,
        180 => registry,
        220 => registry,
        _ => 0,
    } * index as i32;
    //println!("registry: {}, index {}, value: {}", registry, index, value);
    value
}

pub fn task10_part1_v1() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut registry: i32 = 1;
    let mut index: usize = 0;
    let mut total_sum = 0;

    for line in input {
        if line.contains("noop") {
            noop();
            index += 1;
            total_sum += debug(registry, index);
        } else if line.contains("addx") {
            index += 1;
            total_sum += debug(registry, index);
            index += 1;
            total_sum += debug(registry, index);
            let value = line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            addx(value, &mut registry);
            
        } else {
            panic!("Unknown instruction");
        }
        
    }
    println!("{}", total_sum);
}

fn print_screen(screen: &[[bool; 40]; 6]) {
    for row in screen {
        for pixel in row {
            if *pixel {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn screen_ray(screen: &mut [[bool; 40]; 6], time: usize, registry: i32) {
    let pos = (time-1) % (40 * 6);
    let x: i32 = (pos % 40) as i32;
    let y = pos / 40;
    let diff_x: i32 = x - registry;
    if diff_x.abs() <= 1 {
        screen[y][x as usize] = true;
    }
    
}

pub fn task10_part2_v1() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut registry: i32 = 1;
    let mut index: usize = 0;
    let mut screen = [[false; 40]; 6];

    for line in input {
        if line.contains("noop") {
            index += 1;
            screen_ray(&mut screen, index, registry);
            noop();
        } else if line.contains("addx") {
            index += 1;
            screen_ray(&mut screen, index, registry);
            index += 1;
            screen_ray(&mut screen, index, registry);
            let value = line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            addx(value, &mut registry);
            
        } else {
            panic!("Unknown instruction");
        }
        
        
    }
    print_screen(&screen);
}