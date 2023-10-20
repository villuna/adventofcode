#include <iostream>
#include <stdexcept>
#include "intcode.hpp"
#include "util.hpp"

enum opcode {
    ADD = 1,
    MUL = 2,
    INPUT = 3,
    OUTPUT = 4,
    JUMPIFTRUE = 5,
    JUMPIFFALSE = 6,
    LESSTHAN = 7,
    EQUAL = 8,
    HALT = 99,
};

std::vector<int> read_op(int op) {
    std::vector<int> out;

    out.push_back(op % 100);

    op /= 100;

    while (op != 0) {
        out.push_back(op % 10);
        op /= 10;
    }

    return out;
}

void IntcodeComputer::load_program(const std::string& input) {
    pc = 0;
    std::vector<std::string> input_split = split_string(input, ",");
    ops.clear();

    for (std::string op : input_split) {
        ops.push_back(std::stoi(op));
    }
}

int IntcodeComputer::solve(int input) {
    pc = 0;
    int output = 0;

    while (true) {
        if (pc < 0 || pc >= ops.size()) {
            throw std::runtime_error("program counter out of bounds");
        }

        std::vector<int> op_vec = read_op(ops[pc]);

        // If the next opt is halt, we don't care what the previous output was
        if (op_vec[0] == HALT) {
            break;
        }

        // Otherwise we should check the previous output is 0
        if (output != 0) {
            throw std::runtime_error("program error: code " + std::to_string(output));
        }

        if (op_vec[0] == ADD) {
            int a = value(op_vec, 1);
            int b = value(op_vec, 2);
            int addr = wr_addr(op_vec, 3);

            ops[addr] = a + b;
            pc += 4;
        } else if (op_vec[0] == MUL) {
            int a = value(op_vec, 1);
            int b = value(op_vec, 2);
            int addr = wr_addr(op_vec, 3);

            ops[addr] = a * b;
            pc += 4;
        } else if (op_vec[0] == INPUT) {
            int addr = wr_addr(op_vec, 1);
            ops[addr] = input;
            pc += 2;
        } else if (op_vec[0] == OUTPUT) {
            int addr = value(op_vec, 1);
            output = addr;
            pc += 2;
        } else if (op_vec[0] == JUMPIFTRUE) {
            int param = value(op_vec, 1);
            int addr = value(op_vec, 2);

            if (param != 0) {
                pc = addr;
            } else {
                pc += 3;
            }
        } else if (op_vec[0] == JUMPIFFALSE) {
            int param = value(op_vec, 1);
            int addr = value(op_vec, 2);

            if (param == 0) {
                pc = addr;
            } else {
                pc += 3;
            }
        } else if (op_vec[0] == LESSTHAN) {
            int a = value(op_vec, 1);
            int b = value(op_vec, 2);
            int addr = wr_addr(op_vec, 3);

            if (a < b) {
                ops[addr] = 1;
            } else {
                ops[addr] = 0;
            }

            pc += 4;
        } else if (op_vec[0] == EQUAL) {
            int a = value(op_vec, 1);
            int b = value(op_vec, 2);
            int addr = wr_addr(op_vec, 3);

            if (a == b) {
                ops[addr] = 1;
            } else {
                ops[addr] = 0;
            }

            pc += 4;
        } else {
            throw std::runtime_error("incorrect operation! " + std::to_string(ops[pc]));
        }
    }

    return output;
}
    
int IntcodeComputer::value(std::vector<int>& op_vec, int offset) {
    int mode;

    if (offset < op_vec.size()) {
        mode = op_vec[offset];
    } else {
        mode = 0;
    }

    int res;
    if (mode == 0) {
        res = ops[ops[pc + offset]];
    } else if (mode == 1) {
        res = ops[pc + offset];
    } else {
        throw std::invalid_argument("mode should be 0 or 1, got " + std::to_string(mode));
    }

    return res;
}

int IntcodeComputer::wr_addr(std::vector<int>& op_vec, int offset) {
    return ops[pc + offset];
}
