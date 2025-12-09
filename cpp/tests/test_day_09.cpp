#include <catch2/catch_test_macros.hpp>

#include "twenty_five/day_09.h"

const std::string test_input = R"(7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3)";

TEST_CASE("Day 09 - Part 1") {
  std::string result = day_09::part1(test_input);
  REQUIRE(result == "50");
}

TEST_CASE("Day 09 - Part 2") {
  std::string result = day_09::part2(test_input);
  REQUIRE(result == "24");
}
