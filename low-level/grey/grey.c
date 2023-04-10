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

int main()
{
    const int n = N;
    int cnt = 1;
    int R[N];
    int Num[N];
    int Base[N];
    fill(R,N,0);
    fill(Num,N,0);
    fill(Base,N,2);
    int* np = &Num[n-1];
    int* bp = &Base[n-1];
    int* rp = &R[n-1];

    // 0 -> Vorwärts
    // 1 -> Rückwärts
    while (1) {
        while ( (*np == (*bp)-1 && !(*rp)) || (*np == 0 && *rp) ) {
            if(*np == (*bp)-1 && !(*rp)) 
                *rp = !(*rp);
            if(*np == 0 && *rp)
                *rp = !(*rp);
            np--;            
            bp--;            
            rp--;            
            
        }
        if (!(*rp)) {
            (*np)++;
        }
        else {
            (*np)--;
        }

        #ifdef PRINT
        print(Num);
        #endif
        cnt++;

        // Doesnt work for Base 2
        if (Num[0] == Base[0]-1 && Num[n-1] == 0) break;
        np = &Num[n-1];
        bp = &Base[n-1];
        rp = &R[n-1];

    }
    printf("%d\n", cnt);
}
