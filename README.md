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
| data/1.txt | task1_part1_v1 | 160.8 | 32.0 |
| data/1.txt | task1_part1_v2 | 159.5 | 10.7 |
| data/1.txt | task1_part2_v2 | 159.3 | 17.1 |
| data/2.txt | task2_part1_v1 | 160.3 | 22.9 |
| data/2.txt | task2_part1_v2 | 161.8 | 18.0 |
| data/2.txt | task2_part2_v2 | 158.0 | 12.7 |
| data/3.txt | task3_part1_v1 | 170.1 | 54.7 |
| data/3.txt | task3_part2_v1 | 166.5 | 13.3 |
| data/4.txt | task4_part1_v1 | 169.5 | 21.4 |
| data/4.txt | task4_part1_v2 | 171.5 | 25.1 |
| data/4.txt | task4_part2_v1 | 168.2 | 16.7 |
| data/4.txt | task4_part2_v2 | 169.8 | 19.0 |
| data/5.txt | task5_part1_v1 | 170.8 | 22.1 |
| data/5.txt | task5_part2_v1 | 170.2 | 17.8 |
| data/6.txt | task6_part1_v1 | 174.1 | 24.6 |
| data/6.txt | task6_part2_v1 | 176.6 | 16.8 |