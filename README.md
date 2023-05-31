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
| data/1.txt | task1_part1_v1 | 98.5 | 32.5 |
| data/1.txt | task1_part1_v2 | 102.4 | 19.4 |
| data/1.txt | task1_part2_v2 | 102.4 | 19.2 |
| data/2.txt | task2_part1_v1 | 101.1 | 18.5 |
| data/2.txt | task2_part1_v2 | 96.2 | 11.6 |
| data/2.txt | task2_part2_v2 | 101.6 | 18.9 |