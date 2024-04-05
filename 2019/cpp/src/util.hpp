#ifndef UTIL_HPP
#define UTIL_HPP

#include <string>
#include <vector>
#include <sstream>
#include <cstdint>

struct coord {
    int x;
    int y;

    coord& operator+=(const coord& rhs) {
        x += rhs.x;
        y += rhs.y;
        return *this;
    }

    int norm();
};

bool operator==(const coord lhs, const coord rhs);
coord operator+(const coord lhs, const coord rhs);
coord operator-(const coord lhs, const coord rhs);
coord make_coord(int x, int y);

namespace std {
    template<>
    struct hash<coord> {
        std::uint64_t operator()(const coord& t) const {
            std::hash<std::string> s;
            std::ostringstream out("");
            out << t.x << t.y;

            return s(out.str());
        }
    };
}

std::string read_input(int day);
std::vector<std::string> read_input_lines(int day);
std::vector<std::string> split_string(const std::string& in, const std::string& delimiter);

#endif
