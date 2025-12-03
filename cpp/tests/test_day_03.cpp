#include <catch2/catch_test_macros.hpp>

#include "twenty_five/day_03.h"

const std::string test_input =
    R"(987654321111111
811111111111119
234234234234278
818181911112111)";

TEST_CASE("Day 03 - Part 1") {
  std::string result = day_03::part1(test_input);
  REQUIRE(result == "357");
}

TEST_CASE("Day 03 - Part 2") {
  std::string result = day_03::part2(test_input);
  REQUIRE(result == "3121910778619");
}