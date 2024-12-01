import gleeunit
import gleeunit/should
import twenty_four/day_01

pub fn main() {
  gleeunit.main()
}

const input = "3   4
4   3
2   5
1   3
3   9
3   3"

// gleeunit test functions end in `_test`
pub fn day_01_test() {
  day_01.part_one(input)
  |> should.equal(11)

  day_01.part_two(input)
  |> should.equal(31)
}
