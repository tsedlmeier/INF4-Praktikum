#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include <string.h>

#ifndef N
#define N 6
#endif

void print(int* A, int n)
{
    for (int i = 0; i<n; i++)
    {
        printf("%d", A[i]);
    }
    printf("\n");
}

void fill(int* A, int n, int val)
{
    for (int i = 0; i < n; i++) {
        A[i] = val;
    }
}


int main()
{
    int base = 2; //colors
    int Num[N]; 
    fill(Num, N, 0);
    int c = 0;

    while ( Num[0] < base-1 ) {
        int i = 0;
        for (i = N-1; i >= 0; i--) {
            if (Num[i]!=base-1) {
                Num[i]++;
                break;
            }
        }
        // Prefix is all digits before incl. increment
        // Append Prefix to increment
        int j=0;
        while ( i < N-1 ) {
            i++;
            Num[i] = Num[j];
            j++;
        }
        // j tells the position of increment starting
        // (N-j) converts idx 6543210 to 1234567 
        // if (N-j) divides N --> is a necklace
        if ( N%(N-j) == 0 ) {
            #ifdef PRINT
            print(Num,N);
            #endif
            c++;
        }
    }
    printf("Anzahl: %d\n", c+1);
}
