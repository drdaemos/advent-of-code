#include "day_01.h"

#include <fmt/core.h>

#include <regex>
#include <sstream>
#include <string>
#include <vector>

#include "shared/file_utils.h"

namespace day_01 {

struct Instruction {
  bool clockwise;
  int steps;
};

int apply_instruction(int dial, const Instruction& instruction) {
  if (!instruction.clockwise) {
    dial -= instruction.steps;
  } else if (instruction.clockwise) {
    dial += instruction.steps;
  } else {
    throw std::runtime_error("Invalid turn instruction: " +
                             std::to_string(instruction.clockwise));
  }
  return dial;
}

std::vector<Instruction> parse_instructions(const std::string& input) {
  std::vector<Instruction> instructions;
  std::istringstream stream(input);
  std::string line;
  while (std::getline(stream, line)) {
    instructions.push_back({line[0] == 'R', std::stoi(line.substr(1))});
  }
  return instructions;
}

void run() {
  auto input = shared::read_file("inputs/day_01.txt");
  fmt::print("{}\n", part1(input));
  fmt::print("{}\n", part2(input));
}

std::string part1(const std::string& input) {
  auto instructions = parse_instructions(input);
  auto at_zero = 0;
  auto dial = 50;

  for (const auto& instruction : instructions) {
    dial = apply_instruction(dial, instruction);
    dial = dial % 100;
    at_zero += dial == 0 ? 1 : 0;
  }

  return std::to_string(at_zero);
}

std::string part2(const std::string& input) {
  auto instructions = parse_instructions(input);
  auto at_zero = 0;
  auto dial = 50;

  for (const auto& instruction : instructions) {
    auto orig_dial = dial;
    dial = apply_instruction(dial, instruction);
    if (dial <= 0 && orig_dial > 0) {
      at_zero += std::abs(dial) / 100 + 1;
    } else {
      at_zero += std::abs(dial) / 100;
    }
    dial = ((dial % 100) + 100) % 100;
  }

  return std::to_string(at_zero);
}

}  // namespace day_01
