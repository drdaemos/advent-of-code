# Advent Of Code solutions

This repo contains my solutions to the https://adventofcode.com/ coding challenge.

## Dependencies
- Go v1.19

## Structure
Solutions are grouped by the advent year and separated into `day*` folders which contain several files:

- `input.txt` has my personal input in plain text form
- `solution.go` / `day*.go` has the solution for both parts of the day
- `solution_test.go` contains tests based on the input provided in the challenge description (with known answer)

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