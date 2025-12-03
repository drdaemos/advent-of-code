#include <sstream>
#pragma once

namespace shared {

std::string repeat(std::string value, int n) {
  std::ostringstream os;
  for (int i = 0; i < n; i++)
    os << value;
  return os.str();
}

} // namespace shared