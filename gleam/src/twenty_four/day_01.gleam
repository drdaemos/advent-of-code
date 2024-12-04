import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  let input =
    simplifile.read(from: "./inputs/day_01.txt")
    |> result.lazy_unwrap(fn() {
      panic as "No input provided, expected to have './inputs/day_01.txt' in the app folder."
    })

  io.println("part one: " <> part_one(input) |> int.to_string())
  io.println("part two: " <> part_two(input) |> int.to_string())
}

pub fn part_one(input: String) -> Int {
  parse_input(input)
  |> list.map(fn(numbers) { list.sort(numbers, by: int.compare) })
  |> list.transpose
  |> list.map(fn(pair) {
    list.reduce(pair, int.subtract) |> result.unwrap(0) |> int.absolute_value
  })
  |> list.fold(0, int.add)
}

pub fn part_two(input: String) -> Int {
  let lists = parse_input(input)
  let similarities = list.last(lists) |> result.unwrap([])

  list.first(lists)
  |> result.unwrap([])
  |> list.map(fn(value) {
    list.count(similarities, fn(x) { x == value }) * value
  })
  |> list.fold(0, int.add)
}

fn parse_input(input: String) -> List(List(Int)) {
  input
  |> string.trim
  |> string.split("\n")
  |> list.map(fn(line) {
    string.trim(line)
    |> string.split("   ")
    |> list.map(fn(value) { int.parse(value) |> result.unwrap(0) })
  })
  |> list.transpose
}
