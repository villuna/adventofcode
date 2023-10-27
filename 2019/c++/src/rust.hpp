#ifndef RUST_HPP
#define RUST_HPP

#include <cstddef>
#include <cstdint>

extern "C" void handle_panic(const char* msg, size_t msg_len, const char* file, size_t file_len, uint32_t line_num);

#endif
