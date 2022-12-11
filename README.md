[![Update README ⭐](https://github.com/jrodal98/advent-of-code-2022/actions/workflows/readme-stars.yml/badge.svg)](https://github.com/jrodal98/advent-of-code-2022/actions/workflows/readme-stars.yml)

# Advent of code 2022

<!--- advent_readme_stars table --->
## 2022 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2022/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2022/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2022/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2022/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2022/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2022/day/6) | ⭐ | ⭐ |
| [Day 7](https://adventofcode.com/2022/day/7) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2022/day/8) | ⭐ | ⭐ |
| [Day 9](https://adventofcode.com/2022/day/9) | ⭐ | ⭐ |
| [Day 10](https://adventofcode.com/2022/day/10) | ⭐ | ⭐ |
| [Day 11](https://adventofcode.com/2022/day/11) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

## Directory generation script

`generate_dir.sh` is a Bash script for creating a directory for solutions to Advent of Code and initializing a Rust project in it. It creates a src/main.rs file based on `template.rs` and injects the user provided sample solution to part 1 of the challenge into a unit test. It also prompts the user to enter the sample input file. If the user has set the AOC_SESSION environment variable, it will download the problem input, otherwise it will prompt the user to provide the problem input as well. Finally, the script opens the rust file, the sample input file, and the problem input file after they have been created.

To get your session token:

1. Go to advent of code website
2. Open developer console
3. Navigate to "application tab"
4. Copy "session" cookie
