# Advent of Code 2023

[![About](https://img.shields.io/badge/Advent%20of%20Code-2023-brightgreen?style=flat-square)](https://adventofcode.com/2023/about)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg?style=flat-square)](https://en.wikipedia.org/wiki/Rust_(programming_language))
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](https://mit-license.org/)
![Days completed](https://img.shields.io/badge/Days%20completed-11-red?style=flat-square)
![Stars](https://img.shields.io/badge/Stars-23-yellow?style=flat-square)

This is the third year in a row I (plan to) use rust to solve Advent of Code. But I must warn you: I haven't programmed a lot over the past year, so I am likely not in the best shape (School has been rough - I need a break) - expect the worst.

Just like last year, I am lazy and currently setting this project up in a hurry (In class, actually) just so I have the basic framework once the event starts. So this project is literally a cleaned-up copy of my [solutions for the previous year](https://github.com/LeMoonStar/AoC22), which in term are a cleaned up version of my [Advent of Code 2021 solutions](https://github.com/LeMoonStar/AoC21).  
**The Following therefore still contains screenshots of the 2021 binary. The usage is the same, but the name is `aoc23` instead of `aoc21`**

Later on, maybe even before AoC starts, I'll clean this up more, maybe create a crate out of the framework so I don't have to copy each year over and over again.

## Usage

There are multiple ways to run my solutions, the easiest and most comfortable one is the `auto` command:  
It automatically downloads your input. For this it requires you to provide your Advent of Code session id, which you can find in the websites cookies after logging in.  
Simply provide the session token by setting the `AOC_SESSION` environment variable or using the -s argument:  
`AOC_SESSION=XXXMYSESSION ./aoc23 [DAY] auto` or `./aoc23 [DAY] auto -s XXXMYSESSION`.  
In this example, the environment variable for the AoC session is set using `export AOC_SESSION=XXXMYSESSION`, so I can run the command without specifying the session token again:  
![auto command in action](./images/auto.png)  

If you don't want to automatically download the input, you can also use the `run` command, which uses a locally stored file or the stdin input:  
`./aoc23 [DAY] run -f my_input.txt`:  
![run command in action](./images/run.png)  

If you just want to run the day's example, simply use the `test` command, as this project already includes the examples:
`./aoc23 [DAY] test`:  
![test command in action](./images/test.png)  

## Compiling

This project uses `Cargo`, so compiling is pretty easy:  
`cargo build --release`  
The resulting binary can be found at `./targets/release/aoc22`. You can also directly run the project using `cargo run --release [arguments for aoc23]`  
the `--release` option is not required, but it results in better performance.

## Check out other AoC23 solutions

| Repository                                                                                           | Language                                |
|------------------------------------------------------------------------------------------------------|-----------------------------------------|
| [Trojaner's AdventofCode2023](https://github.com/TrojanerHD/AdventofCode2023)                        | TypeScript (Bun) (+ Rust)               |
| [Niklas' Advent-of-Code-2023](https://github.com/derNiklaas/Advent-of-Code-2023)                     | Kotlin                                  |
| [noeppi_noeppi's aoc](https://github.com/noeppi-noeppi/aoc/tree/master/2023)                         | Multiple                                |
| [Seralius' AOC2023](https://github.com/Seralius/AOC2023)                                             | JavaScript                              |
| [Daan Breur's AdventofCode](https://github.com/daanbreur/AdventofCode)                               | Rust                                    |
| [Hax's advent-of-code](https://github.com/Schlauer-Hax/advent-of-code)                                              | TypeScript                              |
