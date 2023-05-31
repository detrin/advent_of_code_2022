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
| data/1.txt | task1_part1_v1 | 95.5 | 11.8 |
| data/1.txt | task1_part1_v2 | 97.8 | 16.1 |
| data/1.txt | task1_part2_v2 | 99.5 | 23.5 |
| data/2.txt | task2_part1_v1 | 99.2 | 16.2 |
| data/2.txt | task2_part1_v2 | 98.6 | 13.7 |
| data/2.txt | task2_part2_v2 | 98.9 | 13.8 |
| data/3.txt | task3_part1_v1 | 99.5 | 24.5 |
| data/3.txt | task3_part2_v1 | 92.7 | 11.6 |