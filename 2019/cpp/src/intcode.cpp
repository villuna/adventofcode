#include <iostream>
#include <stdexcept>
#include <cstdint>
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
    OFFSET = 9,
    HALT = 99,
};

enum mode {
    POSITION = 0,
    IMMEDIATE = 1,
    RELATIVE = 2,
};

std::vector<int64_t> read_op(int64_t op) {
    std::vector<int64_t> out;

    out.push_back(op % 100);

    op /= 100;

    while (op != 0) {
        out.push_back(op % 10);
        op /= 10;
    }

    return out;
}

void IntcodeComputer::load_program(const std::string& input) {
    std::vector<std::string> input_split = split_string(input, ",");
    pc = 0;
    output = 0;
    relative_base = 0;
    ops.clear();
    extra_ops.clear();

    for (std::string op : input_split) {
        ops.push_back(std::stoll(op));
    }
}

void IntcodeComputer::load_input(const std::vector<int64_t>& input) {
    this->input = input;
    input_index = 0;
}

void IntcodeComputer::set_input_at(int64_t index, int64_t value) {
    if (index < input.size()) {
        input[index] = value;
    }
}

void IntcodeComputer::set_run_mode(run_mode mode) {
    this->mode = mode;
}

run_result IntcodeComputer::run() {
    while (true) {
        if (pc < 0) {
            throw std::runtime_error("program counter out of bounds");
        }

        std::vector<int64_t> op_vec = read_op(get_op(pc));

        if (op_vec[0] == HALT) {
            return {
                .code = output,
                .halted = true,
            };
        }

        if (op_vec[0] == ADD) {
            int64_t a = value(op_vec, 1);
            int64_t b = value(op_vec, 2);
            int64_t addr = wr_addr(op_vec, 3);

            set_op(addr, a + b);
            pc += 4;
        } else if (op_vec[0] == MUL) {
            int64_t a = value(op_vec, 1);
            int64_t b = value(op_vec, 2);
            int64_t addr = wr_addr(op_vec, 3);

            set_op(addr, a * b);
            pc += 4;
        } else if (op_vec[0] == INPUT) {
            int64_t addr = wr_addr(op_vec, 1);
            set_op(addr, input[input_index]);

            if (input_index < input.size() - 1) {
                input_index++;
            }

            pc += 2;
        } else if (op_vec[0] == OUTPUT) {
            int64_t addr = value(op_vec, 1);
            output = addr;
            pc += 2;

            if (mode == RETURN_ON_OUTPUT) {
                return {
                    .code = output,
                    .halted = false,
                };
            }
        } else if (op_vec[0] == JUMPIFTRUE) {
            int64_t param = value(op_vec, 1);
            int64_t addr = value(op_vec, 2);

            if (param != 0) {
                pc = addr;
            } else {
                pc += 3;
            }
        } else if (op_vec[0] == JUMPIFFALSE) {
            int64_t param = value(op_vec, 1);
            int64_t addr = value(op_vec, 2);

            if (param == 0) {
                pc = addr;
            } else {
                pc += 3;
            }
        } else if (op_vec[0] == LESSTHAN) {
            int64_t a = value(op_vec, 1);
            int64_t b = value(op_vec, 2);
            int64_t addr = wr_addr(op_vec, 3);

            if (a < b) {
                set_op(addr, 1);
            } else {
                set_op(addr, 0);
            }

            pc += 4;
        } else if (op_vec[0] == EQUAL) {
            int64_t a = value(op_vec, 1);
            int64_t b = value(op_vec, 2);
            int64_t addr = wr_addr(op_vec, 3);

            if (a == b) {
                set_op(addr, 1);
            } else {
                set_op(addr, 0);
            }

            pc += 4;
        } else if (op_vec[0] == OFFSET) {
            int64_t off = value(op_vec, 1);
            relative_base += off;
            pc += 2;
        } else {
            throw std::runtime_error("incorrect operation at pc=" + std::to_string(pc) + ": " + std::to_string(ops[pc]));
        }
    }
}

int64_t IntcodeComputer::get_op(int64_t index) {
    if (index < ops.size()) {
        return ops[index];
    } else {
        auto op = extra_ops.find(index);

        if (op == extra_ops.end()) {
            return 0;
        } else {
            return op->second; 
        }
    }
}

void IntcodeComputer::set_op(int64_t index, int64_t value) {
    if (index < ops.size()) {
        ops[index] = value;
    } else {
        extra_ops[index] = value;
    }
}
    
int64_t IntcodeComputer::value(std::vector<int64_t>& op_vec, int64_t offset) {
    int64_t mode;

    if (offset < op_vec.size()) {
        mode = op_vec[offset];
    } else {
        mode = 0;
    }

    int64_t res;
    if (mode == POSITION) {
        res = get_op(get_op(pc + offset));
    } else if (mode == IMMEDIATE) {
        res = get_op(pc + offset);
    } else if (mode == RELATIVE) {
        res = get_op(get_op(pc + offset) + relative_base);
    } else {
        throw std::invalid_argument("unrecognised mode: " + std::to_string(mode));
    }

    return res;
}

int64_t IntcodeComputer::wr_addr(std::vector<int64_t>& op_vec, int64_t offset) {
    int64_t mode;

    if (offset < op_vec.size()) {
        mode = op_vec[offset];
    } else {
        mode = 0;
    }

    if (mode == POSITION) {
        return get_op(pc + offset);
    } else if (mode == RELATIVE) {
        return get_op(pc + offset) + relative_base;
    } else {
        throw std::invalid_argument("unsupported write mode: " + std::to_string(mode));
    }
}
