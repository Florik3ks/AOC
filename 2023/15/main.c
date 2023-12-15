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
    readinputCustomLinelength(lines, &lineCount, argv[1], 1048576);

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
    int sum = 0;
    char current[256];
    current[0] = '\0';
    
    for (int c = 0; c < strlen(lines[0]); c++)
    {
        if(lines[0][c] == ','){
            sum += hash(current);
            current[0] = '\0';
        }
        else if(lines[0][c] != '\n'){
            char new[2] = {lines[0][c], '\0'};
            strncat(current, new, 2);
        }
    }
    sum += hash(current);
    printf("A \"%s\"\n", current);
    return sum;
}

int hash(char* c){
    int sum = 0;
    for (int i = 0; i < strlen(c); i++){
        sum += c[i];
        sum *= 17;
        sum %= 256;
    }
    return sum;
}

uint64_t tasktwo(char *lines[], int linecount)
{
    return -1;
}
