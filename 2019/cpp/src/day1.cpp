#include "days.hpp"
#include "util.hpp"

#include <iostream>

int extra_fuel(int fuel) {
    int sum = 0;
    int extra = fuel;

    while (extra > 0) {
        sum += extra;
        extra = (extra / 3) - 2;
    }

    return sum;
}

void solve_d1(std::vector<std::string>& input, int& p1, int& p2) {
    int sum = 0;
    int extra = 0;

    for (std::string word : input) {
        int weight = stoi(word);
        int fuel = (weight / 3) - 2;
        sum += fuel;
        extra += extra_fuel(fuel);
    }

    p1 = sum;
    p2 = extra;
}

void day1() {
    std::vector<std::string> input = read_input_lines(1);
    int p1, p2;

    solve_d1(input, p1, p2);

    std::cout << p1 << ", " << p2 << std::endl;
}
