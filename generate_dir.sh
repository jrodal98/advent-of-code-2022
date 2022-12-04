#!/bin/bash

rs_template_file=$(realpath template.rs)
solutions='solutions'
day_dir_base='day'

mkdir -p "$solutions"

# compute number of "day" directories that already exist
num_directories_created=$(fd $day_dir_base $solutions | wc -w)
# compute the number for the next day director
day_num=$((num_directories_created + 1))
# we are going to pad day_num with 0s
zeros="00"
day_num_zero_padded="${zeros:${#day_num}:${#zeros}}${day_num}"
# construct directory name
directory_name="${solutions}/${day_dir_base}$day_num_zero_padded"

temp_sample_input_file=$(mktemp)
echo "Paste sample input here (and delete this line!)" >"$temp_sample_input_file"
${VISUAL:-${EDITOR:-vi}} "$temp_sample_input_file"
read -rp 'Sample solution: ' sample_solution

mkdir "$directory_name"
cd "$directory_name" || exit
cargo init
mkdir data

input_file="data/input.txt"
if [ -z "$AOC_SESSION" ]; then
  echo "Paste problem input here (and delete this line!)" >"$input_file"
  ${VISUAL:-${EDITOR:-vi}} "$input_file"
else
  curl --cookie "session=${AOC_SESSION}" "https://adventofcode.com/2022/day/$day_num/input" >"$input_file"
fi

mv "$temp_sample_input_file" data/sample.txt
sed "s/PART1_SAMPLE_SOLUTION/$sample_solution/" "$rs_template_file" >src/main.rs

${VISUAL:-${EDITOR:-vi}} src/main.rs
