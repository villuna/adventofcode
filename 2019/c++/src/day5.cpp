#include <iostream>
#include <string>
#include "days.hpp"
#include "util.hpp"
#include "intcode.hpp"

void day5() {
    std::string input = read_input(5);
    IntcodeComputer computer(input);

    computer.load_input(std::vector({1}));
    std::cout << "Part 1: " << computer.run().code << std::endl;

    computer.load_program(input);
    computer.load_input(std::vector({5}));
    std::cout << "Part 2: " << computer.run().code << std::endl;
}

