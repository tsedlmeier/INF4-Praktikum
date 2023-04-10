#include <stdlib.h>
#include <stdio.h>

#ifndef N
#define N 4
#endif

void print(int* A)
{
    for (int i = 0; i<N; i++)
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

int maximum(int* A, int start, int end)
{
    int max = A[0];
    for (int i = start; i < end; i++) {
        if(A[i] > max) 
            max = A[i];
    }

    return max;
}

int main()
{
    int A[N];
    fill(A,N,0);
    static int cnt=1;

    do {
        for (int i = N-1; i >= 1; i--) {
            if( A[i] < maximum(A,1,i)+1 )
            {
                A[i]++;
                cnt++;
                #ifdef PRINT
                print(A);
                #endif
                break;
            }
            else {
                A[i]=0;
            }
        }
    } while(A[N-1]!=N-1);
    printf("Anzahl: %d\n", cnt);
    return 0;
}
