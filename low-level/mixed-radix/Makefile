CFLAGS= -O3 -Wextra -Wall
LDFLAGS= 

.PHONY: all

all: mixed debug fast

run: 
	./permutation

mixed: mixed.c
	gcc -std=c99 -DPRINT $(CFLAGS) $(LDFLAGS) $< -o $@

debug: mixed.c
	gcc -std=c99 -g $(LDFLAGS) $< -o $@

fast: mixed.c
	gcc -std=c99 -O2 $(LDFLAGS) $< -o $@

clean:
	rm -rf mixed debug fast
