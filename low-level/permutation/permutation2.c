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

void swap(int* a, int* b)
{
    int tmp = *b;
    *b = *a;
    *a = tmp;
}

int next(int* A)
{
    static int is_trivial = 0;
    is_trivial = !is_trivial;
    if (is_trivial) {
        swap(&A[N-2],&A[N-1]);
        return 1;
    }

    int i,j;
    for (i = N-2; i >= 0; i--) {
        if (A[i-1] < A[i]){
            i--;
            break;
        }
    }

    if ( i < 0 ) return 0;

    for (j = N-1; j >= 1; j--) {
        if (A[j] > A[i]) 
            break;
    }
    swap(&A[i], &A[j]);
    i++;
    j=N-1;
    while(i < j) {
        swap(&A[i], &A[j]);
        i++;j--;
    }
    return 1;
}

int main()
{
    int A[N];
    for (int i = 0; i < N; i++) {
        A[i] = i;
    }

    static int cnt = 0;
    do {
        #ifdef PRINT
        print(A);
        #endif /* ifdef PRINT */
        cnt++;
    } while( next(A) );
    printf("Permutations: %d\n", cnt);
}
