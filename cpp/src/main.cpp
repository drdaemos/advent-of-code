#include <fmt/base.h>

#include <argparse/argparse.hpp>
#include <functional>
#include <map>
#include <string>

#include "twenty_five/day_01.h"
#include "twenty_five/day_02.h"
#include "twenty_five/day_03.h"

int main(int argc, char **argv) {
  argparse::ArgumentParser program("Advent of Code");

  program.add_argument("-d")
      .required()
      .help("run the solution for the specified day")
      .scan<'i', int>();

  try {
    program.parse_args(argc, argv);
  } catch (const std::exception &err) {
    fmt::print("Error: {}\n\n", err.what());
    return 1;
  }

  auto day = program.get<int>("-d");

  // Map of day number to run functions
  std::map<int, std::function<void()>> solutions = {
      {1, day_01::run},
      {2, day_02::run},
      {3, day_03::run},
  };

  auto it = solutions.find(day);
  if (it == solutions.end()) {
    fmt::print("Error: Day {} not implemented yet\n", day);
    return 1;
  }

  fmt::print("Running solution for day {}\n\n", day);
  it->second();

  return 0;
}