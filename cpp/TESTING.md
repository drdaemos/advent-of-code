# Testing Guide

This project uses [Catch2](https://github.com/catchorg/Catch2) for testing.

## Running Tests

From the project root directory:

```bash
# Build the project and tests
cd build
cmake ..
cmake --build .

# Run all tests
cd ..
./build/aoc_tests

# Run tests with verbose output
./build/aoc_tests -s

# Run specific tests by tag
./build/aoc_tests "[day01]"
./build/aoc_tests "[part1]"

# List all tests
./build/aoc_tests --list-tests
```

## Code Structure

Each day follows this pattern:

### Day Implementation (`src/twenty_five/day_XX.h`)

```cpp
#pragma once
#include <string>

namespace day_XX {
    void run();  // Reads input file and runs both parts
    std::string part1(const std::string& input);
    std::string part2(const std::string& input);
}
```

### Day Implementation (`src/twenty_five/day_XX.cpp`)

```cpp
#include "day_XX.h"
#include <fmt/core.h>
#include "shared/file_utils.h"

namespace day_XX {
    void run() {
        auto input = shared::read_file("inputs/day_XX.txt");
        fmt::print("{}\n", part1(input));
        fmt::print("{}\n", part2(input));
    }

    std::string part1(const std::string& input) {
        // Your solution here
        return "answer";
    }

    std::string part2(const std::string& input) {
        // Your solution here
        return "answer";
    }
}
```

### Main Program

The main program just calls `run()`:

```cpp
// In main.cpp
std::map<int, std::function<void()>> solutions = {
    {1, day_01::run},
};
```

## Writing Tests

Tests hardcode the example input inline and call `part1()` and `part2()` directly:

```cpp
#include <catch2/catch_test_macros.hpp>
#include "twenty_five/day_XX.h"

TEST_CASE("Day XX - Part 1 - Example", "[dayXX][part1]") {
    const std::string test_input = R"(your
example
input
here)";

    std::string result = day_XX::part1(test_input);
    REQUIRE(result == "expected answer");
}

TEST_CASE("Day XX - Part 2 - Example", "[dayXX][part2]") {
    const std::string test_input = R"(your
example
input
here)";

    std::string result = day_XX::part2(test_input);
    REQUIRE(result == "expected answer");
}
```

### Common Assertions

```cpp
REQUIRE(result == "expected");     // Equality
REQUIRE(result != "unexpected");   // Inequality
REQUIRE(!result.empty());          // Non-empty check
REQUIRE(value > 0);                // Comparison
```

## Catch2 Features

Catch2 provides many useful features:

- **Sections**: Group related test steps
- **BDD-style**: SCENARIO, GIVEN, WHEN, THEN
- **Matchers**: Advanced assertions
- **Benchmarking**: Performance testing

See the [Catch2 documentation](https://github.com/catchorg/Catch2/blob/devel/docs/Readme.md) for more details.
