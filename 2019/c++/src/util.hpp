#ifndef UTIL_HPP
#define UTIL_HPP

#include <string>
#include <vector>

std::string read_input(int day);
std::vector<std::string> read_input_lines(int day);
std::vector<std::string> split_string(const std::string& in, const std::string& delimiter);

#endif
