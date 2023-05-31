mod task1;
mod task2;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|x| x.as_str()) {
        Some("task1_part1_v1") => task1::task1_part1_v1(),
        Some("task1_part1_v2") => task1::task1_part1_v2(),
        Some("task1_part2_v1") => task1::task1_part2_v1(),
        Some("task2_part1_v1") => task2::task2_part1_v1(),
        Some("task2_part1_v2") => task2::task2_part1_v2(),
        Some("task2_part2_v1") => task2::task2_part2_v1(),
        _ => println!("Invalid argument"),
    }
}
