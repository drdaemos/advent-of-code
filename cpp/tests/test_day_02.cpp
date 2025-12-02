#include <catch2/catch_test_macros.hpp>

#include "twenty_five/day_02.h"

const std::string test_input =
    R"(11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124)";

TEST_CASE("Day 02 - Part 1") {
  std::string result = day_02::part1(test_input);
  REQUIRE(result == "1227775554");
}

TEST_CASE("Day 02 - Part 2") {
  std::string result = day_02::part2(test_input);
  REQUIRE(result == "4174379265");
}