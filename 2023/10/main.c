#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <inttypes.h>

#include "../util/readinput.h"
#include "../util/utility.h"

#include "main.h"

int main(int argc, char *argv[])
{
    char *lines[5000];
    int lineCount;
    if (argc < 2)
    {
        fprintf(stderr, "no input file\n");
        exit(0);
    }
    printf("%s with %s\n", argv[0], argv[1]);
    readinput(lines, &lineCount, argv[1]);

    printf("task one: %llu\n", taskone(lines, lineCount));
    printf("task two: %llu\n", tasktwo(lines, lineCount));
}

uint64_t taskone(char *lines[], int linecount)
{
    return -1;
}

uint64_t tasktwo(char *lines[], int linecount)
{
    return -1;
}
