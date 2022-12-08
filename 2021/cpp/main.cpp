#include <iostream>
#include <fstream>
#include <string>
#include <bits/stdc++.h>

using namespace std;

void day_two(string filename, int part) {
    if (part != 1 && part != 2) {
        cout << "Invalid part number, silly" << endl;
    }

    ifstream file(filename);
    string line;

    int depth = 0;
    int aim = 0;
    int horizontal = 0;

    while (getline(file, line)) {
        stringstream s(line);
        string command;
        int distance;

        s >> command;
        s >> distance;

        if (command.compare("forward") == 0) {
            horizontal += distance;

            depth += aim * distance;
        } else if (command.compare("up") == 0) {
            if (part == 1) {
                depth -= distance;
            } else {
                aim -= distance;
            }
        } else if (command.compare("down") == 0) {
            if (part == 1) {
                depth += distance;
            } else {
                aim += distance;
            }
        }
    }

    cout << "depth: " << depth << ", distance: " << horizontal << endl;
    cout << "Product: " << depth * horizontal;
}

int main() {
    cout << "Hello (from c++ this time)" << endl;
    cout << "Listen I know you're here to do day 2 so lets get to the point." << endl;
    cout << "Enter the problem: ";

    int problem;
    string filename;

    cin >> problem;

    cout << "Enter the filename: ";

    cin >> filename;

    day_two(filename, problem);

    return 0;
}

