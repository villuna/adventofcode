#include "days.hpp"
#include "util.hpp"
#include <iostream>
#include <set>
#include <unordered_map>
#include <queue>

struct orbit {
    std::string orbited;
    std::string satellite;
};

void part1(std::vector<orbit>& input) {
    std::unordered_map<std::string, std::vector<std::string>> orbits;

    for (orbit o : input) {
        std::cout << o.orbited << " ) " << o.satellite << std::endl;

        if (auto satellites = orbits.find(o.orbited); satellites != orbits.end()) {
            satellites->second.push_back(o.satellite);
        } else {
            std::vector<std::string> satellites_new;
            satellites_new.push_back(o.satellite);
            orbits[o.orbited] = satellites_new;
        }
    }

    std::unordered_map<std::string, int> levels;
    levels["COM"] = 0;

    std::queue<std::string> frontier;
    frontier.push("COM");

    int num_orbits = 0;

    while (!frontier.empty()) {
        std::string next = frontier.front();
        int level = levels[next];
        num_orbits += level;

        std::vector<std::string> satellites = orbits[next];

        for (std::string s : satellites) {
            levels[s] = level + 1;
            frontier.push(s);
        }
        
        frontier.pop();
    }

    std::cout << "total number of orbits is " << num_orbits << std::endl;
}

void part2(std::vector<orbit>& input) {
    std::string start;
    std::string dest;

    std::unordered_map<std::string, std::vector<std::string>> graph;

    for (orbit o : input) {
        if (o.satellite == "YOU") {
            start = o.orbited; 
        }

        if (o.satellite == "SAN") {
            dest = o.orbited;
        }

        if (auto connections = graph.find(o.orbited); connections  != graph.end()) {
            connections->second.push_back(o.satellite);
        } else {
            std::vector<std::string> connections_new;
            connections_new.push_back(o.satellite);
            graph[o.orbited] = connections_new;
        }

        if (auto connections = graph.find(o.satellite); connections  != graph.end()) {
            connections->second.push_back(o.orbited);
        } else {
            std::vector<std::string> connections_new;
            connections_new.push_back(o.orbited);
            graph[o.satellite] = connections_new;
        }
    }

    std::set<std::string> visited;
    std::queue<std::pair<std::string, int>> frontier;
    frontier.push(std::pair(start, 0));

    while (!frontier.empty()) {
        auto pair = frontier.front();
        std::string next = pair.first;
        int dist = pair.second;

        visited.insert(next);

        for (std::string body : graph[next]) {
            if (body == dest) {
                std::cout << "distance is " << dist + 1 << std::endl;
                return;
            }

            // In C++ (before C++20) we don't say set.contains(value),
            // we say set.find(value) != set.end()
            // and i think thats horrendous
            if (visited.find(body) == visited.end()) {
                frontier.push(std::pair(body, dist + 1));
            }
        }

        frontier.pop();
    }
}

void day6() {
    std::vector<std::string> lines = read_input_lines(6);

    std::vector<orbit> inputs;

    for (std::string line : lines) {
        std::vector<std::string> pair = split_string(line, ")");

        orbit o;
        o.orbited = pair[0];
        o.satellite = pair[1];
        inputs.push_back(o);
    }

    part1(inputs);
    part2(inputs);
}
