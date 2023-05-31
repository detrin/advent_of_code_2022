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
| data/1.txt | task1_part1_v1 | 156.4 | 12.7 |
| data/1.txt | task1_part1_v2 | 161.0 | 17.1 |
| data/1.txt | task1_part2_v2 | 158.4 | 15.8 |
| data/2.txt | task2_part1_v1 | 160.2 | 16.3 |
| data/2.txt | task2_part1_v2 | 158.5 | 14.6 |
| data/2.txt | task2_part2_v2 | 159.0 | 14.4 |
| data/3.txt | task3_part1_v1 | 158.4 | 15.7 |
| data/3.txt | task3_part2_v1 | 158.0 | 13.5 |
| data/4.txt | task4_part1_v1 | 160.8 | 21.8 |
| data/4.txt | task4_part1_v2 | 158.9 | 16.5 |
| data/4.txt | task4_part2_v1 | 157.8 | 14.0 |
| data/4.txt | task4_part2_v2 | 157.5 | 11.1 |
| data/5.txt | task5_part1_v1 | 159.3 | 13.2 |
| data/5.txt | task5_part2_v1 | 161.4 | 17.2 |