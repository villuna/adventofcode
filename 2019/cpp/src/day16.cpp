#include "days.hpp"
#include "util.hpp"
#include <algorithm>
#include <execution>
#include <iostream>
#include <armadillo>
#include <string>

constexpr int PHASES = 100;

int fft_pattern(int row, int col) {
    int col_scaled = (int)std::floor((float)(col + 1) / (float)(row + 1)); 
    int out = col_scaled % 2;
    if (col_scaled % 4 == 3)
        out -= 2;

    return out;
}

int ones_place(int x) {
    if (x < 0) {
        x *= -1;
    }

    return x % 10;
}

arma::imat create_transformation_matrix(int size) {
    arma::imat trans(size, size);

    for (int i = 0; i < size; i++) {
        for (int j = 0; j < size; j++) {
            trans(i, j) = fft_pattern(i, j);
        }
    }

    return trans;
}

void d16_part1(const std::string& input) {
    int length = input.length();
    arma::ivec input_vec(length);

    for (int i = 0; i < length; i++) {
        input_vec(i) = input[i] - '0';
    }

    arma::imat trans = create_transformation_matrix(length);

    for (int _x = 0; _x < PHASES; _x++) {
        input_vec = trans * input_vec;
        for (int i = 0; i < length; i++) {
            input_vec(i) = ones_place(input_vec(i));
        }
    }

    std::cout << "Part 1: ";
    for (int i = 0; i < 8; i++)
        std::cout << input_vec(i);
    std::cout << std::endl;

}

void d16_part2(const std::string& input) {
    // For part 2, the matrix is so big that linear algebra libraries are futile
    // we have to do it ourselves using epic maffs
    int length = input.length();
    int offset = std::stoi(input.substr(0, 7));
    int real_length = length * 10000 - offset;
    std::vector<int> real_signal;

    for (int i = 0; i < real_length; i++) {
        int rev_index = length - (i % length) - 1;
        real_signal.push_back(input[rev_index] - '0');
    }

    for (int _x = 0; _x < PHASES; _x++) {
        // This far down along the matrix, the only numbers in the upper triangle are 1
        // so we don't even need to use the matrix at all. all the numbers are just linear
        // combinations.
        for (int k = 0; k < real_length; k++) {
            if (k != 0) {
                real_signal[k] += real_signal[k - 1];
            }
        }

        std::for_each(
            std::execution::par,
            real_signal.begin(),
            real_signal.end(),
            [] (int& i) {
                i = ones_place(i);
            }
        );
    }

    std::string output_string;

    for (int i = 1; i <= 8; i++) {
        output_string.push_back(real_signal[real_signal.size() - i] + '0');
    }

    std::cout << "Part 2: " << output_string << std::endl;
}

void day16() {
    std::string input = strip(read_input(16));
    d16_part1(input);
    d16_part2(input);
}
