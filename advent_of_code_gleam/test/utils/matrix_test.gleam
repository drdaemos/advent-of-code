import gleeunit
import gleeunit/should
import utils/matrix

pub fn main() {
  gleeunit.main()
}

const matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]

pub fn rotate45_test() {
  let expected = [[1], [2, 4], [3, 5, 7], [6, 8], [9]]
  matrix.rotate45(matrix)
  |> should.equal(expected)
}
