# Compiler
CC = clang

# Compiler flags with all warnings
CFLAGS = -Wall -Wextra -Werror -std=c11 -g -O0

lexer: lexer.o
	$(CC) $(CFLAGS) -o lexer lexer.o

parser: parser.o lexer.o
	$(CC) $(CFLAGS) -o parser parser.o lexer.o

test: lexer.o tests/lexer.test.o
	$(CC) $(CFLAGS) -o lexer.test tests/lexer.test.o lexer.o
	./lexer.test

clean:
	rm -f *.o tests/*.o lexer parser lexer.test