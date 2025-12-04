#include "day_04.h"

#include <complex>
#include <fmt/core.h>
#include <fmt/ranges.h>

#include <sstream>
#include <string>
#include <unordered_set>

#include "shared/file_utils.h"

using Pos = std::complex<int>;

// Hash function for using Pos in unordered_set
struct PosHash {
  std::size_t operator()(const Pos &p) const {
    return std::hash<int>()(p.real()) ^ (std::hash<int>()(p.imag()) << 1);
  }
};

namespace day_04 {

void run() {
  auto input = shared::read_file("inputs/day_04.txt");
  fmt::print("{}\n", part1(input));
  fmt::print("{}\n", part2(input));
}

auto parse_input(const std::string &input) {
  std::unordered_set<Pos, PosHash> filled_positions;
  std::istringstream stream(input);
  std::string line;

  int y = 0;
  while (std::getline(stream, line)) {
    for (int x = 0; x < static_cast<int>(line.size()); ++x) {
      if (line[x] == '@') {
        filled_positions.emplace(x, y);
      }
    }
    ++y;
  }
  return filled_positions;
}

constexpr std::array<Pos, 8> DIRECTIONS = { //
    Pos{-1, -1},                            //
    Pos{0, -1},                             //
    Pos{1, -1},                             //
    Pos{-1, 0},                             //
    Pos{1, 0},                              //
    Pos{-1, 1},                             //
    Pos{0, 1},                              //
    Pos{1, 1}};                             //

int filled_adjacent_count(const Pos &p,
                          const std::unordered_set<Pos, PosHash> &filled) {
  return std::count_if(
      DIRECTIONS.begin(), DIRECTIONS.end(),
      [&](const Pos &dir) { return filled.contains(p + dir); });
}

std::string part1(const std::string &input) {
  auto grid = parse_input(input);

  int accessible_count = 0;
  for (const auto &pos : grid) {
    int count = filled_adjacent_count(pos, grid);

    if (count < 4) {
      ++accessible_count;
    }
  }
  return std::to_string(accessible_count);
}

std::string part2(const std::string &input) {
  auto grid = parse_input(input);

  int removed_count = 0;
  bool grid_is_dirty;
  do {
    grid_is_dirty = false;
    for (auto it = grid.begin(); it != grid.end();) {
      int count = filled_adjacent_count(*it, grid);

      if (count < 4) {
        it = grid.erase(it);
        ++removed_count;
        grid_is_dirty = true;
      } else {
        it++;
      }
    }
  } while (grid_is_dirty);

  return std::to_string(removed_count);
}

} // namespace day_04
