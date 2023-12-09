#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int readinput(char *lines[], int *lineCount, char *inputfile)
{

    int i = 0;
    FILE *fp = fopen(inputfile, "r");

    if (fp == 0)
    {
        fprintf(stderr, "failed to open input.txt\n");
        exit(1);
    }

    char line[512];
    int lc = 0;
    while (fgets(line, sizeof(line), fp) != NULL)
    {
        lc++;
        lines[i] = malloc(strlen(line));
        strcpy(lines[i], line);
        i++;
    }
    *lineCount = lc;
    fclose(fp);
}