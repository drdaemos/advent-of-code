#include <sstream>

namespace shared {
std::string repeat(std::string value, int n) {
  std::ostringstream os;
  for (int i = 0; i < n; i++)
    os << value;
  return os.str();
}

std::vector<std::string> get_lines(const std::string &value) {
  std::vector<std::string> result;
  std::istringstream stream(value);
  std::string line;
  while (std::getline(stream, line)) {
    result.push_back(line);
  }
  return result;
}
} // namespace shared