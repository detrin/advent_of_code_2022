use std::io::{self, BufRead};
use std::cmp::max;
use std::cmp::min;

struct Sensor {
    x: i32,
    y: i32,
    range: i32,
}

impl Sensor {
    fn new(x: i32, y: i32, range: i32) -> Sensor {
        Sensor { x: x, y: y, range: range }
    }

    fn in_reach(&self, x: i32, y: i32) -> bool {
        let dist = (self.x - x).abs() + (self.y - y).abs();
        dist <= self.range
    }
}

struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn new(start: i32, end: i32) -> Interval {
        Interval { start: start, end: end }
    }

    fn subset(&self, other: &Interval) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn print(&self) {
        println!("({}, {})", self.start, self.end);
    }
}

fn merge_intervals(intervals: &mut Vec<Interval>) {
    let mut i = 0;
    while i < intervals.len() - 1 {
        let mut j = i + 1;
        while j < intervals.len() {
            if intervals[i].end >= intervals[j].start {
                intervals[i].end = max(intervals[i].end, intervals[j].end);
                intervals.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }
}

fn parse_coordinate<'a>(line: &'a str, prefix: &'a str, suffix: &'a str) -> (i32, &'a str) {
    let index = line.find(prefix).unwrap() + prefix.len();
    let line_right = &line[index..];
    let index_suffix = line_right.find(suffix).unwrap();
    let value_str = &line_right[..index_suffix];
    let value = value_str.parse().unwrap();
    (value, &line_right[index_suffix..])
}

fn get_next_x_pos(x_centre: i32, x: i32) -> i32 {
    if x_centre > x {
        x + 2 * (x_centre - x) + 1
    } else if x_centre < x {
        x - 2 * (x - x_centre)
    } else {
        x + 1
    }
}

pub fn task15_part1_v1() {
    let stdin = io::stdin();

    let mut sensors: Vec<Sensor> = Vec::new();
    for line in stdin.lock().lines() {
        let mut line = line.unwrap();

        line.push_str("#");
        let (x, line) = parse_coordinate(&line, "Sensor at x=", ", y=");
        let (y, line) = parse_coordinate(&line, "y=", ": closest beacon is at x=");
        let (x_beacon, line) = parse_coordinate(&line, "x=", ", y=");
        let (y_beacon, _) = parse_coordinate(&line, "y=", "#");

        let range = (x - x_beacon).abs() + (y - y_beacon).abs();
        sensors.push(Sensor::new(x, y, range));
    }
    // calculate the average x
    let mut x_sum = 0;
    for sensor in &sensors {
        x_sum += sensor.x;
    }
    let x_avg = x_sum / sensors.len() as i32;
    let y = 2000000; // 20 2000000
    let mut x = x_avg;
    let mut patience = 1000000;
    let mut reach_cnt = 0;
    while patience > 0 {
        patience -= 1;
        let mut in_range = false;
        for sensor in &sensors {
            if sensor.in_reach(x, y) {
                in_range = true;
                reach_cnt += 1;
                break;
            }
        }
        if in_range {
            patience = 1000000;
        }
        x = get_next_x_pos(x_avg, x);
        
    }
    println!("reach_cnt: {}", reach_cnt-1);


}

pub fn task15_part2_v1() {
    let stdin = io::stdin();

    let mut sensors: Vec<Sensor> = Vec::new();
    for line in stdin.lock().lines() {
        let mut line = line.unwrap();

        line.push_str("#");
        let (x, line) = parse_coordinate(&line, "Sensor at x=", ", y=");
        let (y, line) = parse_coordinate(&line, "y=", ": closest beacon is at x=");
        let (x_beacon, line) = parse_coordinate(&line, "x=", ", y=");
        let (y_beacon, _) = parse_coordinate(&line, "y=", "#");

        let range = (x - x_beacon).abs() + (y - y_beacon).abs();
        sensors.push(Sensor::new(x, y, range));
    }
    // calculate the average x
    let mut x_sum = 0;
    for sensor in &sensors {
        x_sum += sensor.x;
    }
    let x_max = 4000000;
    let y_max = 4000000;
    for y in 0..y_max {
        let mut intervals: Vec<Interval> = Vec::new();
        for sensor in &sensors {
            let x_sensor = sensor.x;
            let y_sensor = sensor.y;
            let diff = sensor.range - (y_sensor - y).abs();
            if diff < 0 {
                continue;
            }
            let interval_start = max(0, x_sensor - diff);
            let interval_end = min(x_max, x_sensor + diff) + 1;
            intervals.push(Interval { start: interval_start, end: interval_end });
        }
        intervals.sort_by(|a, b| a.start.cmp(&b.start));
        merge_intervals(&mut intervals);
        // println!("y: {}, len: {}", y, intervals.len());
        let row_interval = Interval::new(0, x_max);
        let is_in = intervals.iter().any(|interval: &Interval| row_interval.subset(&interval));
        // println!("is_in: {}", is_in);
        if !is_in {
            // println!("y: {}", y);
            let mut x = 0;
            for interval in intervals {
                // interval.print();
                if interval.start > x {
                    println!("{}", 4000000*x as i64 + y as i64);
                    break;
                }
                x = interval.end;
            }
        }
    }
}