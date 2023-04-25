N=4
CFLAGS= -std=c99 -O2 -Wextra -Wall -DN=$(N)
LDFLAGS= -lm

.PHONY: all

all: neck debug fast

run: 
	./neck

neck: neck.c
	gcc -DPRINT -DN=$(N) -Wall $(LDFLAGS) $< -o $@

debug: neck.c
	gcc -std=c99 -g  $(LDFLAGS) $< -o $@

fast: neck.c
	gcc $(CFLAGS) $(LDFLAGS) $< -o $@

clean:
	rm -rf neck debug fast
