#include <ranges>
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

std::vector<std::string> split_string(const std::string &value,
                                      const std::string delimiter) {
  std::string_view string_view{value};
  std::string_view delimiter_view{delimiter};

  auto split_view = std::views::split(string_view, delimiter_view);
  std::vector<std::string> tokens =
      std::ranges::to<std::vector<std::string>>(split_view);
  return tokens;
}

} // namespace shared