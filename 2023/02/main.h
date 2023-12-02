#ifndef MAIN_H_  
#define MAIN_H_

int taskone(char *lines[], int linecount);
int getGameID(char *line);
int parseDecimalChar(char c);
int getNumberByIndex(char *string, int *index);

int tasktwo(char *lines[], int linecount);
int matchesString(char *line, int startIndex, char *string);

#endif