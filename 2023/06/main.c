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

    printf("task one: %d\n", taskone(lines, lineCount));
    printf("task two: %d\n", tasktwo(lines, lineCount));
}

uint64_t taskone(char *lines[], int linecount)
{
    int numLen;
    int length = 0;
    // get array length
    for (int i = 0; i < strlen(lines[0]);)
    {
        uint64_t num = getNumberByIndexUint_t64(lines[0], &i, &numLen);
        if (num != 0)
        {
            length++;
        }
    }

    // create arrays
    uint32_t *times = (uint32_t *)malloc(length * sizeof(uint32_t));
    uint32_t *distances = (uint32_t *)malloc(length * sizeof(uint32_t));

    int index = 0;
    for (int i = 0; i < strlen(lines[0]);)
    {
        uint64_t num = getNumberByIndexUint_t64(lines[0], &i, &numLen);
        if (num != 0)
        {
            times[index] = num;
            index++;
        }
    }

    index = 0;
    for (int i = 0; i < strlen(lines[1]);)
    {
        uint64_t num = getNumberByIndexUint_t64(lines[1], &i, &numLen);
        if (num != 0)
        {
            distances[index] = num;
            index++;
        }
    }

    // compute stuff
    uint64_t result = 1;
    for (int race = 0; race < length; race++)
    {
        int waysToWin = 0;
        uint32_t time = times[race];
        uint32_t dist = distances[race];
        for (int i = 0; i < time; i++)
        {
            if ((time - i) * i > dist)
            {
                // printf("\t race %d is winnable with %d\n", race, i);
                waysToWin++;
            }
        }
        // printf("race %d has %d ways to win\n", race, waysToWin);
        result *= waysToWin;
    }

    free(times);
    free(distances);
    return result;
}

uint64_t tasktwo(char *lines[], int linecount)
{
    uint64_t time = 0;
    uint64_t dist = 0;

    int numLen;
    for (int c = 0; c < strlen(lines[0]); c++)
    {
        int num = parseDecimalChar(lines[0][c]);
        if (num != -1)
        {
            time *= 10;
            time += num;
        }
    }

    for (int c = 0; c < strlen(lines[1]); c++)
    {
        int num = parseDecimalChar(lines[1][c]);
        if (num != -1)
        {
            dist *= 10;
            dist += num;
        }
    }


    // compute stuff
    uint64_t result = 1;
    int waysToWin = 0;
    for (int i = 0; i < time; i++)
    {
        if ((time - i) * i > dist)
        {
            waysToWin++;
        }
    }
    return waysToWin;
}
