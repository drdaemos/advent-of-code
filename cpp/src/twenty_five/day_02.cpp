#include "day_02.h"

#include <fmt/core.h>

#include <numeric>
#include <set>
#include <sstream>
#include <string>
#include <vector>

#include "shared/file_utils.h"
#include "shared/math_utils.h"
#include "shared/string_utils.h"

namespace day_02 {

void run() {
  auto input = shared::read_file("inputs/day_02.txt");
  fmt::print("{}\n", part1(input));
  fmt::print("{}\n", part2(input));
}

struct Range {
  long long start;
  long long end;
};

std::vector<Range> parse_ranges(const std::string &input) {
  std::vector<Range> ranges;
  std::istringstream stream(input);
  std::string line;
  while (std::getline(stream, line, ',')) {
    auto dash_pos = line.find('-');
    ranges.push_back({std::stoll(line.substr(0, dash_pos)),
                      std::stoll(line.substr(dash_pos + 1))});
  }
  return ranges;
}

std::string part1(const std::string &input) {
  auto ranges = parse_ranges(input);
  std::vector<long long> invalid = {};
  for (const auto &range : ranges) {
    auto min_pattern_len = shared::digits(range.start) / 2;
    auto max_pattern_len = shared::digits(range.end) / 2;

    auto start = std::pow(10, min_pattern_len - 1);
    auto end = std::pow(10, max_pattern_len);

    for (long long i = start; i < end; ++i) {
      auto pattern = std::stoll(std::to_string(i) + std::to_string(i));

      if (shared::in_range(pattern, range.start, range.end)) {
        invalid.push_back(pattern);
      }
    }
  }

  return std::to_string(std::reduce(invalid.begin(), invalid.end(), 0LL));
}

std::string part2(const std::string &input) {
  auto ranges = parse_ranges(input);
  std::set<long long> invalid = {};
  for (const auto &range : ranges) {
    auto max_pattern_len = shared::digits(range.end) / 2;

    auto start = std::pow(10, 0);
    auto end = std::pow(10, max_pattern_len + 1);

    for (long long i = start; i < end; ++i) {
      for (int size = 2; size <= shared::digits(range.end) / shared::digits(i);
           ++size) {
        auto pattern = shared::repeat(std::to_string(i), size);

        if (shared::in_range(std::stoll(pattern), range.start, range.end)) {
          invalid.insert(std::stoll(pattern));
        }
      }
    }
  }
  return std::to_string(std::reduce(invalid.begin(), invalid.end(), 0LL));
}

} // namespace day_02
