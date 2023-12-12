#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <pthread.h>
#include <inttypes.h>
#include <sys/time.h>
#include "../util/readinput.h"
#include "../util/utility.h"

#include "multithreading_test.h"

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

uint64_t taskTwoResult = UINT64_MAX;
struct thread_info
{
    int id;
    uint64_t *seeds;
    uint64_t length;
    char **lines;
    int linecount;
};
int inputlinecount;

uint64_t tasktwo(char *lines[], int linecount)
{
    int numLen;
    uint64_t length = 0;
    pthread_t threads[7];
    int rc;
    int threadID = 1;
    // get needed seed array length
    for (int i = 7; i < strlen(lines[0]);)
    {
        uint64_t num1 = getNumberByIndexUint_t64(lines[0], &i, &numLen);
        uint64_t num2 = getNumberByIndexUint_t64(lines[0], &i, &numLen);
        uint64_t *seeds = (uint64_t *)malloc(num2 * sizeof(uint64_t));
        for (uint64_t j = 0; j < num2; j++)
        {
            seeds[j] = num1 + j;
        }
        struct thread_info *tinfo;
        tinfo = malloc(sizeof(struct thread_info));
        tinfo->id = threadID;
        tinfo->seeds = seeds;
        tinfo->length = num2;
        tinfo->lines = lines;
        tinfo->linecount = linecount;

        printf("creating thread %d with array size %fGB\n", threadID, (num2 * sizeof(uint64_t)) * 1.0E-9);

        rc = pthread_create(&threads[threadID], NULL, taskTwoThread, tinfo);
        threadID++;
        if (rc)
        {
            printf("Unable to create thread, %d", rc);
            exit(-1);
        }
    }

    for (int i = 0; i < threadID; i++)
    {
        pthread_join(i, NULL);
    }

    return taskTwoResult;
}

void *taskTwoThread(void *arg)
{
    // Store the value argument passed to this thread
    struct thread_info *tinfo = arg;
    uint64_t length = tinfo->length;
    uint64_t *seeds = tinfo->seeds;

    char **input = tinfo->lines;
    int inputlinecount = tinfo->linecount;

    int numLen;

    char *map[50] = {""};
    int mapIndex = 0;
    char first;
    for (int i = 2; i < inputlinecount; i++)
    {
        first = input[i][0];
        char last;
        if (strlen(input[i]) > 1)
        {
            last = input[i][strlen(input[i]) - 2]; // last is always \n\0
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
            map[mapIndex] = input[i];
            mapIndex++;
        }
    }
    if (first != '\n')
    {
        convert(map, mapIndex, seeds, length);
    }

    uint64_t min = UINT64_MAX;
    for (int i = 0; i < length; i++)
    {
        if (seeds[i] < min)
        {
            min = seeds[i];
        }
        if (seeds[i] < taskTwoResult)
        {
            taskTwoResult = seeds[i];
        }
    }

    printf("thread %d done with minimum %llu\n", tinfo->id, min);
}
