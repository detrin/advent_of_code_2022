mod task1;
mod task10;
mod task11;
mod task12;
mod task2;
mod task3;
mod task4;
mod task5;
mod task6;
mod task7;
mod task8;
mod task9;
mod task13;
mod task14;
mod task15;
mod task16;
mod task17;
mod task18;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|x| x.as_str()) {
        Some("task1_part1_v1") => task1::task1_part1_v1(),
        Some("task1_part1_v2") => task1::task1_part1_v2(),
        Some("task1_part2_v1") => task1::task1_part2_v1(),

        Some("task2_part1_v1") => task2::task2_part1_v1(),
        Some("task2_part1_v2") => task2::task2_part1_v2(),
        Some("task2_part2_v1") => task2::task2_part2_v1(),

        Some("task3_part1_v1") => task3::task3_part1_v1(),
        Some("task3_part2_v1") => task3::task3_part2_v1(),

        Some("task4_part1_v1") => task4::task4_part1_v1(),
        Some("task4_part1_v2") => task4::task4_part1_v2(),
        Some("task4_part2_v1") => task4::task4_part2_v1(),
        Some("task4_part2_v2") => task4::task4_part2_v2(),

        Some("task5_part1_v1") => task5::task5_part1_v1(),
        Some("task5_part2_v1") => task5::task5_part2_v1(),

        Some("task6_part1_v1") => task6::task6_part1_v1(),
        Some("task6_part2_v1") => task6::task6_part2_v1(),

        Some("task7_part1_v1") => task7::task7_part1_v1(),
        Some("task7_part2_v1") => task7::task7_part2_v1(),

        Some("task8_part1_v1") => task8::task8_part1_v1(),
        Some("task8_part2_v1") => task8::task8_part2_v1(),

        Some("task9_part1_v1") => task9::task9_part1_v1(),
        Some("task9_part1_v2") => task9::task9_part1_v2(),
        Some("task9_part2_v1") => task9::task9_part2_v1(),

        Some("task10_part1_v1") => task10::task10_part1_v1(),
        Some("task10_part2_v1") => task10::task10_part2_v1(),

        Some("task11_part1_v1") => task11::task11_part1_v1(),
        Some("task11_part2_v1") => task11::task11_part2_v1(),

        Some("task12_part1_v1") => task12::task12_part1_v1(),
        Some("task12_part2_v1") => task12::task12_part2_v1(),

        Some("task13_part1_v1") => task13::task13_part1_v1(),
        Some("task13_part2_v1") => task13::task13_part2_v1(),

        Some("task14_part1_v1") => task14::task14_part1_v1(),
        Some("task14_part2_v1") => task14::task14_part2_v1(),

        Some("task15_part1_v1") => task15::task15_part1_v1(),
        Some("task15_part2_v1") => task15::task15_part2_v1(),

        Some("task16_part1_v1") => task16::task16_part1_v1(),
        Some("task16_part2_v1") => task16::task16_part2_v1(),

        Some("task17_part1_v1") => task17::task17_part1_v1(),
        Some("task17_part2_v1") => task17::task17_part2_v1(),

        Some("task18_part1_v1") => task18::task18_part1_v1(),

        _ => println!("Invalid argument"),
    }
}
