#include "days.hpp"
#include "util.hpp"
#include "intcode.hpp"
#include <iostream>
#include <map>
#include <stdexcept>

enum direction {
    UP = 0,
    LEFT,
    DOWN,
    RIGHT,
};

direction rotate(direction d, int command) {
    if (command == 0) {
        d = (direction)((d + 1) % 4);
    } else if (command == 1) {
        int dn = d - 1;

        if (dn < 0) {
            dn = 3;
        }

        d = (direction)dn;
    } else {
        throw std::invalid_argument("command must be 1 or 0");
    }

    return d;
}

std::pair<int, int> move(std::pair<int, int> pos, direction dir) {
    switch (dir) {
        case UP:
            pos.second -= 1;
            break;
        case LEFT:
            pos.first -= 1;
            break;
        case DOWN:
            pos.second += 1;
            break;
        case RIGHT:
            pos.first += 1;
    }

    return pos;
}

std::map<std::pair<int, int>, int> paint(IntcodeComputer& computer, int start_colour) {
    std::map<std::pair<int, int>, int> board;
    std::pair<int, int> position = std::pair(0, 0);
    direction current_direction = UP;

    board[position] = start_colour;

    std::vector<int64_t> input({0});
    computer.load_input(input);

    run_result res;

    do {
        int current_colour;

        if (auto entry = board.find(position); entry != board.end()) {
            current_colour = entry->second;
        } else {
            current_colour = 0;
        }

        computer.set_input_at(0, current_colour);
        res = computer.run();

        board[position] = res.code;

        res = computer.run();
        int command = res.code;

        current_direction = rotate(current_direction, command);
        position = move(position, current_direction);
    } while (res.type != TYPE_HALTED);

    return board;
}

void part1(IntcodeComputer& computer) {
    auto board = paint(computer, 0);

    int count = 0;
    for (auto i = board.begin(); i != board.end(); i++) {
        count++;
    }

    std::cout << count << std::endl;
}

void part2(IntcodeComputer& computer) {
    auto board = paint(computer, 1);

    int minx = 0, maxx = 0, miny = 0, maxy = 0;

    for (auto i = board.begin(); i != board.end(); i++) {
        int x = i->first.first;
        int y = i->first.second;

        if (x < minx)
            minx = x;
        else if (x > maxx)
            maxx = x;
        if (y < miny)
            miny = y;
        else if (y > maxy)
            maxy = y;
    }

    for (int y = miny; y <= maxy; y++) {
        for (int x = minx; x <= maxx; x++) {
            if (auto entry = board.find(std::pair(x, y)); entry != board.end()) {
                if (entry->second == 0)
                    std::cout << " ";
                else
                    std::cout << "#";
            } else {
                std::cout << " ";
            }
        }
        std::cout << std::endl;
    }
}

void day11() {
    std::string input = read_input(11);
    IntcodeComputer computer(input);
    computer.set_run_mode(RETURN_ON_OUTPUT);
    part1(computer);
    computer.load_program(input);
    part2(computer);
}
