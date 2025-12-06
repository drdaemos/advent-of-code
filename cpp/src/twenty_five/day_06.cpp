#include "day_06.h"

#include <fmt/base.h>
#include <fmt/ranges.h>

#include <numeric>
#include <regex>
#include <sstream>
#include <string>

#include "shared/file_utils.h"

using Problem = std::pair<std::vector<long long>, char>;

namespace day_06 {

void run() {
  auto input = shared::read_file("inputs/day_06.txt");
  fmt::print("{}\n", part1(input));
  fmt::print("{}\n", part2(input));
}

auto parse_input(const std::string &input) {
  std::vector<Problem> problems;
  std::istringstream stream(input);
  std::string line;
  std::regex pattern(R"((\d+|[*+]))");

  size_t column = 0;
  while (std::getline(stream, line)) {
    column = 0;
    auto matches_begin =
        std::sregex_iterator(line.begin(), line.end(), pattern);

    for (auto i = matches_begin; i != std::sregex_iterator(); ++i) {
      auto match_str = (*i).str();

      if (column >= problems.size()) {
        problems.resize(column + 1, Problem{{}, '?'});
      }

      if (match_str == "*" || match_str == "+") {
        problems.at(column).second = match_str[0];
      } else {
        problems.at(column).first.push_back(std::stoll(match_str));
      }

      ++column;
    }
  }

  return problems;
}

long long solve_problems(const std::vector<Problem> &problems) {
  long long result = 0;

  for (const auto &problem : problems) {
    if (problem.second == '*') {
      result += std::accumulate(problem.first.begin(), problem.first.end(), 1LL,
                                std::multiplies<long long>());
    } else if (problem.second == '+') {
      result += std::accumulate(problem.first.begin(), problem.first.end(), 0LL,
                                std::plus<long long>());
    }
  }

  return result;
}

std::string part1(const std::string &input) {
  return std::to_string(solve_problems(parse_input(input)));
}

auto parse_input_2(const std::string &input) {
  std::vector<Problem> problems;
  std::istringstream stream(input);
  std::string line;
  std::vector<std::string> lines;

  while (std::getline(stream, line)) {
    lines.push_back(line);
  }

  size_t row_count = lines.size();
  size_t column_count = lines[0].size();
  std::vector<long long> values = {};

  // check by column right-to-left
  for (int col = (column_count - 1); col >= 0; --col) {
    std::string value;
    // check all rows by column except last with operator sign
    for (size_t row = 0; row < row_count - 1; ++row) {
      auto ch = lines[row][col];
      if (ch >= '0' && ch <= '9') {
        value += lines[row][col];
      }
    }

    if (!value.empty()) {
      values.push_back(std::stoll(value));
    }

    // if operator is met - flush problem
    auto op = lines[row_count - 1][col];
    if (op == '*' || op == '+') {
      problems.push_back({values, op});
      values = {};
    }
  }

  return problems;
}

std::string part2(const std::string &input) {
  return std::to_string(solve_problems(parse_input_2(input)));
}

} // namespace day_06
