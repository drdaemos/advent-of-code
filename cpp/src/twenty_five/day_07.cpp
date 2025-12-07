#include "day_07.h"

#include <fmt/base.h>
#include <fmt/ranges.h>

#include <map>
#include <set>
#include <string>
#include <vector>

#include "shared/file_utils.h"
#include "shared/string_utils.h"

namespace day_07 {

void run() {
  auto input = shared::read_file("inputs/day_07.txt");
  fmt::print("{}\n", part1(input));
  fmt::print("{}\n", part2(input));
}

using SplitterColumns = std::vector<int>;

std::vector<SplitterColumns> parse_input(const std::string &input) {
  auto lines = shared::get_lines(input);

  std::vector<SplitterColumns> grid;

  for (const auto &line : lines) {
    SplitterColumns row;
    for (size_t col = 0; col < line.size(); ++col) {
      char ch = line[col];
      if (ch == '^' || ch == 'S') {
        row.push_back(col);
      }
    }

    if (row.size() > 0) {
      grid.push_back(row);
    }
  }

  return grid;
}

int parse_column_count(const std::string &input) {
  auto lines = shared::get_lines(input);
  if (lines.empty()) {
    return 0;
  }
  return static_cast<int>(lines[0].size());
}

void debug_print(const int column_count,         //
                 const std::set<int> &positions, //
                 const SplitterColumns &row) {
  for (size_t i = 0; i < column_count - 1; ++i) {
    if (std::find(row.begin(), row.end(), i) != row.end()) {
      fmt::print("^");
    } else {
      fmt::print(".");
    }
  }
  fmt::print("\n");

  for (size_t i = 0; i < column_count - 1; ++i) {
    if (positions.contains(i)) {
      fmt::print("|");
    } else {
      fmt::print(" ");
    }
  }
  fmt::print("\n");
}

std::string part1(const std::string &input) {
  auto grid = parse_input(input);
  auto column_count = parse_column_count(input);

  int splits = 0;
  std::set<int> positions;
  for (const auto &row : grid) {

    // on first copy the starting positions
    if (positions.empty()) {
      for (const auto pos : row) {
        positions.insert(pos);
      }
    } else {
      // generate beams on both sides of splitters
      std::set<int> new_positions;

      for (const auto splitter : row) {
        if (!positions.contains(splitter)) {
          continue;
        }

        ++splits;
        positions.erase(splitter);
        if (splitter - 1 >= 0) {
          new_positions.insert(splitter - 1);
        }
        if (splitter + 1 <= column_count - 1) {
          new_positions.insert(splitter + 1);
        }
      }

      positions.merge(new_positions);
    }

    // debug print
    debug_print(column_count, positions, row);
  }

  return std::to_string(splits);
}

struct PathCounter {
  const std::vector<SplitterColumns> &rows;
  std::map<std::pair<int, int>, long long> memo;

  long long count_paths(int row, int col) {
    auto key = std::make_pair(row, col);

    if (auto it = memo.find(key); it != memo.end()) {
      return it->second;
    }

    if (row == rows.size() - 1) {
      return memo[key] = 1;
    }

    const auto &next_splitters = rows[row + 1];
    auto iter =
        std::lower_bound(next_splitters.begin(), next_splitters.end(), col);
    bool hits_splitter = (iter != next_splitters.end() && *iter == col);

    long long result;
    if (hits_splitter) {
      result = count_paths(row + 1, col - 1) + //
               count_paths(row + 1, col + 1);
    } else {
      result = count_paths(row + 1, col);
    }
    return memo[key] = result;
  }
};

std::string part2(const std::string &input) {
  auto grid = parse_input(input);
  PathCounter counter{grid};
  return std::to_string(counter.count_paths(0, grid[0][0]));
}

} // namespace day_07
