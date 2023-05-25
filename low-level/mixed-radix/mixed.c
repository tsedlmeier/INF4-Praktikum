#include <stdlib.h>
#include <stdio.h>

#define N 4

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

int main(int argc, char** argv)
{
    const int n = N;
    int Base[N];
    int Num[N]; 
    fill(Base, n, 2);
    fill(Num, n, 0);

    int* bp = &Base[N-1]; 
    int* np = &Num[N-1]; 
    static int c = 0;
    while (1) {
        int cnt = 0;
        while (*np == (*bp)-1) {
            cnt++;
            print(Num);
            *np = 0;
            np--;
            bp--;
        }
        c++;
        if(cnt == n) break;
        (*np)++;
        bp = &Base[N-1]; 
        np = &Num[n-1];
    }
    // printf("%d", c);
}
