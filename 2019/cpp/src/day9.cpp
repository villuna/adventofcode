#include "days.hpp"
#include "util.hpp"
#include "intcode.hpp"
#include <cstdint>
#include <iostream>

void day9() {
    std::string program = read_input(9);
    IntcodeComputer computer(program);
    std::vector<int64_t> in_vec;
    in_vec.push_back(1);
    computer.load_input(in_vec);

    std::cout << "part 1: " << computer.run().code << std::endl;

    in_vec[0] = 2;
    computer.load_program(program);
    computer.load_input(in_vec);

    std::cout << "part 2: " << computer.run().code << std::endl;
}
