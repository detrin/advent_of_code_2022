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
| data/1.txt | task1_part1_v1 | 107.7 | 14.6 |
| data/1.txt | task1_part1_v2 | 108.1 | 15.6 |
| data/1.txt | task1_part2_v2 | 102.3 | 12.6 |
| data/2.txt | task2_part1_v1 | 106.5 | 12.5 |
| data/2.txt | task2_part1_v2 | 103.0 | 14.3 |
| data/2.txt | task2_part2_v2 | 103.7 | 11.5 |
| data/3.txt | task3_part1_v1 | 105.7 | 14.6 |
| data/3.txt | task3_part2_v1 | 103.0 | 11.9 |
| data/4.txt | task4_part1_v1 | 107.4 | 13.1 |
| data/4.txt | task4_part1_v2 | 105.7 | 14.4 |
| data/4.txt | task4_part2_v1 | 103.3 | 10.8 |
| data/4.txt | task4_part2_v2 | 109.2 | 16.0 |