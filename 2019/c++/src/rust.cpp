#include <cstdint>
#include <cstdlib>
#include <iostream>
#include "days.hpp"
#include "util.hpp"
#include "rust.hpp"

#define RUST_DAY(n) extern "C" { \
    void rust_day##n(const char *input); \
} \
void day##n() {\
    std::string input = read_input(n); \
    rust_day##n(input.c_str()); \
}

void print_rust_str(const char *str, size_t str_len) {
    for (int i = 0; i < str_len; i++)
        std::cerr << str[i];
}

void handle_panic(const char* msg, size_t msg_len, const char* file, size_t file_len, uint32_t line_num) {
    std::cerr << "Panic! At line " << line_num << ", file ";

    print_rust_str(file, file_len);
    std::cerr << std::endl;

    print_rust_str(msg, msg_len);
    std::cerr << std::endl;

    abort();
}

RUST_DAY(3);
RUST_DAY(8);
