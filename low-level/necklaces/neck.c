#include <stdlib.h>
#include <stdio.h>
#include <math.h>

#ifndef N
#define N 4
#endif

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

void rotate_left(int* A, int n)
{
    int first = A[0];
    for (int i = 0; i <= n-2; i++) {
        A[i]=A[i+1];
    }
    A[n-1] = first;
}

int get_represent(int* A, int radix, int n)
{
    int min = conv_dec(A,radix,n);
    for (int i = 0; i < n; i++) {
        rotate_left(A,n);
        int new = conv_dec(A,radix,n);
        min = (new < min) ? new : min;
    }
    return min;
}

int contains(int* A, int val, int n)
{
    for (int i = 0; i < n; i++) {
        if( A[i]==val) {
            return 1;
        }
    }
    return 0;
}

int main()
{
    const int n = N;
    int colors = 10;
    int Num[N]; 
    fill(Num, n, 0);

    int* Min = (int*)calloc(1,sizeof(int));
  
    int repr = 0;
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
            repr = get_represent(Num,colors,n);
            if(!contains(Min,repr,c)){
                c++;
                Min = (int*)realloc(Min,c*sizeof(int));
                Min[c-1] = repr;
                #ifdef PRINT
                print(Num,N);
                printf(" -- %d\n", repr);
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
    free(Min);
}
