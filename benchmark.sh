#!/bin/bash
hyperfine_cmd="hyperfine --warmup 10 --runs 100"

$hyperfine_cmd "cat data/1.txt | cargo run task1_part1_v1" 
$hyperfine_cmd "cat data/1.txt | cargo run task1_part1_v2" 
$hyperfine_cmd "cat data/1.txt | cargo run task1_part2_v2" 

$hyperfine_cmd "cat data/2.txt | cargo run task2_part1_v1" 
$hyperfine_cmd "cat data/2.txt | cargo run task2_part1_v2"
$hyperfine_cmd "cat data/2.txt | cargo run task2_part2_v2"

$hyperfine_cmd "cat data/3.txt | cargo run task3_part1_v1" 
$hyperfine_cmd "cat data/3.txt | cargo run task3_part2_v1"

$hyperfine_cmd "cat data/4.txt | cargo run task4_part1_v1"
$hyperfine_cmd "cat data/4.txt | cargo run task4_part1_v2"
$hyperfine_cmd "cat data/4.txt | cargo run task4_part2_v1"
$hyperfine_cmd "cat data/4.txt | cargo run task4_part2_v2"

$hyperfine_cmd "cat data/5.txt | cargo run task5_part1_v1"
$hyperfine_cmd "cat data/5.txt | cargo run task5_part2_v1"

$hyperfine_cmd "cat data/6.txt | cargo run task6_part1_v1"
$hyperfine_cmd "cat data/6.txt | cargo run task6_part2_v1"

$hyperfine_cmd "cat data/7.txt | cargo run task7_part1_v1"
$hyperfine_cmd "cat data/7.txt | cargo run task7_part2_v1"