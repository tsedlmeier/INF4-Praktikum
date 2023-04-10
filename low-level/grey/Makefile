N=30
CFLAGS= -std=c99 -O2 -Wextra -Wall -DN=$(N)
LDFLAGS=  

.PHONY: all

all: grey debug fast

run: 
	./grey

grey: grey.c
	gcc -std=c99 -DPRINT -Wall $(LDFLAGS) $< -o $@

debug: grey.c
	gcc -std=c99 -g $(LDFLAGS) $< -o $@

fast: grey.c
	gcc $(CFLAGS) $(LDFLAGS) $< -o $@

clean:
	rm -rf grey debug fast
