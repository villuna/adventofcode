#include <vector>
#include <string>

enum run_mode {
    RETURN_ON_HALT,
    RETURN_ON_OUTPUT,
};

struct run_result {
    int code;
    bool halted;
};

class IntcodeComputer {
public:
    IntcodeComputer() { }

    IntcodeComputer(const std::string& input) {
        load_program(input);
    }

    void load_program(const std::string& input);
    void load_input(const std::vector<int>& input);
    void set_input_at(int index, int value);
    void set_run_mode(run_mode mode);
    run_result run();

private:
    run_mode mode = RETURN_ON_HALT;
    int pc;
    int output;
    int input_index;
    std::vector<int> ops;
    std::vector<int> input;

    int value(std::vector<int>& op_vec, int offset);
    int wr_addr(std::vector<int>& op_vec, int offset);
};
