# Compiler
CC = clang

# Compiler flags with all warnings
CFLAGS = -Wall -Wextra -Werror -std=c11 -g -O0


build/lexer.o: lexer.c lexer.h
	$(CC) $(CFLAGS) -o build/lexer.o -c lexer.c

build/parser.o: parser.c
	$(CC) $(CFLAGS) -o build/parser.o -c parser.c

build/code-gen.o: code-gen.c 
	$(CC) $(CFLAGS) -o build/code-gen.o -c code-gen.c

build/lexer.test.o: build/lexer.o tests/lexer.test.c
	$(CC) $(CFLAGS) -o build/lexer.test.o -c tests/lexer.test.c	

test-lexer: build/lexer.o build/lexer.test.o
	$(CC) $(CFLAGS) -o build/lexer.test build/lexer.test.o build/lexer.o
	./build/lexer.test

test-parser: build/parser.o build/lexer.o
	$(CC) $(CFLAGS) -o build/parser.test tests/parser.test.c build/parser.o build/lexer.o

test-code-gen: build/code-gen.o build/parser.o build/lexer.o
	$(CC) $(CFLAGS) -o build/code-gen.test tests/code-gen.test.c build/code-gen.o build/parser.o build/lexer.o

clean:
	rm -r build/*
	