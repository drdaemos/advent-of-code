import gleam/dict
import gleam/int
import gleam/io
import gleam/list
import gleam/option
import gleam/regexp
import gleam/result
import gleam/string
import simplifile
import utils/matrix
import utils/string_search

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
  |> get_rotated_versions
  |> list.map(find_substrings(_, "XMAS"))
  |> list.flatten
  |> list.length
}

pub fn part_two(input: String) -> Int {
  parse_input(input)
  |> find_crosses
}

fn parse_input(input: String) -> List(List(String)) {
  input
  |> string.trim
  |> string.split("\n")
  |> list.map(fn(line) {
    string.trim(line)
    |> string.to_graphemes
  })
}

// Builds all relevant rotations of the input matrix
fn get_rotated_versions(input: List(List(String))) -> List(List(List(String))) {
  let ninety_clockwise = matrix.rotate90(input)
  let one_eighty_clockwise = ninety_clockwise |> matrix.rotate90
  let two_seventy_clockwise = one_eighty_clockwise |> matrix.rotate90
  [
    input,
    matrix.rotate45(input),
    ninety_clockwise,
    ninety_clockwise |> matrix.rotate45,
    one_eighty_clockwise,
    one_eighty_clockwise |> matrix.rotate45,
    two_seventy_clockwise,
    two_seventy_clockwise |> matrix.rotate45,
  ]
}

// Searches for a substring in a 2d matrix of chars, returns a list of all match positions (row, col)
fn find_substrings(
  input: List(List(String)),
  substring: String,
) -> List(#(Int, Int)) {
  let matches =
    input
    |> list.map(fn(line) { string.join(line, "") })
    |> list.map(fn(line) {
      string_search.find_substring_indexes(line, substring)
    })

  list.map2(matches, list.range(0, list.length(matches)), fn(line, row) {
    list.map(line, fn(col) { #(row, col) })
  })
  |> list.flatten
}

const cross_patterns = ["MSSM", "MMSS", "SSMM", "SMMS"]

// Count X-MAS crosses
fn find_crosses(input: List(List(String))) -> Int {
  // Find all letter "A"s
  input
  |> find_substrings("A")
  // And check four diagonal letters
  |> list.fold(0, fn(count, center) {
    let letters = [
      // left-top
      matrix.at(input, center.0 - 1, center.1 - 1),
      // right-top
      matrix.at(input, center.0 - 1, center.1 + 1),
      // right-bottom
      matrix.at(input, center.0 + 1, center.1 + 1),
      // left-bottom
      matrix.at(input, center.0 + 1, center.1 - 1),
    ]

    // If not on edge
    case list.all(letters, result.is_ok) {
      False -> count
      True -> {
        // Check if letters are in the right "MAS" X "MAS" pattern
        let is_cross =
          list.map(letters, fn(item) {
            result.lazy_unwrap(item, fn() { panic })
          })
          |> string.join("")
          |> list.contains(cross_patterns, _)

        case is_cross {
          False -> count
          True -> count + 1
        }
      }
    }
  })
}
