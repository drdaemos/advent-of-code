#pragma once
#include <string>
#include <vector>

namespace shared {
std::string read_file(const std::string &file_path);
std::vector<std::string> read_lines(const std::string &file_path);
} // namespace shared
