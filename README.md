# Solitaire Chess Puzzle Generator

Goal: Generate 'hard' puzzles.

## Heuristics of current algorithm

1. About 6-7 pieces on the board.
2. Select pieces to place based on its weight.
    3. Eg: Queen is too powerful, so it has lower weightage.
    4. Eg: Knights are confusing. More knights.

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
