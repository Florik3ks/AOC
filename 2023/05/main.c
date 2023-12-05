#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <inttypes.h>

#include "../util/readinput.h"
#include "../util/utility.h"

#include "main.h"

const int SEED_LENGTH = 20;

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
    int numLen;
    uint64_t *seeds = (uint64_t *)malloc(SEED_LENGTH * sizeof(uint64_t));

    int index = 0;
    for (int i = 0; i < strlen(lines[0]);)
    {
        uint64_t num = getNumberByIndexUint_t64(lines[0], &i, &numLen);
        if (num != 0)
        {
            seeds[index] = num;
            index++;
        }
    }

    char *map[50] = {""};
    int mapIndex = 0;
    char first;
    for (int i = 2; i < linecount; i++)
    {
        first = lines[i][0];
        char last;
        if (strlen(lines[i]) > 1)
        {
            last = lines[i][strlen(lines[i]) - 2]; // last is always \n\0
        }

        if (first == '\n')
        {
            convert(map, mapIndex, seeds, SEED_LENGTH);
        }
        else if (last == ':')
        {
            for (; mapIndex > 0; mapIndex--)
            {
                map[mapIndex] = "";
            }
        }
        else
        {
            map[mapIndex] = lines[i];
            mapIndex++;
        }
    }
    if (first != '\n')
    {
        convert(map, mapIndex, seeds, SEED_LENGTH);
    }

    uint64_t result = UINT64_MAX;
    for (int i = 0; i < SEED_LENGTH; i++)
    {
        if (seeds[i] == 0)
        {
            continue;
        }
        if (seeds[i] < result)
        {
            result = seeds[i];
        }
    }
    return result;
}

int convert(char *map[], int maplines, uint64_t *seeds, uint64_t arrSize)
{
    int numLen;
    uint64_t *newSeeds = (uint64_t *)malloc(arrSize * sizeof(uint64_t));

    for (int i = 0; i < maplines; i++)
    {
        printf("\t map %d / %d\n", i, maplines);
        int c = 0;
        uint64_t destination = getNumberByIndexUint_t64(map[i], &c, &numLen);
        uint64_t source = getNumberByIndexUint_t64(map[i], &c, &numLen);
        uint64_t range = getNumberByIndexUint_t64(map[i], &c, &numLen);

        for (int j = 0; j < arrSize; j++)
        {
            int seed = seeds[j];
            if (seed == 0)
            {
                continue;
            }
            if (seed >= source && seed < source + range)
            {
                newSeeds[j] = seed + destination - source;
                seeds[j] = 0;
            }
        }
    }

    for (uint64_t j = 0; j < arrSize; j++)
    {
        if (seeds[j] == 0)
        {
            seeds[j] = newSeeds[j];
        }
    }
    free(newSeeds);
}

uint64_t tasktwo(char *lines[], int linecount)
{
    int numLen;
    uint64_t length = 0;
    // get needed seed array length
    for (int i = 7; i < strlen(lines[0]);)
    {
        uint64_t num1 = getNumberByIndexUint_t64(lines[0], &i, &numLen);
        uint64_t num2 = getNumberByIndexUint_t64(lines[0], &i, &numLen);
        length += num2;
    }

    uint64_t *seeds = (uint64_t *)malloc(length * sizeof(uint64_t));
    printf("size of array in GB (poor RAM): %f\n", (length * sizeof(uint64_t)) * 1.0E-9);


    int index = 0;
    for (int i = 7; i < strlen(lines[0]);)
    {
        uint64_t num1 = getNumberByIndexUint_t64(lines[0], &i, &numLen);
        uint64_t num2 = getNumberByIndexUint_t64(lines[0], &i, &numLen);
        for (int k = num1; k < num1 + num2; k++)
        {
            seeds[index] = k;
            index++;
        }
    }

    char *map[50] = {""};
    int mapIndex = 0;
    char first;
    for (int i = 2; i < linecount; i++)
    {
        printf("line %d / %d\n", i, linecount);
        first = lines[i][0];
        char last;
        if (strlen(lines[i]) > 1)
        {
            last = lines[i][strlen(lines[i]) - 2]; // last is always \n\0
        }

        if (first == '\n')
        {
            convert(map, mapIndex, seeds, length);
        }
        else if (last == ':')
        {
            for (; mapIndex > 0; mapIndex--)
            {
                map[mapIndex] = "";
            }
        }
        else
        {
            map[mapIndex] = lines[i];
            mapIndex++;
        }
    }
    if (first != '\n')
    {
        convert(map, mapIndex, seeds, length);
    }

    uint64_t result = UINT64_MAX;
    for (uint64_t i = 0; i < length; i++)
    {
        if (seeds[i] == 0)
        {
            continue;
        }
        if (seeds[i] < result)
        {
            result = seeds[i];
        }
    }
    return result;
}
