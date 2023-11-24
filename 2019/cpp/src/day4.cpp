#include "days.hpp"
#include <iostream>

#define LOW 172930
#define HIGH 683082

bool p1_valid(int password) {
    int prev_digit = 10;
    bool has_double = false;

    for (int i = 0; i < 6; i++) {
        int digit = password % 10;
        if (digit > prev_digit) {
            return false;
        }

        if (digit == prev_digit) {
            has_double = true;
        }

        prev_digit = digit;
        password /= 10;
    }

    return has_double;
}

bool p2_valid(int password) {
    int prev_digit = 10;
    bool has_double = false;

    for (int i = 0; i < 6; i++) {
        int digit = password % 10;
        if (digit > prev_digit) {
            return false;
        }

        if (!has_double && i < 5) {
            bool is_start_of_double = true;

            // If this is not the first digit, it must not be equal to the last digit
            is_start_of_double &= (i == 0 || digit != prev_digit);
            // It must be equal to the next digit
            is_start_of_double &= digit == (password / 10) % 10;
            // It must not be equal to the digit after that (if it exists)
            is_start_of_double &= (i == 4 || digit != (password / 100) % 10);

            has_double |= is_start_of_double;
        }

        prev_digit = digit;
        password /= 10;
    }

    return has_double;
}

int numsbetween_naive(int low, int high, bool (*is_valid)(int)) {
    int res = 0;

    for (int i = low; i < high; i++) {
        if (is_valid(i)) {
            res++;
        }
    }

    return res;
}

void day4() {
    std::cout << "Part 1: " << numsbetween_naive(LOW, HIGH, p1_valid) << std::endl;
    std::cout << "Part 2: " << numsbetween_naive(LOW, HIGH, p2_valid) << std::endl;
}
