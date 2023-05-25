CFLAGS= -O3 -Wextra -Wall
LDFLAGS= 

.PHONY: all

all: mixed2 mixed debug fast

run: 
	./permutation

mixed2: mixed2.c
	gcc -std=c99 -DPRINT $(CFLAGS) $(LDFLAGS) $< -o $@

mixed: mixed.c
	gcc -std=c99 -DPRINT $(CFLAGS) $(LDFLAGS) $< -o $@

debug: mixed.c
	gcc -std=c99 -g $(LDFLAGS) $< -o $@

fast: mixed.c
	gcc -std=c99 -O2 $(LDFLAGS) $< -o $@

clean:
	rm -rf mixed2 mixed debug fast
