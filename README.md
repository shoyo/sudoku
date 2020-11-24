# Simple Sudoku Solver
[![github actions](https://github.com/shoyo/sudoku/workflows/build/badge.svg)](https://github.com/shoyo/sudoku/actions?query=workflow%3Abuild)

## About
A sudoku solver using a simple backtracking algorithm.
Run `main()` with `cargo run`, and run tests with `cargo test`.

## Example
```
% cargo run
SOLVING:
 -  1  -  3  -  -  -  -  -
 5  -  -  -  6  -  -  -  -
 -  4  -  -  -  -  -  2  8
 -  -  6  7  -  -  -  -  3
 -  -  2  -  -  -  9  -  -
 7  -  -  -  -  8  4  -  -
 3  9  -  -  -  -  -  6  -
 -  -  -  -  4  -  -  -  9
 -  -  -  -  -  1  -  5  -

SOLVED:
 6  1  8  3  9  2  5  4  7
 5  2  7  8  6  4  3  9  1
 9  4  3  1  7  5  6  2  8
 4  5  6  7  1  9  2  8  3
 1  8  2  4  5  3  9  7  6
 7  3  9  6  2  8  4  1  5
 3  9  5  2  8  7  1  6  4
 2  7  1  5  4  6  8  3  9
 8  6  4  9  3  1  7  5  2
```

## Author
Shoyo Inokuchi (contact@shoyo.dev)
