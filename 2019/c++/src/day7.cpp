#include <cstdint>
#include <iostream>
#include <string>
#include <algorithm>
#include "days.hpp"
#include "util.hpp"
#include "intcode.hpp"

int run_simulation(IntcodeComputer& computer, std::vector<int> permutation, std::string program) {
    int value = 0;

    for (int setting : permutation) {
        computer.load_program(program);
        std::vector<int64_t> in_vec;
        in_vec.push_back(setting);
        in_vec.push_back(value);
        computer.load_input(in_vec);

        value = computer.run().code;
    }

    return value;
}

void part1(std::string program) {
    std::vector<int> perm = std::vector({0, 1, 2, 3, 4});
    IntcodeComputer computer;
    int max = 0;

    do {
        int res = run_simulation(computer, perm, program);

        if (res > max) {
            max = res;
        }

    } while (std::next_permutation(perm.begin(), perm.end()));

    std::cout << "max value: " << max << std::endl;
}

int run_feedback_simulation(std::vector<int> permutation, std::string program) {
    std::vector<IntcodeComputer> computers;

    for (int i = 0; i < permutation.size(); i++) {
        computers.push_back(IntcodeComputer(program));

        int setting = permutation[i];
        std::vector<int64_t> in_vec;
        in_vec.push_back(setting);
        in_vec.push_back(0);
        computers[i].load_input(in_vec);
        computers[i].set_run_mode(RETURN_ON_OUTPUT);
    }

    run_result res;
    res.code = 0;
    int i = 0;
    bool halted = false;

    while (!halted) {
        computers[i].set_input_at(1, res.code);

        res = computers[i].run();

        if (i == permutation.size() - 1 && res.halted) {
            halted = true;
        }

        i = (i + 1) % permutation.size();
    }

    return res.code;
}

void part2(std::string program) {
    std::vector<int> perm = std::vector({5, 6, 7, 8, 9});
    int max = 0;

    do {
        int res = run_feedback_simulation(perm, program);

        if (res > max) {
            max = res;
        }

    } while (std::next_permutation(perm.begin(), perm.end()));

    std::cout << "max value: " << max << std::endl;
}

void day7() {
    std::string program = read_input(7);
    part1(program);
    part2(program);
}
