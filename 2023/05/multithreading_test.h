#ifndef MULTITHREADTEST_H_  
#define MULTITHREADTEST_H_

uint64_t taskone(char *lines[], int linecount);
uint64_t tasktwo(char *lines[], int linecount);

int convert(char *map[], int maplines, uint64_t *seeds, uint64_t arrSize);
void *taskTwoThread(void *arg);
#endif