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

    printf("task one: %lld\n", taskone(lines, lineCount));
    printf("task two: %llu\n", tasktwo(lines, lineCount));
}

int64_t taskone(char *lines[], int linecount)
{
    int64_t sum = 0;
    for (int i = 0; i < linecount; i++)
    {
        int c = 0;
        int count = 0;
        char *line = lines[i];
        int numLen;
        int num;
        while (c < strlen(line))
        {
            num = getNumberByIndex(line, &c, &numLen);
            if (numLen > 0)
            {
                count++;
            }
        }

        int *numbers = (int *)malloc(count * sizeof(int));
        c = 0;
        int index = 0;
        while (c < strlen(line))
        {
            num = getNumberByIndex(line, &c, &numLen);
            if (numLen > 0)
            {
                numbers[index] = num;
                index++;
            }
        }
        sum += getNextNumber(numbers, count) + numbers[count - 1];
    }
    return sum;
}

int getNextNumber(int *array, int count)
{
    int *newArray = (int *)malloc((count - 1) * sizeof(int));
    int allzero = 1;
    for (int i = count - 1; i > 0; i--)
    {
        int new = array[i] - array[i - 1];
        newArray[i - 1] = new;
        if (new != 0)
        {
            allzero = 0;
        }
    }
    if (allzero)
    {
        return 0;
    }
    return getNextNumber(newArray, count - 1) + newArray[count - 2];
}

uint64_t tasktwo(char *lines[], int linecount)
{
    int64_t sum = 0;
    for (int i = 0; i < linecount; i++)
    {
        int c = 0;
        int count = 0;
        char *line = lines[i];
        int numLen;
        int num;
        while (c < strlen(line))
        {
            num = getNumberByIndex(line, &c, &numLen);
            if (numLen > 0)
            {
                count++;
            }
        }

        int *numbers = (int *)malloc(count * sizeof(int));
        c = 0;
        int index = 0;
        while (c < strlen(line))
        {
            num = getNumberByIndex(line, &c, &numLen);
            if (numLen > 0)
            {
                numbers[index] = num;
                index++;
            }
        }
        sum += numbers[0] - getNextNumberP2(numbers, count);
    }
    return sum;
}

int getNextNumberP2(int *array, int count)
{
    int *newArray = (int *)malloc((count - 1) * sizeof(int));
    int allzero = 1;
    for (int i = count - 1; i > 0; i--)
    {
        int new = array[i] - array[i - 1];
        newArray[i - 1] = new;
        if (new != 0)
        {
            allzero = 0;
        }
    }
    if (allzero)
    {
        return 0;
    }
    return newArray[0] - getNextNumberP2(newArray, count - 1);
}
