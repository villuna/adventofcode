#ifndef RUST_HPP
#define RUST_HPP

#include <cstddef>
#include <cstdint>

extern "C" void handle_panic(const char* msg, size_t msg_len, const char* file, size_t file_len, uint32_t line_num);
extern "C" void print_rust_str(const char *str, size_t str_len);
extern "C" float my_sqrt(float num);
extern "C" float my_atan2(float y, float x);

#endif
