#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../readinput.h"
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

int taskone(char *lines[], int linecount)
{
    int sum = 0;
    for (int i = 0; i < linecount; i++)
    {
        for (int j = 0; j < strlen(lines[i]); j++)
        {
            int numLen;
            int numStart = j;
            int num = getNumberByIndex(lines[i], &j, &numLen);
            if (num > 0)
            {
                if (num == 161)
                {
                    int abc = 3;
                }
                int isPartNumber = 0;
                for (int n = numStart; n < numStart + numLen; n++)
                {
                    if (searchSurroundingForSymbol(lines, linecount, i, n))
                    {
                        isPartNumber = 1;
                        break;
                    }
                }
                if (isPartNumber)
                {
                    sum += num;
                }
            }
        }
    }
    return sum;
}

int searchSurroundingForSymbol(char *lines[], int lineCount, int i, int j)
{
    for (int x = -1; x <= 1; x++)
    {
        for (int y = -1; y <= 1; y++)
        {
            if (
                (i + y >= 0) &&
                (i + y < lineCount) &&
                (j + x >= 0) &&
                (j + x < strlen(lines[i + y])))
            {
                char c = lines[i + y][j + x];
                if (parseDecimalChar(c) == -1 && c != '.' && c != '\n' && c != '\0')
                {
                    return 1;
                }
            }
        }
    }
    return 0;
}

int tasktwo(char *lines[], int linecount)
{
    long int sum = 0;
    for (int i = 0; i < linecount; i++)
    {
        for (int j = 0; j < strlen(lines[i]); j++)
        {
            char c = lines[i][j];
            if (parseDecimalChar(c) == -1 && c != '.' && c != '\n' && c != '\0')
            {
                sum += searchSurroundingForNumbers(lines, linecount, i, j);
            }
        }
    }
    return sum;
}

int searchSurroundingForNumbers(char *lines[], int lineCount, int i, int j)
{
    int numbers[10] = {0};
    int index = 0;
    for (int x = -1; x <= 1; x++)
    {
        for (int y = -1; y <= 1; y++)
        {
            if (
                (i + y >= 0) &&
                (i + y < lineCount) &&
                (j + x >= 0) &&
                (j + x < strlen(lines[i + y])))
            {
                char c = lines[i + y][j + x];
                if (parseDecimalChar(c) != -1)
                {
                    // go to first digit of number
                    // (turing machine flashback)
                    int col = j + x;
                    while (parseDecimalChar(lines[i + y][col]) != -1 && col >= 0)
                    {
                        col--;
                    }
                    col++;
                    int ptr;
                    int num = getNumberByIndex(lines[i + y], &col, &ptr);
                    if (num != 0 && !contains(numbers, num, 10))
                    {
                        numbers[index] = num;
                        index++;
                    }
                }
            }
        }
    }
    if (index == 2)
    {
        return numbers[0] * numbers[1];
    }
    return 0;
}

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
    for (int i = *index; i < strlen(string); i++)
    {
        int result = parseDecimalChar(string[i]);
        if (result == -1)
        {
            *numLen = length;
            return num;
        }
        *index = *index + 1;
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
