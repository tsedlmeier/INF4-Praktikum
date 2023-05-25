CFLAGS= -std=c99 -O2 -Wextra -Wall -DN=$(N)
LDFLAGS= 

.PHONY: all

all: bit fast

run: 
	./bit

bit: bit.c
	gcc -DPRINT -DN=$(N) -Wall $(LDFLAGS) $< -o $@

fast: bit.c
	gcc -O2 -Wall $(LDFLAGS) $< -o $@

clean:
	rm -rf bit fast
