import gleam/io
import gleeunit
import gleeunit/should
import twenty_four/day_02

pub fn main() {
  gleeunit.main()
}

const input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"

// gleeunit test functions end in `_test`
pub fn day_02_test() {
  day_02.part_one(input)
  |> should.equal(2)

  io.debug("PART TWOOOOOOOOOOOOOOOOOOOOo")

  day_02.part_two(input)
  |> should.equal(4)
}
