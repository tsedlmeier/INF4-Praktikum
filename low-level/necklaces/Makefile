N=28
CFLAGS= -std=c99 -O2 -Wextra -Wall -DN=$(N)
LDFLAGS= -lm

.PHONY: all

all: neck debug fast neck2

run: 
	./neck2

neck2: neck2.c
	gcc -DPRINT -g -DN=$(N) -Wall $(LDFLAGS) $< -o $@

neck: neck.c
	gcc -DPRINT -DN=$(N) -Wall $(LDFLAGS) $< -o $@

debug: neck2.c
	gcc -std=c99 -g  $(LDFLAGS) $< -o $@

fast: neck2.c
	gcc $(CFLAGS) $(LDFLAGS) $< -o $@

clean:
	rm -rf neck debug fast neck2
