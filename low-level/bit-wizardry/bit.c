#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>

// Whenever 2 following zeros appear:
// set the right of these 
// copy all left and clear all right of the 1
// *00* -> *01 0 

int main()
{
    uint32_t x = 0;
    static int cnt=1;

    while (x != 0xAAAAAAAA) {
        uint32_t y = ~( x | (x>>1) );    // gen 1 where 2 zeros are nearby
        uint32_t z =  y & ~(y-1);        // isolate lowest 1
        uint32_t mask =  z | ~(z-1) ;    // gen zeros right of lowest 1
        x = (x & mask) | z;             // apply mask to clear bits right of the 1 and set this 1 
        #ifdef PRINT
        printf("%32b - %d\n", x,x);
        #endif /* ifdef PRINT */
        cnt++;
    }
    printf("Anzahl: %d\n", cnt);
    return 0;
}
