#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <inttypes.h>
#include <math.h>

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

    printf("task one: %llu\n", taskone(lines, lineCount));
    printf("task two: %llu\n", tasktwo(lines, lineCount));
}

int hash_idk(char *key)
{
    int result = 0;
    for (int i = 0; i < strlen(key); i++)
    {
        result += key[i];
    }
    return result *= key[0];
}

int insert(map *table, char *key_, char *left_, char *right_)
{
    char *key = malloc(sizeof(char) * 4);
    char *left = malloc(sizeof(char) * 4);
    char *right = malloc(sizeof(char) * 4);
    strcpy(key, key_);
    strcpy(left, left_);
    strcpy(right, right_);
    int index = hash_idk(key);

    for (int i = index; i < table->max; i++)
    {
        if (!table->elements[i].isSet)
        {
            element e = {1, key, left, right};
            table->elements[i] = e;
            return 1;
        }
        if (!strcmp(table->elements[i].key, key))
        {
            return 0;
        }
    }

    for (int i = 0; i < index; i++)
    {
        if (!table->elements[i].isSet)
        {
            element e = {1, key, left, right};
            table->elements[i] = e;
            return 1;
        }
        if (!strcmp(table->elements[i].key, key))
        {
            return 0;
        }
    }
    return -1;
}
int find(map *table, char *key, int isLeft, char **result)
{
    int index = hash_idk(key);
    for (int i = index; i < table->max; i++)
    {
        element e = table->elements[i];
        if (!e.isSet)
        {
            return 0;
        }
        if (e.isSet && !strcmp(e.key, key))
        {
            *result = isLeft ? e.left : e.right;
            return 1;
        }
    }

    for (int i = 0; i < index; i++)
    {
        element e = table->elements[i];
        if (!e.isSet)
        {
            return 0;
        }
        if (e.isSet && !strcmp(e.key, key))
        {
            *result = isLeft ? e.left : e.right;
            return 1;
        }
    }
}

uint64_t taskone(char *lines[], int linecount)
{
    element e[10000] = {{0}};
    map hashmap = {e, 10000};

    for (int i = 2; i < linecount; i++)
    {
        char key[4];
        char left[4];
        char right[4];
        for (int j = 0; j < 3; j++)
        {
            key[j] = lines[i][j];
            left[j] = lines[i][j + 7];
            right[j] = lines[i][j + 12];
        }
        key[3] = '\0';
        left[3] = '\0';
        right[3] = '\0';
        insert(&hashmap, key, left, right);
    }
    int steps = 0;
    char *key = "AAA";
    while (strcmp(key, "ZZZ"))
    {
        int i = steps % (strlen(lines[0]) - 1);
        char instruction = lines[0][i];
        char *result;
        int valid = find(&hashmap, key, instruction == 'L', &result);
        if (!valid)
        {
            printf("???\n");
        }
        key = result;
        steps++;
    }
    return steps;
}

uint64_t tasktwo(char *lines[], int linecount)
{
    element e[10000] = {{0}};
    map hashmap = {e, 10000};

    char *keys[512] = {0};
    int index = 0;
    for (int i = 2; i < linecount; i++)
    {
        char key[4];
        char left[4];
        char right[4];
        for (int j = 0; j < 3; j++)
        {
            key[j] = lines[i][j];
            left[j] = lines[i][j + 7];
            right[j] = lines[i][j + 12];
        }
        key[3] = '\0';
        left[3] = '\0';
        right[3] = '\0';
        insert(&hashmap, key, left, right);
        if (key[2] == 'A')
        {
            char *nkey = malloc(sizeof(char) * 4);
            strcpy(nkey, key);
            keys[index] = nkey;
            index++;
        }
    }
    int times[512] = {0};
    uint64_t result = 1;
    for (int i = 0; i < index; i++)
    {
        int steps = 0;
        char *key = keys[i];
        while (key[2] != 'Z')
        {
            int i = steps % (strlen(lines[0]) - 1);
            char instruction = lines[0][i];
            char *result;
            int valid = find(&hashmap, key, instruction == 'L', &result);
            if (!valid)
            {
                printf("???\n");
            }
            key = result;
            steps++;
        }
        times[i] = steps;
        result = lcm(result, steps);
    }
    
    return result;
}

// https://www.geeksforgeeks.org/lcm-of-two-numbers-in-c/
uint64_t gcd(uint64_t a, uint64_t b) 
{ 
    if (b == 0) 
        return a; 
    return gcd(b, a % b); 
} 
  
// Function to return LCM of two numbers 
uint64_t lcm(uint64_t a, uint64_t b) { return (a / gcd(a, b)) * b; } 
