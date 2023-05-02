N=6
CFLAGS= -std=c99 -O2 -Wextra -Wall -DN=$(N)
LDFLAGS= -lm

.PHONY: all

all: neck_fast TS_algo FKM_algo neck debug fast neck2

run: 
	./neck_fast

neck_fast: neck_fast.c
	gcc -DPRINT -g -DN=$(N) -Wall $(LDFLAGS) $< -o $@

neck2: neck2.c
	gcc -DPRINT -g -DN=$(N) -Wall $(LDFLAGS) $< -o $@

neck: neck.c
	gcc -DPRINT -DN=$(N) -Wall $(LDFLAGS) $< -o $@

debug: neck_fast.c
	gcc -std=c99 -g  $(LDFLAGS) $< -o $@

FKM_algo: neck_fast.c
	gcc $(CFLAGS) -DN=30 $(LDFLAGS) $< -o $@

TS_algo: neck2.c
	gcc $(CFLAGS) -DN=30 $(LDFLAGS) $< -o $@

clean:
	rm -rf neck_fast TS_algo FKM_algo neck debug fast neck2
