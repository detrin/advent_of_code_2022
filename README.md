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
| data/1.txt | task1_part1_v1 | 157.6 | 19.4 |
| data/1.txt | task1_part1_v2 | 157.9 | 11.9 |
| data/1.txt | task1_part2_v2 | 157.4 | 15.8 |
| data/2.txt | task2_part1_v1 | 158.9 | 13.6 |
| data/2.txt | task2_part1_v2 | 157.4 | 8.3 |
| data/2.txt | task2_part2_v2 | 155.7 | 12.9 |
| data/3.txt | task3_part1_v1 | 158.5 | 16.0 |
| data/3.txt | task3_part2_v1 | 158.5 | 19.3 |
| data/4.txt | task4_part1_v1 | 159.0 | 14.2 |
| data/4.txt | task4_part1_v2 | 160.6 | 23.7 |
| data/4.txt | task4_part2_v1 | 160.2 | 12.3 |
| data/4.txt | task4_part2_v2 | 161.1 | 14.8 |
| data/5.txt | task5_part1_v1 | 163.5 | 14.5 |
| data/5.txt | task5_part2_v1 | 164.3 | 17.6 |
| data/6.txt | task6_part1_v1 | 160.6 | 12.7 |
| data/6.txt | task6_part2_v1 | 185.9 | 95.9 |
| data/7.txt | task7_part1_v1 | 165.6 | 16.0 |
| data/7.txt | task7_part2_v1 | 166.8 | 26.2 |