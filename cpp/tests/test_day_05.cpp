#include <catch2/catch_test_macros.hpp>

#include "twenty_five/day_05.h"

const std::string test_input =
    R"(3-5
10-14
16-20
12-18

1
5
8
11
17
32)";

TEST_CASE("Day 05 - Part 1") {
  std::string result = day_05::part1(test_input);
  REQUIRE(result == "3");
}

TEST_CASE("Day 05 - Part 2") {
  std::string result = day_05::part2(test_input);
  REQUIRE(result == "14");
}