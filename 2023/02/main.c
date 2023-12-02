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

int getGameID(char *line)
{
    char num[20] = "";
    int p = 0;
    for (int i = 5; line[i] != ':'; i++)
    {
        strncat(num, &line[i], 1);
    }
    char *endptr;
    return strtol(num, &endptr, 10);
}

int taskone(char *lines[], int linecount)
{
    int r = 12;
    int g = 13;
    int b = 14;

    int sum = 0;
    for (int line = 0; line < linecount; line++)
    {
        int id = getGameID(lines[line]);
        int skip = 0;
        int start = 0;
        for (int c = 0; c < strlen(lines[line]) && !skip; c++)
        {
            if (lines[line][c] == ':')
            {
                start = 1;
            }
            if (!start)
            {
                continue;
            }
            while (parseDecimalChar(lines[line][c]) == -1)
            {
                c++;
            }

            int num = getNumberByIndex(lines[line], &c);

            if (matchesString(lines[line], c, "red"))
            {
                if (num > r)
                {
                    skip = 1;
                }
            }
            if (matchesString(lines[line], c, "green"))
            {
                if (num > g)
                {
                    skip = 1;
                }
            }
            if (matchesString(lines[line], c, "blue"))
            {
                if (num > b)
                {
                    skip = 1;
                }
            }
            while (lines[line][c] != ';' && lines[line][c] != ',' && c < strlen(lines[line]))
            {
                c++;
            }
        }
        if (!skip)
        {
            sum += id;
        }
    }

    return sum;
}

int getNumberByIndex(char *string, int *index)
{
    int num = 0;
    for (int i = *index; i < strlen(string); i++)
    {
        *index = *index + 1;
        int result = parseDecimalChar(string[i]);
        if (result == -1)
        {
            return num;
        }
        num *= 10;
        num += result;
    }
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

int tasktwo(char *lines[], int linecount)
{

    int sum = 0;
    for (int line = 0; line < linecount; line++)
    {
        int rmax = 0;
        int gmax = 0;
        int bmax = 0;
        int id = getGameID(lines[line]);
        int start = 0;
        for (int c = 0; c < strlen(lines[line]); c++)
        {
            if (lines[line][c] == ':')
            {
                start = 1;
            }
            if (!start)
            {
                continue;
            }
            while (parseDecimalChar(lines[line][c]) == -1)
            {
                c++;
            }

            int num = getNumberByIndex(lines[line], &c);

            if (matchesString(lines[line], c, "red"))
            {
                if (num > rmax)
                {
                    rmax = num;
                }
            }
            if (matchesString(lines[line], c, "green"))
            {
                if (num > gmax)
                {
                    gmax = num;
                }
            }
            if (matchesString(lines[line], c, "blue"))
            {
                if (num > bmax)
                {
                    bmax = num;
                }
            }
            while (lines[line][c] != ';' && lines[line][c] != ',' && c < strlen(lines[line]))
            {
                c++;
            }
        }
        sum += rmax * gmax * bmax;
    }

    return sum;
}

int matchesString(char *line, int startIndex, char *string)
{
    for (int i = 0; i < strlen(string); i++)
    {
        if (i + startIndex > strlen(line))
        {
            return 0;
        }
        if (line[i + startIndex] != string[i])
        {
            return 0;
        }
    }
    return 1;
}
