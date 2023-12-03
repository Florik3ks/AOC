#ifndef MAIN_H_  
#define MAIN_H_

int taskone(char *lines[], int linecount);
int tasktwo(char *lines[], int linecount);
int parseDecimalChar(char c);
int getNumberByIndex(char *string, int *index, int *numLen);
int searchSurroundingForSymbol(char *lines[], int lineCount, int i, int j);
int searchSurroundingForNumbers(char *lines[], int lineCount, int i, int j);
int contains(int a[], int num, int max);
#endif