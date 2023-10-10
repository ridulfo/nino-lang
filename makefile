# Compiler
CC = clang

# Compiler flags with all warnings
CFLAGS = -Wall -Wextra -Werror -std=c11 -g -O0

lexer: lexer.o
	$(CC) $(CFLAGS) -o lexer lexer.o

test: lexer.o tests/lexer.test.o
	$(CC) $(CFLAGS) -o lexer.test tests/lexer.test.o lexer.o
	./lexer.test

clean:
	rm -f *.o

clean-all:
	rm -f *.o tests/*.o lexer lexer.test