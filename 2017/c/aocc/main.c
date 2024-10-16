#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <err.h>
#include <fcntl.h>

int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "Usage: aocc [day]\n");
        return -1;
    }

    char *name_buffer;
    if (asprintf(&name_buffer, "build/days/day%s", argv[1]) < 0) {
        err(-1, "Could not create string buffer");
    }

    char *filename_buffer;
    if (asprintf(&filename_buffer, "../input/day%s.txt", argv[1]) < 0) {
        err(-1, "Could not create string buffer");
    }

    int input_file = open(filename_buffer, O_RDONLY);

    if (input_file < 0) {
        err(-1, "Could not open input file, which should be at \"%s\"",
                filename_buffer);
        return -1;
    }

    // This runner will create a parent and child process. The child process will be the solution,
    // and the parent process will run the child and gather data about how it runs.
    // The child will read its stdin from the input file, and its stdout will be redirected
    // to a parent process pipe.
    //
    // This way, each day's solution can be a single child process, and the parent can be in
    // charge of timing and argument handling and etc. (Timing may be influenced by the time it
    // takes to send data through unix pipes, which is a bit of a problem, but exactly *how much*
    // of a problem it ends up being, we shall see).
    //
    // The real reason I'm doing this, of course, is that unix pipes and fork/exec are cool
    int fds[2];
    if (pipe(fds) < 0) {
        err(-1, "Could not open pipe");
    }

    pid_t child;
    if ((child = fork())) {
        // Parent process.
        close(fds[1]);
        FILE *pipe = fdopen(fds[0], "r");

        if (!pipe) {
            kill(child, SIGTERM);
            err(-1, "Could not open pipe to executable!");
        }

        char* line = NULL;
        size_t cap;
        ssize_t len;

        while ((len = getline(&line, &cap, pipe)) >= 0) {
            printf("> %s", line);

            free(line);
            line = NULL;
        }
    } else {
        dup2(fds[1], STDOUT_FILENO);
        close(fds[0]);
        dup2(input_file, STDIN_FILENO);

        // Child
        if (execl(name_buffer, name_buffer, NULL) < 1) {
            err(-1, "Could not run executable \"%s\""
                    "(Maybe this day was not completed?)",
                    name_buffer);
            return -1;
        }
    }
}
