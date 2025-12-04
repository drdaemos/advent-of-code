#include <catch2/catch_test_macros.hpp>

#include "twenty_five/day_04.h"

const std::string test_input =
    R"(..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.)";

TEST_CASE("Day 04 - Part 1") {
  std::string result = day_04::part1(test_input);
  REQUIRE(result == "13");
}

TEST_CASE("Day 04 - Part 2") {
  std::string result = day_04::part2(test_input);
  REQUIRE(result == "43");
}