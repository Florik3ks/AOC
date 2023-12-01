#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "main.h"

enum
{
    MAXLINES = 5000
};

int main(void)
{
    int lineCount = 0;
    int i = 0;
    char *lines[MAXLINES];
    FILE *fp = fopen("input.txt", "r");

    if (fp == 0)
    {
        fprintf(stderr, "failed to open input.txt\n");
        exit(1);
    }

    char line[256];
    while (fgets(line, sizeof(line), fp) != NULL)
    {
        lineCount++;
        lines[i] = malloc(strlen(line));
        strcpy(lines[i], line);
        i++;
    }
    fclose(fp);

    printf("task one: %d\n", taskone(lines, lineCount));
    printf("task two: %d\n", tasktwo(lines, lineCount));
}

int taskone(char *lines[], int linecount)
{
    int sum = 0;
    for (int i = 0; i < linecount; i++)
    {
        int first = -1;
        int last = -1;
        for (int j = 0; j < strlen(lines[i]); j++)
        {
            int result = parseDecimalChar(lines[i][j]);
            if (result != -1)
            {
                if (first == -1)
                {
                    first = result;
                }
                last = result;
            }
        }
        int temp = last + (first * 10);
        sum += temp;
    }
    return sum;
}

int parseDecimalChar(char c)
{
    if (c >= '0' && c <= '9')
    {
        return (int)c - '0';
    }
    return -1;
}

int tasktwo(char *lines[], int linecount)
{
    char *numbers[10] = {"", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};

    int sum = 0;
    for (int i = 0; i < linecount; i++)
    {
        int first = -1;
        int last = -1;
        for (int j = 0; j < strlen(lines[i]); j++)
        {
            int result = parseDecimalChar(lines[i][j]);
            if (result != -1)
            {
                if (first == -1)
                {
                    first = result;
                }
                last = result;
            }
            else
            {
                for (int n = 1; n < 10; n++)
                {
                    if (matchesString(lines[i], j, numbers[n]))
                    {
                        if (first == -1)
                        {
                            first = n;
                        }
                        last = n;
                        break;
                    }
                }
            }
        }
        int temp = last + (first * 10);
        sum += temp;
    }
    return sum;
}

int matchesString(char *line, int startIndex, char *string)
{
    for (int i = 0; i < strlen(string); i++)
    {
        if(i + startIndex > strlen(line)){
            return 0;
        }
        if (line[i + startIndex] != string[i])
        {
            return 0;
        }
    }
    return 1;
}
