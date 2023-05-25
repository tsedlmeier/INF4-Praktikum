CFLAGS= -O2 -Wextra -Wall
LDFLAGS= -lm

.PHONY: all

all: fft debug debug

run: 
	./fft

fft: fft.c
	gcc -std=c99 -DPRINT $(CFLAGS) $(LDFLAGS) $< -o $@

debug: fft.c
	gcc -std=c99 -DPRINT -g $(LDFLAGS) $< -o $@

clean:
	rm fft debug
