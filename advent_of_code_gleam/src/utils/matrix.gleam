import gleam/dict.{type Dict}
import gleam/list
import gleam/option
import gleam/result

pub fn rotate90(matrix: List(List(value))) -> List(List(value)) {
  let rows = list.length(matrix)
  let cols = case list.first(matrix) {
    Ok(row) -> list.length(row)
    Error(Nil) -> 0
  }

  list.range(0, cols - 1)
  |> list.map(fn(col) {
    list.range(rows - 1, 0)
    |> list.filter_map(fn(row) {
      list_at(matrix, row)
      |> result.lazy_unwrap(fn() { panic })
      |> list_at(col)
    })
  })
}

pub fn rotate45(matrix: List(List(value))) -> List(List(value)) {
  let rows = list.length(matrix)
  let cols = case list.first(matrix) {
    Ok(row) -> list.length(row)
    Error(Nil) -> 0
  }

  list.range(0, rows + cols - 2)
  |> list.map(fn(diag) {
    list.range(0, diag + 1)
    |> list.filter_map(fn(row) {
      let col = diag - row
      case row < rows && col < cols && col >= 0 {
        True ->
          list_at(matrix, row)
          |> result.lazy_unwrap(fn() { panic })
          |> list_at(col)
        False -> Error(Nil)
      }
    })
  })
}

pub fn at(matrix: List(List(value)), row: Int, col: Int) -> Result(value, Nil) {
  let line = list_at(matrix, row)
  case line {
    Ok(line) -> list_at(line, col)
    Error(Nil) -> Error(Nil)
  }
}

// Returns nth (0-indexed) element if present, or Nil otherwise
fn list_at(list: List(value), index: Int) -> Result(value, Nil) {
  case index < list.length(list) {
    True -> {
      list.take(list, index + 1)
      |> list.last
    }
    False -> Error(Nil)
  }
}
