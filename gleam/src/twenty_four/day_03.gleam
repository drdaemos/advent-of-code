import gleam/int
import gleam/io
import gleam/list
import gleam/option
import gleam/regexp
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  let input =
    simplifile.read(from: "./inputs/day_03.txt")
    |> result.lazy_unwrap(fn() {
      panic as "No input provided, expected to have './inputs/day_03.txt' in the app folder."
    })

  io.println("part one: " <> part_one(input) |> int.to_string())
  io.println("part two: " <> part_two(input) |> int.to_string())
}

pub fn part_one(input: String) -> Int {
  parse_mul_instructions(input)
  |> list.map(fn(pair) { pair.0 * pair.1 })
  |> list.fold(0, int.add)
}

pub fn part_two(input: String) -> Int {
  parse_enabled_sections(input)
  |> part_one
}

// Removes all sections enclosed in `don't()` - `do()` pairs, leaves only "enabled" substrings
fn parse_enabled_sections(input: String) -> String {
  let assert Ok(regex) =
    regexp.from_string("(?:don't\\(\\)[\\s\\S]*?(?:do\\(\\)|$))")

  input
  |> string.trim
  |> regexp.replace(regex, _, "")
}

// Finds all `mul(\d+,\d+)` substrings and returns them as tuples
fn parse_mul_instructions(input: String) -> List(#(Int, Int)) {
  let assert Ok(regex) = regexp.from_string("(?:mul\\((\\d+),(\\d+)\\))")

  input
  |> string.trim
  |> regexp.scan(regex, _)
  |> list.map(fn(match) {
    #(
      match.submatches |> list.first |> parse_submatch_int,
      match.submatches |> list.last |> parse_submatch_int,
    )
  })
}

fn parse_submatch_int(input: Result(option.Option(String), Nil)) -> Int {
  input
  |> result.unwrap(option.Some("0"))
  |> option.unwrap("0")
  |> int.parse
  |> result.unwrap(0)
}
