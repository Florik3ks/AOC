#ifndef MAIN_H_  
#define MAIN_H_

typedef struct{
    int x;
    int y;
} Vector2;

typedef enum
{
    North,
    East,
    South,
    West
} Direction;

typedef struct {
    char symbol;
    Vector2 pos;
} Node;

typedef struct{
    Vector2 v1;
    Vector2 v2;
} Adjacent;



uint64_t taskone(char *lines[], int linecount);
Adjacent getAdjacent(char *lines[], Vector2 v);
Vector2 getNextPos(char *lines[], Vector2 old, Vector2 new);
uint64_t tasktwo(char *lines[], int linecount);


#endif