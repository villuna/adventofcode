// Writing this intcode computer has been fun but it's also given me a real insight into how tech
// debt hurts development. Writing a class with ever changing requirements while needing to make
// sure that the existing interface/functionality doesn't change means I've had to make some
// strange design decisions.
//
// I feel okay leaving them here because it's just advent of code but it's kind of funny
#include <cstdint>
#include <vector>
#include <string>
#include <unordered_map>

enum run_mode {
    RETURN_ON_HALT,
    RETURN_ON_OUTPUT,
};

enum input_mode {
    DONT_WAIT_FOR_INPUT,
    WAIT_FOR_INPUT,
};

enum result_type {
    TYPE_HALTED,
    TYPE_OUTPUT,
    TYPE_BLOCKING,
};

std::string result_type_desc(result_type type);

// Really wish I had a rust enum rn
struct run_result {
    int64_t code;
    result_type type;
};

class IntcodeComputer {
public:
    IntcodeComputer() { }

    IntcodeComputer(const std::string& input) {
        load_program(input);
    }

    void load_program(const std::string& input);
    void load_input(const std::vector<int64_t>& input);
    void push_input(int64_t input);
    void set_input_at(int64_t index, int64_t value);
    void set_instruction_at(int64_t index, int64_t value);
    void set_run_mode(run_mode mode);
    void set_input_mode(input_mode mode);
    run_result run();

private:
    run_mode r_mode = RETURN_ON_HALT;
    input_mode i_mode = DONT_WAIT_FOR_INPUT;
    int64_t pc;
    int64_t output;
    int64_t input_index = 0;
    int64_t relative_base = 0;
    std::vector<int64_t> ops;
    std::unordered_map<int64_t, int64_t> extra_ops;
    std::vector<int64_t> input;

    int64_t value(std::vector<int64_t>& op_vec, int64_t offset);
    int64_t wr_addr(std::vector<int64_t>& op_vec, int64_t offset);
    int64_t get_op(int64_t index);
    void set_op(int64_t index, int64_t value);
};
