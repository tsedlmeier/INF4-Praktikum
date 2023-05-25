CFLAGS= -O2 -Wextra -Wall
LDFLAGS=  

.PHONY: all

all: permutation2 fast2 fast

run: 
	./permutation2

permutation: permutation.c
	gcc -std=c99 -DPRINT $(CFLAGS) $(LDFLAGS) $< -o $@

permutation2: permutation2.c
	gcc -std=c99 -DN=4 -g -DPRINT $(LDFLAGS) $< -o $@

debug: permutation.c
	gcc -std=c99 -g $(LDFLAGS) $< -o $@

fast2: permutation2.c
	gcc -std=c99 -O2 -DN=11 $(LDFLAGS) $< -o $@

fast: permutation.c
	gcc -std=c99 -O2 -DD=11 $(LDFLAGS) $< -o $@

clean:
	rm -rf fast2 permutation2 permutation debug fast
