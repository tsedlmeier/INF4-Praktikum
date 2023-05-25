#include <stdlib.h>
#include <stdio.h>

#define N 6

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

    static int c = 0;
    for (size_t i = N-1; i >= 0; i--) {
        int j = N-1;
        while (Num[j]==Base[j]-1) {
            Num[j] = 0;
            if (j==0) {
                return 0;
            }
            j--;
        }
        Num[j]++;
        print(Num);
    }
}
