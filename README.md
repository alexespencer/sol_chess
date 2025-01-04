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
Generated a puzzle with 6 pieces after 330 ms
PP..
..PB
.K..
.N..
```

- Solve a puzzle

```bash
$ sol_chess -- --solve PP....PB.K...N..
PP..
..PB
.K..
.N..

Found 1 solutions
1. Nb1 -> c3
2. Nc3 -> a4
3. Na4 -> b2
4. Nb2 -> d3
5. Nd3 -> b4
```

- Generate and solve a puzzle

```bash
$ sol_chess -g -n 6 --print
Generated a puzzle with 6 pieces after 933 ms
.P.N
B.R.
.K..
..N.

Found 1 solutions
1. Ba3 -> b4
2. Bb4 -> c3
3. Bc3 -> d4
4. Bd4 -> b2
5. Bb2 -> c1
```

## Heuristics of current algorithm

1. About 6-7 pieces on the board.
2. Select pieces to place based on its weight.
    3. Eg: Queen is too powerful, so it has lower weightage.
    4. Eg: Knights are confusing. More knights.
3. Discard puzzles with more than one solution.

## Example puzzles generated

1.

```
N...
P.B.
.R..
..KP
```

2.

```
R...
..P.
..B.
.KNN
```
3.

```
.PN.
P...
K..P
.R..
```

4.

```
..PK
...R
P..P
B..N
