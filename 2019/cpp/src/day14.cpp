#include "days.hpp"
#include "util.hpp"
#include <cmath>
#include <iostream>
#include <list>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

struct node {
    int produced_by_reaction = 0;
    std::vector<std::pair<std::string, int>> edges;
};

std::pair<std::string, int> read_ingredient(const std::string& input) {
    auto split = split_string(input, " ");
    int num = std::stoi(split[0]);

    return std::pair(split[1], num);
}

struct reaction_graph {
    std::unordered_map<std::string, node> graph;

    reaction_graph(std::vector<std::string> input) {
        for (std::string line : input) {
            auto splits = split_string(line, " => ");
            std::vector<std::string> lhs = split_string(splits[0], ", ");
            auto rhs = read_ingredient(splits[1]);
            
            graph.insert(std::pair(rhs.first, node()));
            graph[rhs.first].produced_by_reaction = rhs.second;

            for (std::string ingredient : lhs) {
                auto parsed = read_ingredient(ingredient);
                graph[rhs.first].edges.push_back(parsed);
            }
        }
    }

    std::list<std::string> topological_sort() {
        std::unordered_map<std::string, mark> marks;
        std::list<std::string> res;

        h_visit("FUEL", marks, res);

        return res;
    }

private:
    enum mark {
        MARK_TEMPORARY,
        MARK_PERMANENT,
    };

    void h_visit(std::string node, std::unordered_map<std::string, mark>& map, std::list<std::string>& result) {
        if (auto entry = map.find(node); entry != map.end()) {
            if (entry->second == MARK_TEMPORARY)
                throw std::runtime_error("cycle exists in the graph!");
            else if (entry->second == MARK_PERMANENT)
                return;
        }

        map[node] = MARK_TEMPORARY;

        for (auto edge : graph[node].edges) {
            h_visit(edge.first, map, result);
        }

        map[node] = MARK_PERMANENT;
        result.push_front(node);
    }
};

std::pair<std::string, int> extract_next_ingredient(
    std::unordered_map<std::string, int>& ingredients,
    const std::list<std::string>& order
) {
    for (std::string s : order) {
        if (auto ing = ingredients.find(s); ing != ingredients.end()) {
            std::pair<std::string, int> res = *ing;
            ingredients.erase(ing);
            return res;
        }
    }

    throw std::runtime_error("no ingredients found");
}

void day14() {
    reaction_graph graph(read_input_lines(14));
    std::list<std::string> order = graph.topological_sort();
    std::unordered_map<std::string, int> ingredients = { {"FUEL", 1} };

    while (ingredients.size() > 1 || ingredients.find("ORE") == ingredients.end()) {
        for (auto i = ingredients.begin(); i != ingredients.end(); i++) {
            std::cout << i->second << " " << i->first << ",";
        }
        std::cout << std::endl;
        std::pair<std::string, int> next_ingredient = extract_next_ingredient(ingredients, order);

        float num_needed_f = (float)next_ingredient.second / (float)graph.graph[next_ingredient.first].produced_by_reaction;
        int num_needed = (int)std::ceil(num_needed_f);

        for (std::pair<std::string, int> component : graph.graph[next_ingredient.first].edges) {
            ingredients.insert({component.first, 0}).first->second += component.second * num_needed;
        }
    }

    std::cout << "Part 1: " << ingredients["ORE"] << std::endl;
}
