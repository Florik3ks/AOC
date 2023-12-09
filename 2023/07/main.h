#ifndef MAIN_H_  
#define MAIN_H_

uint64_t taskone(char *lines[], int linecount);
uint64_t tasktwo(char *lines[], int linecount);

int isFiveOfAKind(char *hand);
int isFourOfAKind(char *hand);
int isFullHouse(char *hand);
int isThreeOfAKind(char *hand);
int isTwoPair(char *hand);
int isOnePair(char *hand);
int isHighCard(char *hand);

int compareHands(const void *a, const void *b);
int compareHands2(const void *a, const void *b);
int getHighestCardType(char *hand);
char *getHighestCardTypeText(int type);
int getHighestCardTypeJoker(char *hand);

int isXOfAKind(char *hand, int x);
typedef struct
{
    char hand[6];
    int type;
    int num;
} hand;

#endif