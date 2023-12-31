![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/lukaswilde/advent-of-code-2023/rust-test.yml?branch=main&label=Tests&logo=github&style=for-the-badge)
![GitHub](https://img.shields.io/github/license/lukaswilde/advent-of-code-2023?style=for-the-badge)
![](https://img.shields.io/badge/Made%20with-Rust-orange?style=for-the-badge&logo=rust)

# Advent of Code 2023

Here you can find my solutions to the [Advent of Code](https://adventofcode.com) puzzles from 2023. I do not focus on speed but rather on 
general and readable solutions using Rust 🦀.


## Usage

To run the solutions, make sure you have Rust installed. The solutions are organized as a workspace, so to run the program for e.g. `day 2`, use

```zsh
cargo run --release --bin day2 -- -i day2/puzzle.txt
```

Alternativly, the programs are configured to also accept the problem text directly. For this, use

```zsh
cargo run --release --bin day2 -- "Here is the problem text"
```

You can also build all solutions using 

```zsh
cargo build --release
```

And run the executable like above:

```zsh
./target/release/day2 -i day02/puzzle.txt
```


<!--- advent_readme_stars table --->
## 2023 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2023/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2023/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2023/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2023/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2023/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2023/day/6) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

