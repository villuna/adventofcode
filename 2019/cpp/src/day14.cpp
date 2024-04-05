#include "days.hpp"
#include "util.hpp"
#include <cassert>
#include <cmath>
#include <cstdint>
#include <iostream>
#include <list>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

struct edge {
    std::string ingredient;
    int needed;
};

struct node {
    int produced_by_reaction = 0;
    std::vector<edge> edges;
};

std::pair<std::string, int> read_ingredient(const std::string& input) {
    auto split = split_string(input, " ");
    int num = std::stoi(split[0]);

    return std::pair(split[1], num);
}

// The graph of reactions.
// From inspecting the input, this is going to be a directed acyclic graph.
// Every node in the graph is connected, and we have exactly one starting point (ore),
// and fuel cannot be made into anything else.
struct reaction_graph {
    reaction_graph(std::vector<std::string> input) {
        for (std::string line : input) {
            auto splits = split_string(line, " => ");
            std::vector<std::string> lhs = split_string(splits[0], ", ");
            auto rhs = read_ingredient(splits[1]);
            
            auto n = &graph.insert(std::pair(rhs.first, node())).first->second;
            n->produced_by_reaction = rhs.second;

            for (std::string ingredient : lhs) {
                auto parsed = read_ingredient(ingredient);
                n->edges.push_back({ .ingredient = parsed.first, .needed = parsed.second });
            }
        }
    }

    // Returns a list of all the ingredients in the list sorted topologically.
    //
    // Topological here means that an ingredient will always be listed before anything it is created out of.
    // If we have a graph such as
    //
    // 1 ORE => 1 A
    // 1 ORE, 1 A => 1 B
    // 1 ORE, 1 A => 1 C
    // 1 B, 1 C => 1 FUEL
    //
    // Then the possible topological orderings are:
    // - FUEL, B, C, A, ORE
    // - FUEL, C, B, A, ORE
    //
    // Because of how the input is structured, the first element will always be FUEL and the last ORE
    std::list<std::string> topological_sort() {
        std::unordered_map<std::string, mark> marks;
        std::list<std::string> res;

        h_visit("FUEL", marks, res);

        return res;
    }

    node& operator[](const std::string& key) {
        return graph[key];
    }

private:
    std::unordered_map<std::string, node> graph;

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
            h_visit(edge.ingredient, map, result);
        }

        map[node] = MARK_PERMANENT;
        result.push_front(node);
    }
};

// Find the next ingredient (of the specified ordering) in the ingredients map, and
// remove and return it.
// errors if no ingredients can be found
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
    // We go through the ingredients in topological order, and break them down into their
    // constituent components. By using topological sort, we know that we only need to break
    // down each ingredient once.
    std::list<std::string> order = graph.topological_sort();
    std::unordered_map<std::string, int> leftovers = {};
    int64_t max_ore = 1000000000000;
    int64_t ore = 1000000000000;
    int made = 0;
    float target = 1;

    // We keep producing fuel until it uses up more ore than we have
    // then we know that's one more fuel than we can produce with 1 trillion ore
    while (ore > 0) {
        std::unordered_map<std::string, int> ingredients = { {"FUEL", 1} };

        // We traverse through the ingredient graph until we only have ore left
        // breaking down each ingredient into its components
        // Since it is topologically ordered, the next ingredient in the ordering is always
        // exactly as many as we need to make
        while (!(ingredients.size() == 1 && ingredients.find("ORE") != ingredients.end())) {
            std::pair<std::string, int> next_ingredient;

            try {
                next_ingredient = extract_next_ingredient(ingredients, order);
            } catch (std::runtime_error e) {
                break;
            }

            std::string name = next_ingredient.first;
            int needed = next_ingredient.second;

            // If we have some in the store, we'll use as much of it as we can
            if (auto stored = leftovers.find(name); stored != leftovers.end()) {
                assert(stored->second > 0);
                needed -= stored->second;
                leftovers.erase(stored);
            }

            float reactions_needed_f = (float)needed / (float)graph[name].produced_by_reaction;
            int reactions_needed = (int)std::ceil(reactions_needed_f);

            // Break down the ingredient into its components
            for (edge component : graph[next_ingredient.first].edges) {
                ingredients.insert({component.ingredient, 0}).first->second += component.needed * reactions_needed;
            }

            int produced = reactions_needed * graph[name].produced_by_reaction;

            // If we are left over with more ingredients than we strictly need, we put them in leftovers
            if (produced > needed) {
                leftovers.insert({ name, 0 }).first->second += produced - needed;
            }
        }

        int ore_used = 0;

        if (auto ore = ingredients.find("ORE"); ore != ingredients.end()) {
            ore_used = ore->second;
        }

        // If this is the first fuel produced, we can print out how much ore it took and this gives
        // us how much it takes to produce 1 fuel. That's part 1, so yay!
        if (made == 0) {
            std::cout << "Part 1: " << ore_used << std::endl;
        }

        // Since this takes a long time, I'm printing a progress bar to monitor it
        if ((float)ore / (float)max_ore <= target) {
            std::cout << "Calculating part 2: " << std::ceil((1 - target) * 100) << "% done (" << "ore = " << ore << ")" << std::endl;
            target -= 0.01;
        }

        ore -= ore_used;
        made += 1;
    }

    std::cout << "Part 2: " << made - 1 << std::endl;
}
