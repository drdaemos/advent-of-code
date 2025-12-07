#include "file_utils.h"

#include <fstream>
#include <sstream>
#include <stdexcept>

#include "string_utils.h"

namespace shared {
std::string read_file(const std::string &file_path) {
  std::ifstream file(file_path);

  if (!file.is_open()) {
    throw std::runtime_error("Failed to open file: " + file_path);
  }

  std::stringstream buffer;
  buffer << file.rdbuf();

  return buffer.str();
}

std::vector<std::string> read_lines(const std::string &file_path) {
  auto content = read_file(file_path);
  return shared::get_lines(content);
}
} // namespace shared
