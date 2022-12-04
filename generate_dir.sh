#!/bin/bash

rs_template_file=$(realpath template.rs)
solutions='solutions'
day_dir_base='day'

# compute number of "day" directories that already exist
num_directories_created=$(fd $day_dir_base $solutions | wc -w)
# compute the number for the next day director
day_num=$((num_directories_created + 1))
# we are going to pad day_num with 0s
zeros="00"
day_num_zero_padded="${zeros:${#day_num}:${#zeros}}${day_num}"
# construct directory name
directory_name="${solutions}/${day_dir_base}$day_num_zero_padded"

mkdir "$directory_name"
cd "$directory_name" || exit
cargo init
mkdir data
touch data/input.txt
touch data/sample.txt
cp "$rs_template_file" src/main.rs
