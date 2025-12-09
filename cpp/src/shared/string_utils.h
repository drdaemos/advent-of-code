#pragma once

#include <string>
#include <vector>

namespace shared {

std::string repeat(std::string value, int n);

std::vector<std::string> get_lines(const std::string &value);

std::vector<std::string> split_string(const std::string &value,
                                      const std::string delimiter);

} // namespace shared