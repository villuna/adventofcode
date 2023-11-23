#include "days.hpp"
#include "util.hpp"
#include "intcode.hpp"
#include <iostream>
#include <stdexcept>
#include <thread>
#include <chrono>

enum tile {
    EMPTY = 0,
    WALL,
    BLOCK,
    PADDLE,
    BALL,
};

namespace std {
    template<typename T1, typename T2>
    struct hash<std::pair<T1, T2>> {
        std::uint64_t operator()(const std::pair<T1, T2>& pair) const {
            std::hash<T1> hasher1;
            std::hash<T2> hasher2;

            return hasher1(pair.first) ^ (hasher2(pair.second) << 1);
        }
    };
}

void part1(const std::string& input) {
    IntcodeComputer computer(input);

    std::unordered_map<std::pair<int, int>, tile> board;
    
    run_result res;
    computer.set_run_mode(RETURN_ON_OUTPUT);

    do {
        res = computer.run();
        int x = res.code;

        res = computer.run();
        int y = res.code;

        res = computer.run();
        tile t = (tile)res.code;

        board[std::pair(x, y)] = t;
    } while (res.type != TYPE_HALTED);

    int count = 0;

    for (auto i = board.begin(); i != board.end(); i++) {
        if (i->second == BLOCK) {
            count++;
        }
    }

    std::cout << "Part 1: " << count << std::endl;
}

std::string tile_char(tile t) {
    switch (t) {
        case EMPTY:
            return " ";
        case WALL:
            return "█";
        case BLOCK:
            return "#";
        case BALL:
            return "o";
        case PADDLE:
            return "=";
        default:
            throw std::invalid_argument("not a valid tile");
    }
}

struct game {
    IntcodeComputer computer;
    std::unordered_map<std::pair<int, int>, tile> board;
    run_result res;
    int score = 0;
    int width = 0;
    int height = 0;

    game(const std::string& input) : computer(input) {
        computer.set_instruction_at(0, 2);
        computer.set_run_mode(RETURN_ON_OUTPUT);
        computer.set_input_mode(WAIT_FOR_INPUT);
    }

    void draw() {
        while(true) {
            res = computer.run();
            if (res.type != TYPE_OUTPUT)
                break;

            int x = res.code;

            res = computer.run();
            if (res.type != TYPE_OUTPUT)
                break;

            int y = res.code;

            res = computer.run();
            if (res.type != TYPE_OUTPUT)
                break;

            int t = (tile)res.code;

            if (x == -1 && y == 0)
                score = t;
            else
                board[std::pair(x, y)] = (tile)t;
        }

        if (width == 0 && height == 0) {
            for (auto i = board.begin(); i != board.end(); i++) {
                if (i->first.first > width)
                    width = i->first.first;

                if (i->first.second > height)
                    height = i->first.second;
            }
        }

        std::cout << "SCORE : " << score << std::endl;
        for (int y = 0; y <= height; y++) {
            for (int x = 0; x <= width; x++) {
                std::cout << tile_char(board[std::pair(x, y)]);
            }
            std::cout << std::endl;
        }
    }

    void run() {
        do {
            draw();

            using namespace std::chrono_literals;
            std::this_thread::sleep_for(50ms);

            if (res.type != TYPE_BLOCKING)
                break;

            int next_input = calculate_next_input();
            computer.push_input(next_input);
        } while (res.type != TYPE_HALTED);
    }

    int calculate_next_input() {
        int ball_x = -1;
        int paddle_x = -1;

        for (auto i = board.begin(); i != board.end(); i++) {
            if (i->second == BALL)
                ball_x = i->first.first;

            else if (i->second == PADDLE)
                paddle_x = i->first.first;

            if (ball_x != -1 && paddle_x != -1)
                break;
        }

        if (paddle_x < ball_x)
            return 1;
        else if (paddle_x > ball_x)
            return -1;
        else
            return 0;
    }
};

void part2(const std::string& input) {
    game game(input);
    game.run();
}

void day13() {
    std::string input = read_input(13);
    part1(input); 
    part2(input);
}
