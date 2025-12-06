#pragma once

#include <cmath>
#include <cstddef>

namespace shared {
template <class T> int digits(T number) {
  int digits = 0;
  while (number) {
    number /= 10;
    digits++;
  }
  return digits;
}

template <class T> T get_nth_digit(T number, size_t n) {
  // n=0 gives the rightmost (units) digit
  // n=1 gives the tens digit, etc.
  // Use integer division and modulo
  // Note: pow returns a double, so cast to int
  T result = (number / (T)std::pow(10, n)) % 10;
  return result;
}

template <typename T>
bool in_range(const T &value, const T &low, const T &high) {
  return !(value < low) && !(high < value);
}
} // namespace shared
