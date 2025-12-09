#include <catch2/catch_test_macros.hpp>

#include "twenty_five/day_08.h"

const std::string test_input = R"(162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689)";

TEST_CASE("Day 08 - Part 1") {
  std::string result = day_08::part1(test_input);
  REQUIRE(result == "40");
}

TEST_CASE("Day 08 - Part 2") {
  std::string result = day_08::part2(test_input);
  REQUIRE(result == "25272");
}
