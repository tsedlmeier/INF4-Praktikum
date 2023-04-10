#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#ifndef D
#define D 5
#endif
int stop[D];

void print(int* A)
{
    for (int i = 0; i<D; i++)
    {
        printf("%d", A[i]);
    }
    printf("\n");
}

void roll(int* A, int* p)
{
    int* roller = p+1;
    int is_unique = 1;
    // go from set *p = 1->N and 
    // check if *p is in digits left 
    for (int i = 1 ; i<=D ; i++) {
        is_unique = 1;
        while(1)
        {
            if(*p==i) 
            {
                // is not unique --> reset p
                is_unique = 0;
                p = roller-1;
                break;
            }
            else {
                // unique for the moment --> check more left
                p--;
            }
            if(p == &A[-1]) 
            {
                // is really unique 
                p = roller-1;
                if(is_unique)
                {
                    // safe i val and move to next digit
                    *roller = i;
                    roller++;
                }
                break;
            }
        }
    }
}

// Inkrement second last digit 
// 12345 --> 12355
//              ^-*p
void inc(int *A)
{
    int* p = &A[D-2];
    int* cmp = p-1;
    //      1 2 3 4 5 --> 1 2 3 5 5
    //     *cmp-^ ^-*p
    // search for first digit thats not D (5)
    while(*p == D)
    {
        p--;
        cmp = p-1;
    }
    // Increment that found digit
    (*p)++;
    // Check if Increment is valid
    while(cmp != &A[-1])
    {
        // Handle same
        if(*p == *cmp)
        {
            // if Increment reaches D (5) move to next digit
            while(*p == D)
                p--;
            (*p)++;
            cmp = p-1;
        }
        // Digit is still valid
        else
            cmp--;
    }
    // Roll out
    roll(A,p);
}

void perm(int* A)
{
    static int cnt = 0; cnt++;

    #ifdef PRINT
    print(A);
    #endif

    if(memcmp(A,stop, sizeof(int)*D) == 0) 
    {
        printf("Permutations: %d", cnt);
        return;
    }
    inc(A);
    perm(A);
}

int main()
{
    int A[D];
    for (int i = 0; i < D; i++) {
        stop[i] = D-i;
        A[i] = i+1;
    }
    perm(A);
}
