# Advent of Code 2021
This repository contains my solutions for the [Advent of Code 2021](https://adventofcode.com/2021) programming puzzles, written in Rust 🦀.

This was my fourth Advent of Code and I plan to do it "live", starting on December 1st and generally trying to complete each puzzle on the day it is released.

Goals:
1. Solve the puzzles in a reasonably robust way.
    * Create solutions that are "general enough", such that any reasonable input would be solved by the program. In some cases it would be prohibitively difficult to make a truly general solution, so if the input data shows clear intent to not require one some shortcuts are acceptable.
    * Create programs with good structure. Use structs, mods, and composition.
    * Follow best practices by writing unit tests wherever it benefits the process of solving the puzzle. Typically, any important functionality with non-trivial edge cases would be tested.
    * Documentation is not a priority. The overall solution nor the individual functions are described with comments, except where it helps the writing process. I'm writing throw-away code for myself - in a real life production environment comments and thorough documentation would of course be required.
2. Write idiomatic Rust code wherever possible.
    * The code compiles with no warnings, has no clippy warnings (see below), and conforms to the Rust formatting guidelines.
    * Use only features of the latest version of stable Rust.
    * No unsafe code.
3. Learn or practice features of the Rust language where the solutions present an opportunity, even if it's not the ideal fit for the situation.
4. Create efficient solutions. Without knowing the puzzles, I would like to have all solutions execute within 1 second total. I usually choose good structure over optimization, but for days when the solution takes a long time to run the optimization may become a priority.
5. Use few external crates, relying mainly on the standard library.

# Building and running
This project uses the [Cargo AoC](https://github.com/gobanos/cargo-aoc) framework, which must be installed in order to build the program. Cargo AoC  makes it easy to download input files and supply them to the program, separate generators and solvers, and execute solutions selectively. It also provides an easy way to benchmark solutions.

All solutions can be tested and run with the usual cargo commands:
* `cargo test`
* `cargo run --release`

The solutions can be selectively run as follows:
* `cargo aoc -d D`, where D is replaced with the relevant day number (1-25)
* `cargo aoc -d D -p P`, same as above but replacing P with the relevant part number (1-2)

## Clippy
The clippy linter does not produce any warnings on the code at the default warning levels, with few exceptions where it is suppressed:
* `clippy::bool_comparison` and `clippy::needless_bool` - I find it far more readable to explicitly write booleans in most places they are used

## Commit hook
Each commit is checked with the following commands:
* `cargo fmt -- --check`
* `cargo test`
* `cargo clippy -- -Dwarnings`

# Execution times
Time measurements were made using the command: `cargo aoc bench -d D`, where D is replaced with the relevant day number (1-25). The average measurement was used; in some cases it would be more accurate to use the fastest measurement as this best represents how the program is capable of performing, however in other cases there is significant variability in the run time due to the program itself (such as when using hashes, which internally have random seeds).

Total: TBD

Day | Part | Time
:--:| :--: | :-------:
1   | 1    | 3.3656 us
1   | 2    | 4.6081 us
2   | 1    | 710.36 ns
2   | 2    | 752.24 ns
3   | 1    | 903.98 ns
3   | 2    | 576.49 ns
4   | 1    | 25.295 us
4   | 2    | 91.882 us
5   | 1    | 5.6363 ms
5   | 2    | 10.752 ms
6   | 1    | 452.02 ns
6   | 2    | 1.4395 us
7   | 1    | 292.23 us
7   | 2    | 593.92 us
8   | 1    | 384.76 ns
8   | 2    | 43.175 us
9   | 1    | 372.50 us
9   | 2    | 1.3180 ms
10  | 1    | 26.156 us
10  | 2    | 28.620 us
11  | 1    | 600.12 us
11  | 2    | 2.1098 ms
12  | 1    | 1.7845 ms
12  | 2    | 53.283 ms
13  | 1    | 90.516 us
13  | 2    | 421.38 us
14  | 1    | 121.56 us
14  | 2    | 558.45 us
15  | 1    | 2.9961 ms
15  | 2    | 185.76 ms
16  | 1    | 12.221 us
16  | 2    | 13.139 us
17  | 1    | 3.3484 ms
17  | 2    | 2.7662 ms
18  | 1    | 1.2581 ms
18  | 2    | 23.345 ms
19  | 1    | 2.5812 s
19  | 2    | 2.5520 s
