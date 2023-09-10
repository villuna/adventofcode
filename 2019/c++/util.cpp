#include "util.hpp"

#include <fstream>
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
        std::cerr << "couldn't open file: " << filename << std::endl;
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
        std::cerr << "couldn't open file: " << filename << std::endl;
    }

    return res;
}
