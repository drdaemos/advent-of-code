import gleam/dict.{type Dict}
import gleam/list
import gleam/result

pub fn rotate45(matrix: List(List(value))) -> List(List(value)) {
  let num_rows = list.length(matrix)
  let num_cols = case list.first(matrix) {
    Ok(row) -> list.length(row)
    Error(Nil) -> 0
  }
  let num_diagonals = num_rows + num_cols - 1

  let diagonals = collect_diagonals(matrix, dict.new())
  let keys = list.range(0, num_diagonals - 1)
  keys
  |> list.map(fn(key) { dict.get(diagonals, key) |> result.unwrap([]) })
}

fn collect_diagonals(
  matrix: List(List(value)),
  diagonals: Dict(Int, List(value)),
) -> Dict(Int, List(value)) {
  collect_rows(matrix, 0, diagonals)
}

fn collect_rows(
  matrix: List(List(value)),
  i: Int,
  diagonals: Dict(Int, List(value)),
) -> Dict(Int, List(value)) {
  case matrix {
    [] -> diagonals
    [row, ..rest] -> {
      let updated_diagonals = collect_row(row, i, 0, diagonals)
      collect_rows(rest, i + 1, updated_diagonals)
    }
  }
}

fn collect_row(
  row: List(value),
  i: Int,
  j: Int,
  diagonals: Dict(Int, List(value)),
) -> Dict(Int, List(value)) {
  case row {
    [] -> diagonals
    [element, ..rest] -> {
      let idx = i + j
      let current_list = dict.get(diagonals, idx) |> result.unwrap([])
      let new_list = list.append(current_list, [element])
      let new_diagonals = dict.insert(diagonals, idx, new_list)
      collect_row(rest, i, j + 1, new_diagonals)
    }
  }
}
