#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include <string.h>

#ifndef N
#define N 4
#endif

// Convert a mixed-radix number to decimal
int conv_dec(int* A, int radix, int n)
{
    // (x)_10 = (a3*radix + a2)*radix + a1)*radix +a0 --> siehe Knuth
    int val_dec = A[0];
    for (int i = 1; i <= n-1; i++) {
        val_dec = val_dec * radix + A[i];
    }
    return val_dec;
}

void print(int* A, int n)
{
    for (int i = 0; i<n; i++)
    {
        printf("%d", A[i]);
    }
}

void fill(int* A, int n, int val)
{
    for (int i = 0; i < n; i++) {
        A[i] = val;
    }
}

void rotate_left(int* A, int n)
{
    int first = A[0];
    for (int i = 0; i <= n-2; i++) {
        A[i]=A[i+1];
    }
    A[n-1] = first;
}

int greater(int* A, int* B)
{
    for (int i = 0; i < N; i++) {
        if(A[i]==B[i]) 
            continue;
        if(A[i]<B[i])
            return 0;
        else
            return 1;
    }
    return -1;
}

int is_repr(int* A)
{
    int repr[N];
    memcpy(repr, A, sizeof(int)*N);
    for (int i = 0; i < N-1; i++) {
        rotate_left(repr,N);
        if ( !greater(repr,A) )
            return 0;
    }
    return 1;
}

int main()
{
    const int n = N;
    int colors = 2;
    int Num[N]; 
    fill(Num, n, 0);

    int do_run = 1;
    static int c = 1;

    while (do_run) {
        while (1) {
            int j = N-1;
            while (Num[j]==colors-1) {
                Num[j] = 0;
                j--;
            }
            Num[j]++;
            if(Num[0] == colors-1)
            {
                do_run = 0;
                break;
            }
            if( is_repr(Num) ){
                c++;
                #ifdef PRINT
                print(Num,N);
                printf(" -- %d\n", conv_dec(Num, colors, N));
                #endif
            }
        }
    }

    #ifdef PRINT
    for (size_t i = 0; i < n; i++) {
        printf("%d", colors-1);
    }
    printf(" -- %d\n", (int)pow(colors,n)-1);
    #endif
    printf("Anzahl: %d\n", c+1);
}
