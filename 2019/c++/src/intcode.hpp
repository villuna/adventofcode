#include <cstdint>
#include <vector>
#include <string>
#include <unordered_map>

enum run_mode {
    RETURN_ON_HALT,
    RETURN_ON_OUTPUT,
};

struct run_result {
    int64_t code;
    bool halted;
};

class IntcodeComputer {
public:
    IntcodeComputer() { }

    IntcodeComputer(const std::string& input) {
        load_program(input);
    }

    void load_program(const std::string& input);
    void load_input(const std::vector<int64_t>& input);
    void set_input_at(int64_t index, int64_t value);
    void set_run_mode(run_mode mode);
    run_result run();

private:
    run_mode mode = RETURN_ON_HALT;
    int64_t pc;
    int64_t output;
    int64_t input_index;
    int64_t relative_base = 0;
    std::vector<int64_t> ops;
    std::unordered_map<int64_t, int64_t> extra_ops;
    std::vector<int64_t> input;

    int64_t value(std::vector<int64_t>& op_vec, int64_t offset);
    int64_t wr_addr(std::vector<int64_t>& op_vec, int64_t offset);
    int64_t get_op(int64_t index);
    void set_op(int64_t index, int64_t value);
};
