#!/bin/bash

dir='day'
directory_name="${dir}$(( `ls| grep ${dir} | wc -w` + 1 ))"

mkdir "$directory_name"
cd "$directory_name"
cargo init
mkdir data
touch data/input.txt
touch data/sample.txt
cp ../template.rs src/main.rs
