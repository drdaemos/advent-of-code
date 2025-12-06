#include <catch2/catch_test_macros.hpp>

#include "twenty_five/day_06.h"

const std::string test_input =
    R"(123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  )";

TEST_CASE("Day 06 - Part 1") {
  std::string result = day_06::part1(test_input);
  REQUIRE(result == "4277556");
}

TEST_CASE("Day 06 - Part 2") {
  std::string result = day_06::part2(test_input);
  REQUIRE(result == "3263827");
}