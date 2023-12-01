# Advent Of Code solutions

This repo contains my solutions to the https://adventofcode.com/ coding challenge. As a self-imposed rule, I try not to rewrite any logic after receiving the answer - only small code-style refactors are done (to maintain the repo in a good shape).

## Dependencies
- Go v1.19
- PHP v7.4
- Clojure v1.11.1
- Elixir v1.14
- Rust v1.68

## Structure
Solutions are grouped by the advent year and separated into `day*` folders which might contain several files:

- `solution.*` has the solution for both parts of the day and example puzzle input provided in the challenge description
- `input_test.txt` is present when the example puzzle input is too long
- `solution_test.*` contains tests based on the input provided in the challenge description (with known answer)

Additionally, if `input.txt` is present in the day folder, it will be used instead of example input to calculate the answer.

## Usage
To run the solution and see the result based on the given input, run the following:

```shell
cd <year>                       # e.g. cd 2022/
go run main.go <day_folder>     # e.g. go run main.go day01
```

To execute tests for all solutions, run this:
```shell
go test ./...
```