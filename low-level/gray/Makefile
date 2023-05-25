N=30
CFLAGS= -std=c99 -O2 -Wextra -Wall -DN=$(N)
LDFLAGS=  

.PHONY: all

all: gray debug fast

run: 
	./gray

gray: gray.c
	gcc -std=c99 -DPRINT -Wall $(LDFLAGS) $< -o $@

debug: gray.c
	gcc -std=c99 -g $(LDFLAGS) $< -o $@

fast: gray.c
	gcc $(CFLAGS) $(LDFLAGS) $< -o $@

clean:
	rm -rf gray debug fast
