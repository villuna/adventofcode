#include <iostream>
#include <string>
#include <cstdint>
#include "days.hpp"
#include "util.hpp"
#include "intcode.hpp"

void day5() {
    std::string input = read_input(5);
    IntcodeComputer computer(input);

    std::vector<int64_t> in_vec;
    in_vec.push_back(1);
    computer.load_input(in_vec);
    std::cout << "Part 1: " << computer.run().code << std::endl;

    in_vec[0] = 5;
    computer.load_program(input);
    computer.load_input(in_vec);
    std::cout << "Part 2: " << computer.run().code << std::endl;
}

