#pragma once

namespace shared {
template <class T> int digits(T number) {
  int digits = 0;
  while (number) {
    number /= 10;
    digits++;
  }
  return digits;
}

template <typename T>
bool in_range(const T &value, const T &low, const T &high) {
  return !(value < low) && !(high < value);
}
} // namespace shared
