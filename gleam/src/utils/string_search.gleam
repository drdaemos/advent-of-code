import gleam/list
import gleam/string

pub fn find_substring_indexes(haystack: String, needle: String) -> List(Int) {
  case string.contains(haystack, needle) {
    False -> []
    True -> {
      // Loop over the haystack up to a point where a full needle can fit.
      let end = { haystack |> string.length } - { needle |> string.length } + 1

      list.range(0, end)
      |> list.fold([], fn(indexes, i) {
        // Extract the substring of length equal to 'needle' starting at index 'i'.
        let substring = string.slice(haystack, i, needle |> string.length)

        // Check if the extracted substring matches 'needle'.
        case substring == needle {
          True -> {
            [i, ..indexes]
          }
          False -> indexes
        }
      })
      // The resulting list will contain indexes in reverse order, so reverse it.
      |> list.reverse
    }
  }
}
