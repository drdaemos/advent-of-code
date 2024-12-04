import gleam/io
import gleeunit
import gleeunit/should
import twenty_four/day_03

pub fn main() {
  gleeunit.main()
}

const input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"

const input_2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"

pub fn day_03_part_one_test() {
  day_03.part_one(input)
  |> should.equal(161)
}

pub fn day_03_part_two_test() {
  day_03.part_two(input_2)
  |> should.equal(48)
}
