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

pub fn rotate90_test() {
  let expected = [[7, 4, 1], [8, 5, 2], [9, 6, 3]]
  matrix.rotate90(matrix)
  |> should.equal(expected)

  let expected_2 = [[9, 8, 7], [6, 5, 4], [3, 2, 1]]
  matrix.rotate90(matrix)
  |> matrix.rotate90
  |> should.equal(expected_2)
}
