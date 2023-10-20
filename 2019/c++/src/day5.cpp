#include <iostream>
#include <string>
#include "days.hpp"
#include "util.hpp"
#include "intcode.hpp"

void day5() {
    std::string input = read_input(5);
    IntcodeComputer computer(input);
    std::cout << "Part 1: " << computer.solve(1) << std::endl;
    computer.load_program(input);
    std::cout << "Part 2: " << computer.solve(5) << std::endl;
}

