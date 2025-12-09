#include "day_09.h"

#include <complex>
#include <fmt/base.h>
#include <fmt/ranges.h>

#include <algorithm>
#include <string>
#include <unordered_map>
#include <vector>

#include "shared/file_utils.h"
#include "shared/string_utils.h"

using Pos = std::complex<long>;

struct PosHash {
  std::size_t operator()(const Pos &p) const {
    return std::hash<long>()(p.real()) ^ (std::hash<long>()(p.imag()) << 1);
  }
};

namespace day_09 {

void run() {
  auto input = shared::read_file("inputs/day_09.txt");
  fmt::print("{}\n", part1(input));
  fmt::print("{}\n", part2(input));
}

std::vector<Pos> parse_input(const std::string &input) {
  std::vector<Pos> positions;
  auto lines = shared::get_lines(input);
  for (const auto &line : lines) {
    auto parts = shared::split_string(line, ",");
    if (parts.size() == 2) {
      auto x = std::stol(parts[0]);
      auto y = std::stol(parts[1]);
      positions.emplace_back(x, y);
    }
  }
  return positions;
}

long rectangle_area(const Pos &p1, const Pos &p2) {
  auto width = std::abs(p1.real() - p2.real()) + 1;
  auto height = std::abs(p1.imag() - p2.imag()) + 1;
  return width * height;
}

std::string part1(const std::string &input) {
  auto positions = parse_input(input);

  long max_area = 0;
  for (const auto &pos : positions) {
    for (const auto &other : positions) {
      if (pos != other) {
        max_area = std::max(max_area, rectangle_area(pos, other));
      }
    }
  }

  return std::to_string(max_area);
}

bool is_in_polygon(const Pos &point, const std::vector<Pos> &polygon) {
  static std::unordered_map<Pos, bool, PosHash> cache;

  auto it = cache.find(point);
  if (it != cache.end()) {
    return it->second;
  }

  int n = polygon.size();
  int winding_number = 0;

  for (int i = 0; i < n; ++i) {
    Pos p1 = polygon[i];
    Pos p2 = polygon[(i + 1) % n];
    Pos v1 = p1 - point;
    Pos v2 = p2 - point;

    auto cross = v1.real() * v2.imag() - v2.real() * v1.imag();

    // check if point is on edge (cross product = 0 and within bounds)
    if (cross == 0) {
      auto min_x = std::min(p1.real(), p2.real());
      auto max_x = std::max(p1.real(), p2.real());
      auto min_y = std::min(p1.imag(), p2.imag());
      auto max_y = std::max(p1.imag(), p2.imag());
      if (point.real() >= min_x && point.real() <= max_x &&
          point.imag() >= min_y && point.imag() <= max_y) {
        return true;
      }
    }

    // calculate winding number
    if (v1.imag() <= 0) {
      if (v2.imag() > 0 && cross > 0) {
        ++winding_number;
      }
    } else {
      if (v2.imag() <= 0 && cross < 0) {
        --winding_number;
      }
    }
  }

  bool result = winding_number != 0;
  cache[point] = result;
  return result;
}

std::string part2(const std::string &input) {
  auto corners = parse_input(input);

  long max_area = 0;
  int pos_idx = 0;
  for (const auto &pos : corners) {
    fmt::println("Checking pos {} of {}", pos_idx, corners.size());
    pos_idx++;
    for (const auto &other : corners) {
      // skip same point
      if (pos == other) {
        continue;
      }

      if (rectangle_area(pos, other) <= max_area) {
        continue;
      }

      std::vector<Pos> points_to_check;

      // check the two other corners
      Pos c1(other.real(), pos.imag());
      Pos c2(pos.real(), other.imag());
      points_to_check.push_back(c1);
      points_to_check.push_back(c2);

      // check edges
      for (long x = std::min(pos.real(), other.real());
           x <= std::max(pos.real(), other.real()); ++x) {
        points_to_check.emplace_back(x, pos.imag());
        points_to_check.emplace_back(x, other.imag());
      }
      for (long y = std::min(pos.imag(), other.imag());
           y <= std::max(pos.imag(), other.imag()); ++y) {
        points_to_check.emplace_back(pos.real(), y);
        points_to_check.emplace_back(other.real(), y);
      }

      bool all_in_polygon = true;
      for (const auto &p : points_to_check) {
        if (!is_in_polygon(p, corners)) {
          all_in_polygon = false;
          break;
        }
      }

      if (all_in_polygon) {
        max_area = std::max(max_area, rectangle_area(pos, other));
      }
    }
  }

  return std::to_string(max_area);
}

} // namespace day_09
