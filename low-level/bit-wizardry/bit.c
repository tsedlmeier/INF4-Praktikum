#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>

// Whenever 2 following zeros appear:
// set the right of these 
// copy all left and clear all right of the 1
// *00* -> *01 0 

int main()
{
    uint8_t x = 0;
    static int cnt=1;

    while (x != 0xAA) {
        uint8_t y = ~( x | (x>>1) );    // gen 1 where 2 zeros are nearby
        uint8_t z =  y & ~(y-1);        // isolate lowest 1
        uint8_t mask =  z | ~(z-1) ;    // gen zeros right of lowest 1 and ones left
        x = (x & mask) | z;             // apply mask to clear bits right of the 1 and set this 1 
        printf("%8b - %d\n", x,x);
        cnt++;
    }
    printf("Anzahl: %d\n", cnt);
    return 0;
}
