import gleam/int
import gleam/io
import gleam/list
import gleam/option
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  let input =
    simplifile.read(from: "./inputs/day_02.txt")
    |> result.lazy_unwrap(fn() {
      panic as "No input provided, expected to have './inputs/day_02.txt' in the app folder."
    })

  io.println("part one: " <> part_one(input) |> int.to_string())
  io.println("part two: " <> part_two(input) |> int.to_string())
}

pub fn part_one(input: String) -> Int {
  parse_input(input)
  |> list.count(fn(report) { is_safe(report, False) })
}

pub fn part_two(input: String) -> Int {
  parse_input(input)
  |> list.count(fn(report) { is_safe(report, True) })
}

// Gives a list of reports (a report is a list of numbers)
fn parse_input(input: String) -> List(List(Int)) {
  input
  |> string.trim
  |> string.split("\n")
  |> list.map(fn(line) {
    string.trim(line)
    |> string.split(" ")
    |> list.map(fn(value) { int.parse(value) |> result.unwrap(0) })
  })
}

type ReportCheck {
  SafeReport(is_increasing: Bool, previous: Int, length: Int)
  UnknownReport(previous: option.Option(Int))
  UnsafeReport(length: Int)
}

// Checks if given report is safe (gradually increasing / decreasing)
fn is_safe(report: List(Int), with_dampening: Bool) -> Bool {
  let state =
    report
    |> list.fold_until(UnknownReport(option.None), fn(acc, value) {
      let is_previous_none = case acc {
        UnknownReport(previous) -> option.is_none(previous)
        SafeReport(..) -> False
        UnsafeReport(..) -> False
      }
      let previous = case acc {
        UnknownReport(previous) -> option.unwrap(previous, 0)
        SafeReport(_, previous, _) -> previous
        UnsafeReport(..) -> 0
      }

      case acc {
        // Initial cases
        UnknownReport(_) if is_previous_none ->
          list.Continue(UnknownReport(option.Some(value)))
        UnknownReport(_)
          if !is_previous_none && value > previous && value - previous <= 3
        -> list.Continue(SafeReport(True, value, 2))
        UnknownReport(_)
          if !is_previous_none && value < previous && previous - value <= 3
        -> list.Continue(SafeReport(False, value, 2))
        // Good cases
        SafeReport(is_increasing, previous, counter)
          if is_increasing && value > previous && value - previous <= 3
        -> list.Continue(SafeReport(is_increasing, value, counter + 1))
        SafeReport(is_increasing, previous, counter)
          if !is_increasing && value < previous && previous - value <= 3
        -> list.Continue(SafeReport(is_increasing, value, counter + 1))
        // Bad cases
        SafeReport(_, _, counter) -> list.Stop(UnsafeReport(counter + 1))
        UnknownReport(..) -> list.Stop(UnsafeReport(2))
        UnsafeReport(..) -> panic as "Unclear case"
      }
    })

  case state {
    SafeReport(..) -> True
    UnknownReport(..) -> False
    UnsafeReport(counter) if with_dampening ->
      retry_various_removals(report, counter, 0)
    UnsafeReport(..) -> False
  }
}

fn retry_various_removals(
  report: List(Int),
  detected_index: Int,
  tries: Int,
) -> Bool {
  case tries > 3 {
    True -> False
    False ->
      fn() {
        let attempt = is_safe(exclude_nth(report, detected_index), False)
        case attempt {
          True -> True
          False -> retry_various_removals(report, detected_index - 1, tries + 1)
        }
      }()
  }
}

// Removes nth (0-based) element from the list
fn exclude_nth(list: List(Int), index_n: Int) -> List(Int) {
  list
  |> list.take(index_n)
  |> list.append(list.drop(list, index_n + 1))
}
