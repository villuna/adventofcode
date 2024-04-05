#include "days.hpp"
#include "util.hpp"
#include "intcode.hpp"
#include <cassert>
#include <iostream>
#include <optional>
#include <queue>
#include <stdexcept>
#include <unordered_map>
#include <unordered_set>

enum command {
    NORTH = 1,
    SOUTH = 2,
    WEST = 3,
    EAST = 4
};

command opposite(command cmd) {
    switch (cmd) {
        case NORTH: return SOUTH;
        case SOUTH: return NORTH;
        case EAST: return WEST;
        case WEST: return EAST;
    }

    throw std::runtime_error("invalid command");
}

coord step(command cmd) {
    switch (cmd) {
        case NORTH:
            return make_coord(0, -1);
        case SOUTH:
            return make_coord(0, 1);
        case WEST:
            return make_coord(-1, 0);
        case EAST:
            return make_coord(1, 0);
    }

    throw std::runtime_error("invalid command");
}

enum tile {
    FREE,
    WALL,
};

enum result_code {
    HIT_WALL = 0,
    MOVED = 1,
    FOUND_OXYGEN = 2,
};

void map_area_h(IntcodeComputer& computer, std::unordered_map<coord, tile>& map, std::optional<coord>& oxygen_system, coord pos) {
    map.insert({pos, FREE});

    for (int i = 1; i <= 4; i++) {
        coord next = pos + step((command)i);

        if (map.find(next) != map.end()) {
            continue;
        }

        computer.push_input(i);
        run_result res = computer.run();

        switch (res.code) {
            case HIT_WALL:
                map.insert({ next, WALL });
                break;

            // This makes me feel dirty but when in rome i guess?
            case FOUND_OXYGEN:
                oxygen_system = next;
            case MOVED:
                map_area_h(computer, map, oxygen_system, next);
                int opp = opposite((command)i);
                computer.push_input(opp);
                res = computer.run();
                assert(res.type == TYPE_OUTPUT);
                assert(res.code == MOVED || res.code == FOUND_OXYGEN);
                break;
        }
    }
}

void map_area(IntcodeComputer& computer, std::unordered_map<coord, tile>& map, std::optional<coord>& oxygen_system) {
    map_area_h(computer, map, oxygen_system, make_coord(0, 0));
}

struct queue_entry {
    int travelled;
    int distance;
    coord position;

    int heuristic() const {
        return travelled + distance;
    }
};

bool operator<(const queue_entry lhs, const queue_entry rhs) {
    return lhs.heuristic() < rhs.heuristic();
}

bool operator==(const queue_entry lhs, const queue_entry rhs) {
    return lhs.heuristic() == rhs.heuristic();
}

int a_star(const std::unordered_map<coord, tile>& map, coord start, coord dest) {
    std::priority_queue<queue_entry> frontier;
    std::unordered_set<coord> visited;
    frontier.push({ 0, (start - dest).norm(), start });

    while (!frontier.empty()) {
        queue_entry current = frontier.top();
        visited.insert(current.position);
        frontier.pop();

        for (int i = 1; i <= 4; i++) {
            coord next = current.position + step((command)i);

            if (next == dest) {
                return current.travelled + 1;
            }

            if (map.find(next)->second == FREE && visited.find(next) == visited.end()) {
                frontier.push({ .travelled = current.travelled + 1, .distance = (next - dest).norm(), .position = next});
            }
        }
    }

    throw std::runtime_error("couldnt find a path");
}

void day15() {
    IntcodeComputer computer(read_input(15)); 
    computer.set_run_mode(RETURN_ON_OUTPUT);
    computer.set_input_mode(WAIT_FOR_INPUT);

    std::unordered_map<coord, tile> map;
    std::optional<coord> m_oxygen_system = {};

    map_area(computer, map, m_oxygen_system);

    if (!m_oxygen_system.has_value()) {
        std::cerr << "Error: couldn't find oxygen system anywhere" << std::endl;
        return;
    }

    coord oxygen_system = *m_oxygen_system;

    int steps = a_star(map, make_coord(0, 0), oxygen_system);
    std::cout << "Part 1: " << steps << std::endl;

    int max_steps = 0;
    for (auto t = map.begin(); t != map.end(); t++) {
        if (t->second == WALL) {
            continue;
        }

        // Kind of an inefficient way of doing things but
        // meh
        // a star is super fast it wont take too long
        int steps = a_star(map, oxygen_system, t->first);
        if (steps > max_steps) {
            max_steps = steps;
        }
    }

    std::cout << "Part 2: " << max_steps << std::endl;
}
