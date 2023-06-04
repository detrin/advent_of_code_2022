use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x: x, y: y }
    }

    fn new_from_str(s: &str) -> Point {
        let mut iter = s.split(",");
        let x = iter.next().unwrap().parse::<usize>().unwrap();
        let y = iter.next().unwrap().parse::<usize>().unwrap();
        Point { x: x, y: y }
    }

    fn print(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

fn fill_map_with_points(map: &mut HashMap<Point, usize>, pt_start: Point, pt_end: Point, pt_type: usize) {
    let mut x = pt_start.x;
    let mut y = pt_start.y;
    if pt_start.x == pt_end.x {
        while y != pt_end.y {
            map.insert(Point::new(x, y), pt_type);
            if pt_start.y < pt_end.y {
                y += 1;
            } else {
                y -= 1;
            }
        }
        if !map.contains_key(&pt_end) {
            map.insert(pt_end, pt_type);
        }
    } 
    if pt_start.y == pt_end.y {
        while x != pt_end.x {
            map.insert(Point::new(x, y), pt_type);
            if pt_start.x < pt_end.x {
                x += 1;
            } else {
                x -= 1;
            }
        }
        if !map.contains_key(&pt_end) {
            map.insert(pt_end, pt_type);
        }
    }
}

fn apply_gravity_on_pt(map: &mut HashMap<Point, usize>, pt: Point) -> Point {
    let pt_type = map.get(&pt).unwrap().clone();
    let pt_below = Point::new(pt.x, pt.y + 1);
    if !map.contains_key(&pt_below) {
        map.insert(pt_below, pt_type);
        map.remove(&pt);
        return pt_below;
    } else {
        let pt_left = Point::new(pt.x - 1, pt.y + 1);
        let pt_right = Point::new(pt.x + 1, pt.y + 1);
        if !map.contains_key(&pt_left) {
            map.insert(pt_left, pt_type);
            map.remove(&pt);
            return pt_left;
        } 
        if !map.contains_key(&pt_right) {
            map.insert(pt_right, pt_type);
            map.remove(&pt);
            return pt_right;
        }       
        return pt;
    }
}

fn check_if_stable(map: &mut HashMap<Point, usize>, pt: Point) -> bool {
    let pt_below = Point::new(pt.x, pt.y + 1);
    if !map.contains_key(&pt_below) {
        return false;
    } else {
        let pt_left = Point::new(pt.x - 1, pt.y + 1);
        let pt_right = Point::new(pt.x + 1, pt.y + 1);
        if !map.contains_key(&pt_left) {
            return false;
        } 
        if !map.contains_key(&pt_right) {
            return false;
        }       
        return true;
    }
}

fn check_if_falling_forever(map: &mut HashMap<Point, usize>, pt: Point) -> bool {
    for pt_check in map.keys() {
        if pt_check.x == pt.x && pt_check.y > pt.y {
            return false;
        }
    }
    return true;
}

fn print_map(map: &HashMap<Point, usize>) {
    let mut min_x = 10000;
    let mut min_y = 10000;
    let mut max_x = 0;
    let mut max_y = 0;
    for (pt, _) in map {
        if pt.x < min_x {
            min_x = pt.x;
        }
        if pt.y < min_y {
            min_y = pt.y;
        }
        if pt.x > max_x {
            max_x = pt.x;
        }
        if pt.y > max_y {
            max_y = pt.y;
        }
    }
    min_x -= 1;
    max_x += 1;
    max_y += 1;
    for y in min_y..max_y+1 {
        for x in min_x..max_x+1 {
            let pt = Point::new(x, y);
            if map.contains_key(&pt) {
                print!("{}", map.get(&pt).unwrap());
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn add_floor(map: &mut HashMap<Point, usize>) {
    let mut max_y = 0;
    for pt in map.keys() {
        if pt.y > max_y {
            max_y = pt.y;
        }
    }
    max_y += 2;
    map.insert(Point::new(500, max_y), 1);
    for x in 1..max_y+1 {
        map.insert(Point::new(500+x, max_y), 1);
        map.insert(Point::new(500-x, max_y), 1);
    }
}

pub fn task14_part1_v1() {
    let stdin = io::stdin();
    // parse following line into vec of vec of usize
    // 503,4 -> 502,4 -> 502,9 -> 494,9

    let mut map: HashMap<Point, usize> = HashMap::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        // split on " -> "
        let mut iter = line.split(" -> ");
        let mut pt_prev = Point::new_from_str(iter.next().unwrap());
        // pt_prev.print();
        for tuple in iter {
            let pt_current = Point::new_from_str(tuple);
            // pt_current.print();
            fill_map_with_points(&mut map, pt_prev, pt_current, 1);
            pt_prev = pt_current;
        }
        
    }
    // print_map(&map);
    let mut pt = Point::new(500, 0);
    map.insert(pt, 2);
    let mut sand_cnt = 1;
    while !check_if_falling_forever(&mut map, pt) {
        while !check_if_falling_forever(&mut map, pt) && !check_if_stable(&mut map, pt) {
            // println!("pt: {:?}", pt);
            //print_map(&map);
            pt = apply_gravity_on_pt(&mut map, pt);
        }
        if !check_if_falling_forever(&mut map, pt) {
            pt = Point::new(500, 0);
            map.insert(pt, 2);
            sand_cnt += 1;
        }
    }
    // println!("pt: {:?}", pt);
    // print_map(&map);
    println!("{}", sand_cnt-1);
}


pub fn task14_part2_v1() {
    let stdin = io::stdin();
    // parse following line into vec of vec of usize
    // 503,4 -> 502,4 -> 502,9 -> 494,9

    let mut map: HashMap<Point, usize> = HashMap::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        // split on " -> "
        let mut iter = line.split(" -> ");
        let mut pt_prev = Point::new_from_str(iter.next().unwrap());
        // pt_prev.print();
        for tuple in iter {
            let pt_current = Point::new_from_str(tuple);
            // pt_current.print();
            fill_map_with_points(&mut map, pt_prev, pt_current, 1);
            pt_prev = pt_current;
        }
        
    }
    add_floor(&mut map);
    // print_map(&map);
    let mut pt = Point::new(500, 0);
    map.insert(pt, 2);
    let mut sand_cnt = 1;
    while !check_if_stable(&mut map, pt) {
        while !check_if_stable(&mut map, pt) {
            // println!("pt: {:?}", pt);
            // print_map(&map);
            pt = apply_gravity_on_pt(&mut map, pt);
        }
        pt = Point::new(500, 0);
        map.insert(pt, 2);
        sand_cnt += 1;
    }
    // println!("pt: {:?}", pt);
    // print_map(&map);
    println!("{}", sand_cnt);
}