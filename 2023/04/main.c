#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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

int taskone(char *lines[], int linecount)
{

    int sum = 0;
    for (int i = 0; i < linecount; i++)
    {
        int score = 0;
        int winning[10] = {0};
        int index = 0;
        int c = 0;

        while (lines[i][c] != ':')
        {
            c++;
        }
        c++;
        int length;
        while (lines[i][c] != '|')
        {

            int num = getNumberByIndex(lines[i], &c, &length);
            if (num == 0)
                continue;

            winning[index] = num;
            index++;
        }
        c++;
        while (c < strlen(lines[i]))
        {
            int num = getNumberByIndex(lines[i], &c, &length);
            if (num == 0)
                continue;

            if (contains(winning, num, 10))
            {
                if (score == 0)
                {
                    score++;
                }
                else
                {
                    score *= 2;
                }
            }
        }
        sum += score;
    }
    return sum;
}

int tasktwo(char *lines[], int linecount)
{

    int cards[200] = {0}; // i hate c why is int cards[linecount] not a thing
    for (int i = 0; i < linecount; i++)
    {
        cards[i] = 1;
    }

    for (int i = 0; i < linecount; i++)
    {
        int score = 0;
        int winning[10] = {0};
        int index = 0;
        int c = 0;

        while (lines[i][c] != ':')
        {
            c++;
        }
        c++;
        int length;
        while (lines[i][c] != '|')
        {

            int num = getNumberByIndex(lines[i], &c, &length);
            if (num == 0)
                continue;

            winning[index] = num;
            index++;
        }
        c++;
        while (c < strlen(lines[i]))
        {
            int num = getNumberByIndex(lines[i], &c, &length);
            if (num == 0)
                continue;

            if (contains(winning, num, 10))
            {
                score++;
            }
        }
        for (int k = 0; k < score; k++)
        {
            cards[i + k + 1] += cards[i];
        }
    }

    int result = 0;
    for (int i = 0; i < linecount; i++){
        result += cards[i];
    }
    return result;
}
