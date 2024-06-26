#include "util.hpp"

#include <cctype>
#include <cstdlib>
#include <fstream>
#include <stdexcept>
#include <string>
#include <vector>

bool operator==(const coord lhs, const coord rhs) {
    return lhs.x == rhs.x && lhs.y == rhs.y;
}

coord operator+(const coord lhs, const coord rhs) {
    return {.x = lhs.x + rhs.x, .y = lhs.y + rhs.y};
}

coord operator-(const coord lhs, const coord rhs) {
    return {.x = lhs.x - rhs.x, .y = lhs.y - rhs.y};
}

coord make_coord(int x, int y) {
    return {.x = x, .y = y};
}

int coord::norm() {
    return std::abs(x) + std::abs(y);
}

std::string strip(std::string input) {
    std::string output;
    int begin = 0;

    for (auto c = input.begin(); c != input.end(); c++) {
        if (std::isspace(static_cast<unsigned char>(*c))) {
            begin++;
        } else {
            break;
        }
    }

    int end;
    for (end = input.length(); end > begin; end--) {
        if (!std::isspace(static_cast<unsigned char>(input[end - 1]))) {
            break;
        }
    }

    return input.substr(begin, end - begin);
}

std::string read_input(int day) {
    std::string filename = "../input/day" + std::to_string(day) + ".txt";

    std::ifstream in(filename);
    std::string res;

    if (in.is_open()) {
        std::string line;

        while (std::getline(in, line)) {
            res += line + "\n";
        }

        in.close();
    } else {
        throw std::invalid_argument("file does not exist");
    }

    return res;
}

std::vector<std::string> read_input_lines(int day) {
    std::string filename = "../input/day" + std::to_string(day) + ".txt";

    std::ifstream in(filename);
    std::vector<std::string> res;

    if (in.is_open()) {
        std::string line;

        while (std::getline(in, line)) {
            res.push_back(line);
        }

        in.close();
    } else {
        throw std::invalid_argument("file does not exist");
    }

    return res;
}

std::vector<std::string> split_string(const std::string& in, const std::string& delimiter) {
    // written with help from https://stackoverflow.com/questions/14265581/parse-split-a-string-in-c-using-string-delimiter-standard-c
    std::vector<std::string> res;
    size_t position = 0;
    size_t next;

    while ((next = in.find(delimiter, position)) != std::string::npos) {
        res.push_back(in.substr(position, next - position));
        position = next + delimiter.length();
    }

    res.push_back(in.substr(position, next));

    return res;
}
