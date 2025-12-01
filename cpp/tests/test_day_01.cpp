#include <catch2/catch_test_macros.hpp>

#include "twenty_five/day_01.h"

const std::string test_input = R"(L68
L30
R48
L5
R60
L55
L1
L99
R14
L82)";

TEST_CASE("Day 01 - Part 1 - Example", "[day01][part1]") {
  std::string result = day_01::part1(test_input);
  REQUIRE(result == "3");
}

TEST_CASE("Day 01 - Part 2 - Example", "[day01][part2]") {
  std::string result = day_01::part2(test_input);
  REQUIRE(result == "6");
}