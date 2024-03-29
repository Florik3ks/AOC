#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <inttypes.h>

#include "utility.h"

int contains(int a[], int num, int max)
{
    for (int i = 0; i < max; i++)
    {
        if (a[i] == num)
        {
            return 1;
        }
    }
    return 0;
}

int getNumberByIndex(char *string, int *index, int *numLen)
{
    int length = 0;
    int num = 0;
    int isNegative = 0;
    if (string[*index] == '-')
    {
        *index = *index + 1;
        isNegative = 1;
    }
    for (int i = *index; i < strlen(string); i++)
    {
        int result = parseDecimalChar(string[i]);
        *index = *index + 1;
        if (result == -1)
        {
            *numLen = length;
            return num * (isNegative ? -1 : 1);
        }
        length++;
        num *= 10;
        num += result;
    }
    *numLen = length;
    return num * (isNegative ? -1 : 1);
}

uint64_t getNumberByIndexUint_t64(char *string, int *index, int *numLen)
{
    int length = 0;
    uint64_t num = 0;
    for (int i = *index; i < strlen(string); i++)
    {
        long result = parseDecimalChar(string[i]);
        *index = *index + 1;
        if (result == -1)
        {
            *numLen = length;
            return num;
        }
        length++;
        num *= 10;
        num += result;
    }
    *numLen = length;
    return num;
}

int parseDecimalChar(char c)
{
    if (c >= '0' && c <= '9')
    {
        return (int)c - '0';
    }
    return -1;
}