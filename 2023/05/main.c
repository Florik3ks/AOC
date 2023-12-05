#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../util/readinput.h"
#include "../util/utility.h"

#include "main.h"

const int SEED_LENGTH = 20;
const int SEED_LENGTH_2 = 2000;

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

int taskone(char *lines[], int linecount)
{
    int numLen;
    unsigned long int seeds[SEED_LENGTH];
    memset(seeds, 0, SEED_LENGTH * sizeof(unsigned long int));

    int index = 0;
    for (int i = 0; i < strlen(lines[0]);)
    {
        int num = getNumberByIndex(lines[0], &i, &numLen);
        if (num != 0)
        {
            seeds[index] = num;
            index++;
        }
    }

    char *map[50] = {""};
    int mapIndex = 0;
    for (int i = 2; i < linecount; i++)
    {
        char first = lines[i][0];
        char last;
        if (strlen(lines[i]) > 1)
        {
            last = lines[i][strlen(lines[i]) - 2]; // last is always \n\0
        }

        if (first == '\n')
        {
            convert(map, mapIndex, seeds);
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
    convert(map, mapIndex, seeds);

    unsigned long int result = 2147483647L;
    for (int i = 0; i < SEED_LENGTH; i++)
    {
        if (seeds[i] == 0)
        {
            continue;
        }
        if(seeds[i] < result){
            result = seeds[i];
        }
    }
    return result;
}

int convert(char *map[], int maplines, unsigned long int *seeds)
{
    int numLen;
    unsigned long int newSeeds[SEED_LENGTH];
    memset(newSeeds, 0, SEED_LENGTH * sizeof(unsigned long int));
    for (int i = 0; i < maplines; i++)
    {
        int c = 0;
        int destination = getNumberByIndex(map[i], &c, &numLen);
        int source = getNumberByIndex(map[i], &c, &numLen);
        int range = getNumberByIndex(map[i], &c, &numLen);

        for (int j = 0; j < SEED_LENGTH; j++)
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

    for (int j = 0; j < SEED_LENGTH; j++)
    {
        if (seeds[j] == 0)
        {
            seeds[j] = newSeeds[j];
        }
    }
}

int tasktwo(char *lines[], int linecount)
{
    int numLen;
    int index = 0;
    long long pain = 0;
    for (int i = 7; i < strlen(lines[0]);)
    {
        int num1 = getNumberByIndex(lines[0], &i, &numLen);
        int num2 = getNumberByIndex(lines[0], &i, &numLen);
        pain += num2;
    }






}
