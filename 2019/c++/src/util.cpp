#include "util.hpp"

#include <fstream>
#include <stdexcept>
#include <string>
#include <vector>
#include <iostream>

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
