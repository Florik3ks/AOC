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
    Node *nodes = malloc(linecount * strlen(lines[0]) * sizeof(Node));
    Node start;

    // get start position
    for (size_t i = 0; i < linecount; i++)
    {
        for (size_t j = 0; j < strlen(lines[i]); j++)
        {
            char ch = lines[i][j];
            int n, e, s, w = 0;
            if (ch == 'S')
            {
                char next;
                if (i > 0)
                {
                    next = lines[i - 1][j];
                    if (next == '|' || next == 'F' || next == '7')
                    {
                        n = 1;
                    }
                }
                if (i + 1 < linecount)
                {
                    next = lines[i + 1][j];
                    if (next == '|' || next == 'J' || next == 'L')
                    {
                        s = 1;
                    }
                }
                if (j > 0)
                {
                    next = lines[i][j - 1];
                    if (next == '-' || next == 'F' || next == 'L')
                    {
                        w = 1;
                    }
                }
                if (j + 1 < strlen(lines[0]))
                {
                    next = lines[i][j + 1];
                    if (next == '-' || next == 'J' || next == '7')
                    {
                        e = 1;
                    }
                }

                if (n && e)
                {
                    start.symbol = 'L';
                }
                if (n && w)
                {
                    start.symbol = 'J';
                }
                if (s && e)
                {
                    start.symbol = 'F';
                }
                if (s && w)
                {
                    start.symbol = '7';
                }
                lines[i][j] = start.symbol;
                Vector2 pos = {j, i};
                start.pos = pos;
            }
        }
    }

    Adjacent startAdjacent = getAdjacent(lines, start.pos);
    Vector2 old = start.pos;
    Vector2 pos = startAdjacent.v1;
    size_t count = 1;
    while (!(pos.x == start.pos.x && pos.y == start.pos.y))
    {
        Vector2 tmpOld = pos;
        pos = getNextPos(lines, old, pos);
        old = tmpOld;
        count++;
    }

    // reset start symbol for p2
    lines[start.pos.y][start.pos.x] = 'S';
    return count / 2;
}

Adjacent getAdjacent(char *lines[], Vector2 v)
{
    char ch = lines[v.y][v.x];
    int x1 = 0;
    int x2 = 0;
    int y1 = 0;
    int y2 = 0;
    switch (ch)
    {
    case '|':
        y1 = 1;
        y2 = -1;
        break;
    case '-':
        x1 = 1;
        x2 = -1;
        break;
    case 'L':
        y1 = -1;
        x2 = 1;
        break;
    case 'J':
        y1 = -1;
        x2 = -1;
        break;
    case '7':
        x1 = -1;
        y2 = 1;
        break;
    case 'F':
        x1 = 1;
        y2 = 1;
        break;
    default:
        break;
    }

    Vector2 v1 = {v.x + x1, v.y + y1};
    Vector2 v2 = {v.x + x2, v.y + y2};
    Adjacent a = {v1, v2};
    return a;
}

Vector2 getNextPos(char *lines[], Vector2 old, Vector2 new)
{

    Adjacent a = getAdjacent(lines, new);
    Vector2 v1 = a.v1;
    Vector2 v2 = a.v2;

    if (v1.x == old.x && v1.y == old.y)
    {
        return v2;
    }
    else if (v2.x == old.x && v2.y == old.y)
    {
        return v1;
    }
    else
    {
        printf("???\n");
        exit(-1);
    }
}

uint64_t tasktwo(char *lines[], int linecount)
{
    // i dont know i spent too much time trying to do things that did not work
    return 0;
}
