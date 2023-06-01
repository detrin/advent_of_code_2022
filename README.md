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
| data/1.txt | task1_part1_v1 | 167.5 | 20.8 |
| data/1.txt | task1_part1_v2 | 162.9 | 12.7 |
| data/1.txt | task1_part2_v2 | 165.9 | 16.2 |
| data/2.txt | task2_part1_v1 | 163.1 | 11.4 |
| data/2.txt | task2_part1_v2 | 165.0 | 13.7 |
| data/2.txt | task2_part2_v2 | 159.8 | 16.6 |
| data/3.txt | task3_part1_v1 | 160.5 | 19.5 |
| data/3.txt | task3_part2_v1 | 161.4 | 10.8 |
| data/4.txt | task4_part1_v1 | 162.2 | 20.1 |
| data/4.txt | task4_part1_v2 | 159.8 | 14.9 |
| data/4.txt | task4_part2_v1 | 159.4 | 15.4 |
| data/4.txt | task4_part2_v2 | 161.6 | 14.4 |
| data/5.txt | task5_part1_v1 | 164.0 | 18.8 |
| data/5.txt | task5_part2_v1 | 161.8 | 13.3 |
| data/6.txt | task6_part1_v1 | 163.7 | 14.0 |
| data/6.txt | task6_part2_v1 | 177.6 | 23.8 |
| data/7.txt | task7_part1_v1 | 163.5 | 19.4 |
| data/7.txt | task7_part2_v1 | 165.4 | 15.2 |
| data/8.txt | task8_part1_v1 | 174.7 | 18.5 |
| data/8.txt | task8_part2_v1 | 166.8 | 16.6 |