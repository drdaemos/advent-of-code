#include "day_05.h"

#include <fmt/core.h>
#include <fmt/ranges.h>

#include <sstream>
#include <string>

#include "shared/file_utils.h"
#include "shared/math_utils.h"

using Range = std::pair<long long, long long>;

struct Ingredients {
  std::vector<Range> ranges;
  std::vector<long long> values;
};

namespace day_05 {

void run() {
  auto input = shared::read_file("inputs/day_05.txt");
  fmt::print("{}\n", part1(input));
  fmt::print("{}\n", part2(input));
}

auto parse_input(const std::string &input) {
  std::vector<Range> fresh_ranges;
  std::vector<long long> values;
  std::istringstream stream(input);
  std::string line;

  bool parsing_ranges = true;
  while (std::getline(stream, line)) {
    if (line.empty()) {
      parsing_ranges = false;
      continue;
    }

    if (parsing_ranges) {
      auto dash_pos = line.find('-');
      fresh_ranges.push_back({std::stoll(line.substr(0, dash_pos)),
                              std::stoll(line.substr(dash_pos + 1))});
      continue;
    } else {
      values.push_back(std::stoll(line));
    }
  }

  return Ingredients{fresh_ranges, values};
}

std::string part1(const std::string &input) {
  auto ingredients = parse_input(input);

  int fresh = 0;
  for (const auto value : ingredients.values) {
    for (const auto range : ingredients.ranges) {
      if (shared::in_range(value, range.first, range.second)) {
        ++fresh;
        break;
      }
    }
  }

  return std::to_string(fresh);
}

std::string part2(const std::string &input) {
  auto ingredients = parse_input(input);
  auto fresh = ingredients.ranges;

  std::sort(fresh.begin(), fresh.end(), [](const Range &a, const Range &b) {
    // 'a' comes before 'b'
    return a.first < b.first;
  });

  long fresh_count = 0;
  Range previous = *fresh.begin();
  for (auto it = fresh.begin(); it != fresh.end();) {
    // proceed to next range
    ++it;

    if (it == fresh.end()) {
      break;
    }

    // if ranges do not overlap
    if (it->first > previous.second) {
      // add to fresh count
      fresh_count += (previous.second - previous.first + 1);
      previous = *it;
      // else merge ranges
    } else {
      previous = Range{previous.first, std::max(previous.second, it->second)};
    }
  }

  fresh_count += (previous.second - previous.first + 1);

  return std::to_string(fresh_count);
}

} // namespace day_05
