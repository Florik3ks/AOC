#ifndef MAIN_H_
#define MAIN_H_

uint64_t taskone(char *lines[], int linecount);
uint64_t tasktwo(char *lines[], int linecount);



int hash_idk(char *key);

typedef struct
{
    int isSet;
    char *key;
    char *left;
    char *right;
} element;

typedef struct
{
    element *elements;
    int max;
} map;

int insert(map *table, char *key, char *left, char *right);
int find(map *table, char *key, int isLeft, char **result);
uint64_t gcd(uint64_t a, uint64_t b);
uint64_t lcm(uint64_t a, uint64_t b);

#endif