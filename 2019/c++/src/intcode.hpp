#include <vector>
#include <string>

class IntcodeComputer {
public:
    IntcodeComputer(const std::string& input) {
        load_program(input);
    }

    void load_program(const std::string& input);

    int solve(int input);

private:
    int pc;
    std::vector<int> ops;

    int value(std::vector<int>& op_vec, int offset);
    int wr_addr(std::vector<int>& op_vec, int offset);
};
