#include <iostream>
#include <string>
#include "days.hpp"
#include "util.hpp"

int solve(std::vector<int> ops, int noun, int verb) {
    ops[1] = noun;
    ops[2] = verb;

    int pc = 0;

    while (true) {
        int op = ops[pc];
        int res;

        if (op == 1) {
            res = ops[ops[pc + 1]] + ops[ops[pc + 2]];
        } else if (op == 2) {
            res = ops[ops[pc + 1]] * ops[ops[pc + 2]];
        } else if (op == 99) {
            break;
        }

        ops[ops[pc + 3]] = res;
        pc += 4;
    }

    return ops[0];
}

void day2() {
    std::vector<std::string> input = split_string(read_input(2), ",");
    std::vector<int> ops;

    for (auto op : input) {
        ops.push_back(std::stoi(op)); 
    }

    std::cout << "Part 1: " << solve(ops, 12, 2) << std::endl;

    int target = 19690720;

    for (int i = 0; i <= 99; i++) {
        for (int j = 0; j <= 99; j++) {
            int res = solve(ops, i, j);

            if (res == target) {
                std::cout << "Part 2: (" << i << ", " << j << ") -> " << 100 * i + j << std::endl;
            }
        }
    }

}
