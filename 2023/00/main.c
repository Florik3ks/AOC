#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <sys/time.h>
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


    struct timeval start, stop;
    double secs = 0;
    long minutes = 0;

    gettimeofday(&start, NULL);
    uint64_t t1 = taskone(lines, lineCount);
    gettimeofday(&stop, NULL);
    secs = (double)(stop.tv_sec - start.tv_sec);
    minutes = (long)(secs / 60);
    secs = (long)secs % 60;
    secs += (double)(stop.tv_usec - start.tv_usec) / 1000000;

    printf("task one: %llu (completed in %dmin %fs)\n", t1, minutes, secs);

    gettimeofday(&start, NULL);
    uint64_t t2 = tasktwo(lines, lineCount);
    gettimeofday(&stop, NULL);
    secs = (double)(stop.tv_sec - start.tv_sec);
    minutes = (long)(secs / 60);
    secs = (long)secs % 60;
    secs += (double)(stop.tv_usec - start.tv_usec) / 1000000;

    printf("task two: %llu (completed in %dmin %fs)\n", t2, minutes, secs);
}

uint64_t taskone(char *lines[], int linecount)
{
    return -1;
}

uint64_t tasktwo(char *lines[], int linecount)
{
    return -1;
}
