#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../readinput.h"
#include "main.h"


int main(int argc, char *argv[])
{
    char *lines[5000];
    int lineCount;
    if(argc < 2){
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
    return -1;
}


int tasktwo(char *lines[], int linecount)
{
    return -1;
}
