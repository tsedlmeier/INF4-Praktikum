CFLAGS= -O2 -Wextra -Wall
LDFLAGS=  

.PHONY: all

all: permutation debug fast

run: 
	./permutation

permutation: permutation.c
	gcc -std=c99 -DPRINT $(CFLAGS) $(LDFLAGS) $< -o $@

debug: permutation.c
	gcc -std=c99 -g $(LDFLAGS) $< -o $@

fast: permutation.c
	gcc -std=c99 -O2 -DD=11 $(LDFLAGS) $< -o $@

clean:
	rm -rf permutation debug fast
