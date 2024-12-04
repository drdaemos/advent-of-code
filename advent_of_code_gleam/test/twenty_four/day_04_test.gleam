import gleam/io
import gleeunit
import gleeunit/should
import twenty_four/day_04

pub fn main() {
  gleeunit.main()
}

const input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"

pub fn day_04_part_one_test() {
  day_04.part_one(input)
  |> should.equal(18)
}

pub fn day_04_part_two_test() {
  day_04.part_two(input)
  |> should.equal(9)
}
