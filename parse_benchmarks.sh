#!/bin/bash

# Extract benchmark lines using grep
benchmark_lines=$(grep -E "Benchmark [0-9]+:|Time \(mean ± σ\)" )

# Initialize variables
function_name=""
command=""
mean_time=""
uncertainty=""

echo "## Benchmark Results"
echo ""
echo "| Filename | Function | Mean Time [ms] | Uncertainty [ms] |"
echo "|----------|----------|-----------|-------------|"

while read -r line; do
  if [[ $line =~ ^Benchmark ]]; then
    # Extract function name and command
    file_name=$(echo "$line" | awk -F ": " '{print $2}' | awk '{print $2}' | tr -d '[:space:]')
    function_name=$(echo "$line" | awk -F ": " '{print $2}' | awk '{print $6}' | tr -d '[:space:]')
  elif [[ $line =~ "Time (mean ± σ)" ]]; then
    # Extract mean time and uncertainty using awk
    mean_time=$(echo "$line" | awk -F ": " '{print $2}' | awk -F " ms" '{print $1}' | tr -d '[:space:]')
    uncertainty=$(echo "$line" | awk -F "± " '{print $3}' | awk -F " ms" '{print $1}' | tr -d '[:space:]')

    echo "| $file_name | $function_name | $mean_time | $uncertainty |" 

  fi
done <<< "$benchmark_lines"
