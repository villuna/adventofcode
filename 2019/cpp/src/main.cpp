#include <vector>
#include <exception>
#include <iostream>
#include <string>
#include "util.hpp"
#include "days.hpp"

#define DAYS 4

// I really do hate c's function pointer syntax
// This is an array of functions of type `fn() -> ()``
static void (*functions[DAYS])() = {
    day1,
    day2,
    day3,
    day4,
};

int main(int argc, char** argv) {
    if (argc != 2) {
        std::cerr << "usage: aocpp [day]" << std::endl;
        return 1;
    }

    int day = std::stoi(argv[1]) - 1;

    if (day < 0 || day >= DAYS) {
        std::cerr << "day not completed or invalid" << std::endl;
        return 1;
    }

    functions[day]();
    return 0;
}
