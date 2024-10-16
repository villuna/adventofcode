#include <stdio.h>
#include <aoc.h>
#include <stdlib.h>

int solve_captcha(char *line, ssize_t len, int distance) {
    int total = 0;

    for (int i = 0; i < len; i++) {
        char c = line[i]; 
        if (c < '0' || c > '9') {
            // All characters should be numeric, otherwise the calculation will go awry
            return -1;
        }

        int other = (i + distance) % len;

        if (c == line[other]) {
            total += (int)(c - '0');
        }
    }

    return total;
}

int main() {
    aoc_setup();

    char *line;
    size_t cap;
    ssize_t len = getline(&line, &cap, stdin);

    if (len < 0) {
        fprintf(stderr, "Error: Couldn't read line from input\n"); 
        return -1;
    }

    if (line[len - 1] == '\n') {
        line[len - 1] = '\0';
        len--;
    }
    
    printf("Part 1: %d\n", solve_captcha(line, len, 1));
    printf("Part 2: %d\n", solve_captcha(line, len, len / 2));
    free(line);

    return 0;
}
