#include <aoc.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <vec.h>

VEC_DEFINE(int);

int main() {
    aoc_setup();

    char *line = NULL;
    size_t count;
    ssize_t len;

    int p1_total = 0;
    int p2_total = 0;

    while ((len = getline(&line, &count, stdin)) >= 0) {
        VEC(int) v = vec_int_new();

        const char *space = " \t\n";
        char *next = strtok(line, space);

        if (next == NULL) {
            continue;
        }

        do {
            char *endptr = NULL;
            int num = strtol(next, &endptr, 10);

            if (*endptr != '\0') {
                fprintf(stderr, "Could not parse as number: \"%s\"\n", next);
                return -1;
            }

            vec_int_push(&v, num);
        } while ((next = strtok(NULL, space)));

        // The vec shouldn't be empty so these calls shouldn't segfault
        int min = *vec_int_at(v, 0);
        int max = *vec_int_at(v, 0);

        for (int i = 1; i < v.len; i++) {
            int n = *vec_int_at(v, i);

            if (n < min) min = n;
            if (n > max) max = n;
        }

        p1_total += max - min;

        int divisor, divided;

        for (int i = 0; i < v.len; i++) {
            for (int j = i + 1; j < v.len; j++) {
                int a = *vec_int_at(v, i), b = *vec_int_at(v, j);

                if (b > a) {
                    int temp = a;
                    a = b;
                    b = temp;
                }

                if (a % b == 0) {
                    divisor = b;
                    divided = a;
                    // Goto considered harmful except when breaking out of nested loops
                    // (and implementing raii at home)
                    goto after_loop;
                }
            }
        }

        // If we got to the end of the loop and didn't find any numbers, then we have a big problem
        fprintf(stderr, "Couldn't find two numbers that divide\n");
        return -1;

after_loop:
        p2_total += divided / divisor;

        vec_int_free(v);
    }

    printf("Part 1: %d\n", p1_total);
    printf("Part 2: %d\n", p2_total);

    free(line);
    return 0;
}
