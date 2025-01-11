# Solitaire Chess Puzzle Generator

Goal: Generate 'hard' puzzles.

## Install

- Install Rust from [here](https://www.rust-lang.org/tools/install).
- Run `cargo install --git https://github.com/cool-mist/sol_chess` to install the tool.
- Run `sol_chess --help` to see the options.

## Usage

- Generate a puzzle

```bash
$ sol_chess -g -n 6
Generating a puzzle with 6 pieces with a maximum of 5 solutions
                Total attempts:     7
           Total pieces placed:    71
         Success pieces placed:    42
               Total time (ms):    69

               ♘  .  .  .

               ♙  .  ♖  .

               ♔  .  ♘  ♙

               .  .  .  .
```

- Solve a puzzle

```bash
$ sol_chess -- --solve N...P.R.K.NP....
               ♘  .  .  .

               ♙  .  ♖  .

               ♔  .  ♘  ♙

               .  .  .  .


Found 3 solutions
1. Rc3 -> a3
2. Ra3 -> a4
3. Ra4 -> a2
4. Ra2 -> c2
5. Rc2 -> d2
```

- Generate and solve a puzzle

```bash
$ sol_chess -g -n 6 --print
Generating a puzzle with 6 pieces with a maximum of 5 solutions
                Total attempts:     4
           Total pieces placed:    34
         Success pieces placed:    24
               Total time (ms):    38

               .  .  ♙  .

               ♕  .  .  ♘

               .  .  .  .

               ♗  ♖  .  ♘


Found 5 solutions
1. Rb1 -> a1
2. Ra1 -> d1
3. Rd1 -> d3
4. Qa3 -> d3
5. Qd3 -> c4
```

## Heuristics of current algorithm

1. About 6-7 pieces on the board.
2. Select pieces to place based on its weight.
    3. Eg: Queen is too powerful, so it has lower weightage.
    4. Eg: Knights are confusing. More knights.

