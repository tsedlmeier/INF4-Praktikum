N=15
CFLAGS= -std=c99 -O2 -Wextra -Wall -DN=$(N)
LDFLAGS= 

.PHONY: all

all: bell debug fast

run: 
	./bell

bell: bell.c
	gcc -DPRINT -Wall $(LDFLAGS) $< -o $@

debug: bell.c
	gcc -std=c99 -g  $(LDFLAGS) $< -o $@

fast: bell.c
	gcc $(CFLAGS) $(LDFLAGS) $< -o $@

clean:
	rm -rf bell debug fast
