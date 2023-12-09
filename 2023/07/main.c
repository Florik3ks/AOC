#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <inttypes.h>

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

    int a = 0;

    printf("task one: %d\n", taskone(lines, lineCount));
    printf("task two: %d\n", tasktwo(lines, lineCount));
}

const char labels[] = {'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'};
const char labels2[] = {'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'};
const int LABELS_COUNT = 13;
const int HAND_LENGTH = 5;

uint64_t taskone(char *lines[], int linecount)
{
    hand *hands = (hand *)malloc(linecount * sizeof(hand));

    for (int i = 0; i < linecount; i++)
    {
        for (int j = 0; j < HAND_LENGTH; j++)
        {
            hands[i].hand[j] = lines[i][j];
        }
        hands[i].hand[HAND_LENGTH] = '\0';
        int c = 6;
        int numLen;
        hands[i].num = getNumberByIndex(lines[i], &c, &numLen);
        hands[i].type = getHighestCardType(hands[i].hand);
    }

    qsort(hands, linecount, sizeof(hand), compareHands);

    int result = 0;
    for (int i = 0; i < linecount; i++)
    {
        result += hands[i].num * (i + 1);
    }
    return result;
}

uint64_t tasktwo(char *lines[], int linecount)
{
    hand *hands = (hand *)malloc(linecount * sizeof(hand));

    for (int i = 0; i < linecount; i++)
    {
        for (int j = 0; j < HAND_LENGTH; j++)
        {
            hands[i].hand[j] = lines[i][j];
        }
        hands[i].hand[HAND_LENGTH] = '\0';
        int c = 6;
        int numLen;
        hands[i].num = getNumberByIndex(lines[i], &c, &numLen);

        hands[i].type = getHighestCardTypeJoker(hands[i].hand);
    }

    qsort(hands, linecount, sizeof(hand), compareHands2);

    int result = 0;
    for (int i = 0; i < linecount; i++)
    {
        result += hands[i].num * (i + 1);
    }
    return result;
}

int isFiveOfAKind(char *hand)
{
    return isXOfAKind(hand, 5);
}

int isFourOfAKind(char *hand)
{
    return isXOfAKind(hand, 4);
}

int isFullHouse(char *hand)
{
    return isThreeOfAKind(hand) && isXOfAKind(hand, 2);
}

int isThreeOfAKind(char *hand)
{
    return isXOfAKind(hand, 3);
}

int isTwoPair(char *hand)
{
    int count;
    int hasOnePair = 0;
    for (int i = 0; i < strlen(labels); i++)
    {
        count = 0;
        for (int j = 0; j < strlen(hand); j++)
        {
            if (hand[j] == labels[i])
            {
                count++;
            }
        }
        if (count == 2)
        {
            if (!hasOnePair)
            {
                hasOnePair = 1;
            }
            else
            {
                return 1;
            }
        }
    }
    return 0;
}

int isOnePair(char *hand)
{
    int count;
    for (int i = 0; i < strlen(labels); i++)
    {
        count = 0;
        for (int j = 0; j < strlen(hand); j++)
        {
            if (hand[j] == labels[i])
            {
                count++;
            }
        }
        if (count == 2)
        {
            return 1;
        }
    }
    return 0;
}
int isHighCard(char *hand)
{
    for (int i = 0; i < strlen(hand); i++)
    {
        for (int j = i + 1; j < strlen(hand); j++)
        {
            if (hand[i] == hand[j])
            {
                return 0;
            }
        }
    }
    return 1;
}

int compareHands(const void *a, const void *b)
{
    hand hand_a = *((hand *)a);
    hand hand_b = *((hand *)b);

    if (hand_a.type == hand_b.type)
    {
        for (int i = 0; i < HAND_LENGTH; i++)
        {
            if (hand_a.hand[i] != hand_b.hand[i])
            {
                for (int j = 0; j < LABELS_COUNT; j++)
                {
                    if (hand_a.hand[i] == labels[j])
                    {
                        return 1;
                    }
                    if (hand_b.hand[i] == labels[j])
                    {
                        return -1;
                    }
                }
            }
        }
    }
    else if (hand_a.type < hand_b.type)
    {
        return -1;
    }
    return 1;
}

int compareHands2(const void *a, const void *b)
{
    hand hand_a = *((hand *)a);
    hand hand_b = *((hand *)b);

    if (hand_a.type == hand_b.type)
    {
        for (int i = 0; i < HAND_LENGTH; i++)
        {
            if (hand_a.hand[i] != hand_b.hand[i])
            {
                for (int j = 0; j < LABELS_COUNT; j++)
                {
                    if (hand_a.hand[i] == labels2[j])
                    {
                        return 1;
                    }
                    if (hand_b.hand[i] == labels2[j])
                    {
                        return -1;
                    }
                }
            }
        }
    }
    else if (hand_a.type < hand_b.type)
    {
        return -1;
    }
    return 1;
}

int getHighestCardType(char *hand)
{
    if (isFiveOfAKind(hand))
        return 6;
    if (isFourOfAKind(hand))
        return 5;
    if (isFullHouse(hand))
        return 4;
    if (isThreeOfAKind(hand))
        return 3;
    if (isTwoPair(hand))
        return 2;
    if (isOnePair(hand))
        return 1;
    if (isHighCard(hand))
        return 0;
    return -1;
}

char *getHighestCardTypeText(int type)
{
    switch (type)
    {
    case 0:
        return "highcard";
    case 1:
        return "onepair";
    case 2:
        return "twopair";
    case 3:
        return "threeofakind";
    case 4:
        return "fullhouse";
    case 5:
        return "fourofakind";
    case 6:
        return "five";
    default:
        return "idk";
    }
}

int getHighestCardTypeJoker(char *hand)
{
    int hasJ = 0;
    for (int i = 0; i < HAND_LENGTH; i++)
    {
        if (hand[i] == 'J')
        {
            hasJ = 1;
            break;
        }
    }
    if (!hasJ)
    {
        return getHighestCardType(hand);
    }

    char handcpy[6];
    strcpy(handcpy, hand);
    int max = -1;
    for (int i = 0; i < HAND_LENGTH; i++)
    {
        if (hand[i] == 'J')
        {
            for (int j = 0; j < LABELS_COUNT - 1; j++)
            {
                handcpy[i] = labels2[j];
                int num = getHighestCardTypeJoker(handcpy);
                if (num > max)
                {
                    max = num;
                }
            }
        }
    }
    return max;
}

int isXOfAKind(char *hand, int x)
{
    int count;
    for (int i = 0; i < strlen(labels); i++)
    {
        count = 0;
        for (int j = 0; j < strlen(hand); j++)
        {
            if (hand[j] == labels[i])
            {
                count++;
            }
        }
        if (count == x)
        {
            return 1;
        }
    }
    return 0;
}