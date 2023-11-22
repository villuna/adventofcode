#include "days.hpp"
#include "util.hpp"
#include <cmath>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <fstream>
#include <numeric>
#include <sstream>
#include <stdexcept>
#include <unordered_set>

#define PART1_ITER_LIMIT 1000
#define PART2_ITER_LIMIT 1000000

// There's no reason for this to be a template other than im learning how to use them
template<typename T>
struct vector3 {
    T x;
    T y;
    T z;
    
    vector3<T>() {}

    vector3<T>(T x) {
        this->x = x;
        this->y = x;
        this->z = x;
    }

    vector3<T>(T x, T y, T z) {
        this->x = x;
        this->y = y;
        this->z = z;
    }
};

template<typename T>
bool operator==(const vector3<T>& lhs, const vector3<T>& rhs) {
    return lhs.x == rhs.x && lhs.y == rhs.y && lhs.z == rhs.z;
}

template<typename T>
vector3<T> operator+(const vector3<T>& lhs, const vector3<T>& rhs) {
    return vector3(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z);
}

template<typename T>
vector3<T> operator-(const vector3<T>& lhs, const vector3<T>& rhs) {
    return vector3(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z);
}

template<typename T>
vector3<T> operator*(const T& lhs, const vector3<T>& rhs) {
    return vector3(lhs * rhs.x, lhs * rhs.y, lhs * rhs.z);
}

template<typename T>
std::ostream& operator<<(std::ostream& os, const vector3<T>& vec) {
    os << "<x=" << vec.x << ", y=" << vec.y << ", z=" << vec.z << ">";
    return os;
}

template<typename T>
std::istream& operator>>(std::istream& is, vector3<T>& vec) {
    char buf[5] = {'\0'};
    vector3<int> out(0);
    
    is.get(buf, 4);

    if (std::strcmp(buf, "<x=")) {
        is.setstate(std::ios::failbit);
        return is;
    }

    is >> out.x;

    is.get(buf, 5);

    if (std::strcmp(buf, ", y=")) {
        is.setstate(std::ios::failbit);
        return is;
    }

    is >> out.y;

    is.get(buf, 5);

    if (std::strcmp(buf, ", z=")) {
        is.setstate(std::ios::failbit);
        return is;
    }

    is >> out.z;

    if (is.get() != '>') {
        is.setstate(std::ios::failbit);
        return is;
    }

    vec = out;
    return is;
}

struct moon {
    vector3<int> position = vector3<int>(0);
    vector3<int> velocity = vector3<int>(0);

    int total_energy() {
        return (std::abs(this->position.x) + std::abs(this->position.y) + std::abs(this->position.z))
             * (std::abs(this->velocity.x) + std::abs(this->velocity.y) + std::abs(this->velocity.z));
    }
};

bool operator==(const moon& lhs, const moon& rhs) {
    return lhs.position == rhs.position && lhs.velocity == rhs.velocity;
}

std::ostream& operator<<(std::ostream& os, const moon& moon) {
    os << "pos=" << moon.position << ", vel=" << moon.velocity;
    return os;
}

std::istream& operator>>(std::istream& is, moon& dest) {
    is >> dest.position;
    return is;
}

void part1(moon moons[4]) {
    for (long iter = 0; iter < PART1_ITER_LIMIT; iter++) {
        for (int i = 1; i < 4; i++) {
            for (int j = 0; j < i; j++) {
                if (moons[i].position.x < moons[j].position.x) {
                    moons[i].velocity.x += 1;
                    moons[j].velocity.x -= 1;
                } else if (moons[j].position.x < moons[i].position.x) {
                    moons[j].velocity.x += 1;
                    moons[i].velocity.x -= 1;
                }

                if (moons[i].position.y < moons[j].position.y) {
                    moons[i].velocity.y += 1;
                    moons[j].velocity.y -= 1;
                } else if (moons[j].position.y < moons[i].position.y) {
                    moons[j].velocity.y += 1;
                    moons[i].velocity.y -= 1;
                }

                if (moons[i].position.z < moons[j].position.z) {
                    moons[i].velocity.z += 1;
                    moons[j].velocity.z -= 1;
                } else if (moons[j].position.z < moons[i].position.z) {
                    moons[j].velocity.z += 1;
                    moons[i].velocity.z -= 1;
                }
            }
        }

        for (int i = 0; i < 4; i++) {
            moons[i].position = moons[i].position + moons[i].velocity;
        }
    }

    int energy = 0;

    for (int i = 0; i < 4; i++) {
        energy += moons[i].total_energy();
    }

    std::cout << "part 1: " << energy << std::endl;
}

struct timeslice {
    moon moons[4];

    timeslice(moon moons[4]) {
        std::memcpy(this->moons, moons, sizeof(moon) * 4);
    }
};

std::ostream& operator<<(std::ostream& os, const timeslice& ts) {
    for (int i = 0; i < 4; i++) {
        os << ts.moons[i] << std::endl;
    }
    return os;
}

bool operator==(const timeslice& lhs, const timeslice& rhs) {
    bool out = true;

    for (int i = 0; i < 4; i++)
        out &= lhs.moons[i] == rhs.moons[i];

    return out;
}

namespace std {
    template<>
    struct hash<timeslice> {
        std::uint64_t operator()(const timeslice& t) const {
            std::hash<std::string> s;
            std::ostringstream out("");

            for (int i = 0; i < 4; i++) {
                out << t.moons[0];
            }

            return s(out.str());
        }
    };
}

void part2(moon moons[4]) {
    std::unordered_set<timeslice> timeslices;
    timeslices.insert(timeslice(moons));
    std::cout << "part2: ";

    int iteration = 0;
    while (true) {
        iteration++;
        for (int i = 1; i < 4; i++) {
            for (int j = 0; j < i; j++) {
                if (moons[i].position.x < moons[j].position.x) {
                    moons[i].velocity.x += 1;
                    moons[j].velocity.x -= 1;
                } else if (moons[j].position.x < moons[i].position.x) {
                    moons[j].velocity.x += 1;
                    moons[i].velocity.x -= 1;
                }
            }
        }

        for (int i = 0; i < 4; i++) {
            moons[i].position.x = moons[i].position.x + moons[i].velocity.x;
        }

        auto insertion = timeslices.insert(timeslice(moons));

        if (!insertion.second) {
            break;
        }
    }

    long x_count = iteration;
    std::cout << "x=" << iteration << ", ";

    iteration = 0;
    while (true) {
        iteration++;
        for (int i = 1; i < 4; i++) {
            for (int j = 0; j < i; j++) {
                if (moons[i].position.y < moons[j].position.y) {
                    moons[i].velocity.y += 1;
                    moons[j].velocity.y -= 1;
                } else if (moons[j].position.y < moons[i].position.y) {
                    moons[j].velocity.y += 1;
                    moons[i].velocity.y -= 1;
                }
            }
        }

        for (int i = 0; i < 4; i++) {
            moons[i].position.y = moons[i].position.y + moons[i].velocity.y;
        }

        auto insertion = timeslices.insert(timeslice(moons));

        if (!insertion.second) {
            break;
        }
    }


    long y_count = iteration;
    std::cout << "y=" << iteration << ", ";

    iteration = 0;
    while (true) {
        iteration++;
        for (int i = 1; i < 4; i++) {
            for (int j = 0; j < i; j++) {
                if (moons[i].position.z < moons[j].position.z) {
                    moons[i].velocity.z += 1;
                    moons[j].velocity.z -= 1;
                } else if (moons[j].position.z < moons[i].position.z) {
                    moons[j].velocity.z += 1;
                    moons[i].velocity.z -= 1;
                }
            }
        }

        for (int i = 0; i < 4; i++) {
            moons[i].position.z = moons[i].position.z + moons[i].velocity.z;
        }

        auto insertion = timeslices.insert(timeslice(moons));

        if (!insertion.second) {
            break;
        }
    }

    long z_count = iteration;
    std::cout << "z=" << iteration << std::endl;

    std::cout << "part 2: " << std::lcm(std::lcm(x_count, y_count), z_count) << std::endl;
}

void day12() {
    moon moons[4];

    std::ifstream ifs;
    ifs.open("../input/day12.txt");

    for (int i = 0; i < 4; i++) {
        ifs >> moons[i];

        if (ifs.fail())
            throw std::runtime_error("file format bad");

        ifs.ignore(1);
    }

    part2(moons);
}
