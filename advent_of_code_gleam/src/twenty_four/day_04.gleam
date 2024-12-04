import gleam/int
import gleam/io
import gleam/list
import gleam/option
import gleam/regexp
import gleam/result
import gleam/string
import simplifile
import utils/matrix

pub fn main() {
  let input =
    simplifile.read(from: "./inputs/day_04.txt")
    |> result.lazy_unwrap(fn() {
      panic as "No input provided, expected to have './inputs/day_04.txt' in the app folder."
    })

  io.println("part one: " <> part_one(input) |> int.to_string())
  io.println("part two: " <> part_two(input) |> int.to_string())
}

pub fn part_one(input: String) -> Int {
  parse_input(input)
  |> io.debug
  |> matrix.rotate45
  |> io.debug

  0
}

pub fn part_two(input: String) -> Int {
  0
}

fn parse_input(input: String) -> List(List(String)) {
  input
  |> string.trim
  |> string.split("\n")
  |> list.map(fn(line) {
    string.trim(line)
    |> string.to_graphemes
  })
  |> list.transpose
}
