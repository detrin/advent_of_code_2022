# advent_of_code_2022
Advent of Code 2022 purely in rust


## Installation

```shell
brew install hyperfine
```

## Usage

```shell
cargo build
```

```shell
cat data/2_sample.txt | cargo run task2_v1 
```

## Benchmark Results

| Filename | Function | Mean Time [ms] | Uncertainty [ms] |
|----------|----------|-----------|-------------|
| data/1.txt | task1_part1_v1 | 155.3 | 13.4 |
| data/1.txt | task1_part1_v2 | 157.1 | 15.0 |
| data/1.txt | task1_part2_v2 | 153.6 | 12.7 |
| data/2.txt | task2_part1_v1 | 155.1 | 15.1 |
| data/2.txt | task2_part1_v2 | 156.2 | 15.4 |
| data/2.txt | task2_part2_v2 | 153.4 | 12.3 |
| data/3.txt | task3_part1_v1 | 154.7 | 13.6 |
| data/3.txt | task3_part2_v1 | 155.3 | 13.0 |
| data/4.txt | task4_part1_v1 | 162.4 | 29.5 |
| data/4.txt | task4_part1_v2 | 157.3 | 20.2 |
| data/4.txt | task4_part2_v1 | 160.4 | 31.6 |
| data/4.txt | task4_part2_v2 | 169.7 | 60.4 |
| data/5.txt | task5_part1_v1 | 157.0 | 12.0 |
| data/5.txt | task5_part2_v1 | 157.5 | 16.8 |
| data/6.txt | task6_part1_v1 | 166.4 | 45.0 |
| data/6.txt | task6_part2_v1 | 168.7 | 13.8 |
| data/7.txt | task7_part1_v1 | 158.8 | 20.4 |
| data/7.txt | task7_part2_v1 | 160.4 | 14.7 |
| data/8.txt | task8_part1_v1 | 167.7 | 17.8 |
| data/8.txt | task8_part2_v1 | 164.8 | 41.3 |
| data/9.txt | task9_part1_v1 | 162.0 | 12.1 |
| data/9.txt | task9_part1_v2 | 159.4 | 10.7 |
| data/9.txt | task9_part2_v1 | 164.5 | 12.6 |
| data/10.txt | task10_part1_v1 | 152.0 | 11.1 |
| data/10.txt | task10_part2_v1 | 154.0 | 11.7 |
| data/11.txt | task11_part1_v1 | 160.5 | 16.1 |
| data/11.txt | task11_part2_v1 | 270.4 | 13.9 |
| data/12.txt | task12_part1_v1 | 182.6 | 20.1 |
| data/12.txt | task12_part2_v1 | 186.0 | 19.8 |
| data/13.txt | task13_part1_v1 | 224.0 | 52.0 |
| data/13.txt | task13_part2_v1 | 209.3 | 38.9 |
| data/14.txt | task14_part1_v1 | 423.6 | 60.3 |
| data/14.txt | task14_part2_v1 | 5.545s | 0.070s |
| data/15.txt | task15_part1_v1 | 1.829s | 0.029s|
| data/15.txt | task15_part2_v1 | 8.820s | 0.082s|
| data/16.txt | task16_part1_v1 | 1.330s | 0.042s |
| data/16.txt | task16_part2_v1 | 8.606s | 0.149s |