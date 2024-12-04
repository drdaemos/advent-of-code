import gleeunit
import gleeunit/should
import utils/string_search

pub fn main() {
  gleeunit.main()
}

pub fn find_substring_indexes_test() {
  string_search.find_substring_indexes(
    "a quick brown fox jumps over the brown dog",
    "brown",
  )
  |> should.equal([8, 33])
}
