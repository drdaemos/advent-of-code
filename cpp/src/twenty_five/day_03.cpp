#include "day_03.h"

#include <fmt/core.h>
#include <fmt/ranges.h>

#include <iostream>
#include <numeric>
#include <set>
#include <sstream>
#include <string>
#include <tuple>
#include <vector>

#include "shared/file_utils.h"

namespace day_03 {

void run() {
  auto input = shared::read_file("inputs/day_03.txt");
  fmt::print("{}\n", part1(input));
  fmt::print("{}\n", part2(input));
}

struct Bank {
  std::vector<int> batteries;
};

struct BatteryToUse {
  int value;
  int rank;
};

std::vector<Bank> parse_input(const std::string& input) {
  std::vector<Bank> banks;
  std::istringstream stream(input);
  std::string line;
  while (std::getline(stream, line)) {
    std::vector<int> batteries;
    for (char c : line) {
      batteries.push_back(c - '0');  // convert char to int
    }
    banks.push_back(Bank{batteries});
  }
  return banks;
}

BatteryToUse find_max_digit(std::vector<int> values, int min_rank, int place) {
  int max_value = 0;
  int rank = 0;
  for (int i = min_rank; i < values.size() - place; ++i) {
    if (values.at(i) > max_value) {
      max_value = values.at(i);
      rank = i;
    }
  }

  return BatteryToUse{max_value, rank};
}

auto format_as(const BatteryToUse& p) { return fmt::format("{}", p.value); }

long build_joltage(const std::vector<BatteryToUse>& digits) {
  std::string joltage_str;
  for (const auto& digit : digits) {
    joltage_str += std::to_string(digit.value);
  }
  return std::stol(joltage_str);
}

std::string part1(const std::string& input) {
  auto banks = parse_input(input);
  std::vector<int> joltages = {};

  for (const auto& bank : banks) {
    auto digit_1 = find_max_digit(bank.batteries, 0, 1);
    auto digit_2 = find_max_digit(bank.batteries, digit_1.rank + 1, 0);

    auto joltage = std::format("{}{}", digit_1.value, digit_2.value);
    joltages.push_back(std::stoi(joltage));
  }

  return std::to_string(std::reduce(joltages.begin(), joltages.end(), 0));
}

std::string part2(const std::string& input) {
  auto banks = parse_input(input);
  std::vector<long> joltages = {};

  for (const auto& bank : banks) {
    std::vector<BatteryToUse> digits = {};
    const int MAX_PLACES = 12;

    for (int place = MAX_PLACES; place > 0; --place) {
      auto min_rank = 0;
      if (!digits.empty()) {
        min_rank = digits.back().rank + 1;
      }
      auto digit = find_max_digit(bank.batteries, min_rank, place - 1);
      digits.push_back(digit);
    }

    auto joltage = build_joltage(digits);
    joltages.push_back(joltage);
  }

  return std::to_string(std::reduce(joltages.begin(), joltages.end(), 0L));
}

}  // namespace day_03
