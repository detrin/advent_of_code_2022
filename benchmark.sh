#!/bin/bash
hyperfine_cmd="hyperfine --warmup 10 --runs 100"

$hyperfine_cmd "cat data/1_example.txt | cargo run task1_v1" 
$hyperfine_cmd "cat data/1_example.txt | cargo run task1_v2" 
$hyperfine_cmd "cat data/1.txt | cargo run task1_v1" 
$hyperfine_cmd "cat data/1.txt | cargo run task1_v2" 

