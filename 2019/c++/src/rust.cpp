#include <cstdint>
#include <cstdlib>
#include <iostream>
#include "days.hpp"
#include "util.hpp"
#include "rust.hpp"

struct Day3Result {
    int p1;
    int p2;
};
extern "C" {
    Day3Result rust_day3(const char *input);
}

void handle_panic(const char* msg, size_t msg_len, const char* file, size_t file_len, uint32_t line_num) {
    std::cerr << "Panic! At line " << line_num << ", file ";

    for (int i = 0; i < file_len; i++)
        std::cerr << file[i];

    std::cerr << std::endl;

    for (int i = 0; i < msg_len; i++)
        std::cerr << msg[i];

    std::cerr << std::endl;

    abort();
}

void day3() {
    std::string input = read_input(3);
    std::cout << input << std::endl;
    Day3Result res = rust_day3(input.c_str());

    std::cout << "part 1: " << res.p1 << std::endl;
    std::cout << "part 2: " << res.p2 << std::endl;
}
