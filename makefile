# Compiler
CC = clang

# Compiler flags with all warnings
CFLAGS = -Wall -Wextra -Werror -std=c11 -g -O0


build/lexer.o: lexer.c lexer.h
	$(CC) $(CFLAGS) -o build/lexer.o -c lexer.c

build/parser.o: parser.c
	$(CC) $(CFLAGS) -o build/parser.o -c parser.c

parser: build/parser.o build/lexer.o
	$(CC) $(CFLAGS) -o build/parser build/parser.o build/lexer.o

build/lexer.test.o: build/lexer.o tests/lexer.test.c
	$(CC) $(CFLAGS) -o build/lexer.test.o -c tests/lexer.test.c	

test-lexer: build/lexer.o build/lexer.test.o
	$(CC) $(CFLAGS) -o build/lexer.test build/lexer.test.o build/lexer.o
	./build/lexer.test

clean:
	rm -f build/*
	