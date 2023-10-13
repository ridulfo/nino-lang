#include "../parser.h"

#include <stdio.h>

#include "../lexer.h"

int main() {
    char input[1024];  // A buffer to store input
    if (fgets(input, sizeof(input), stdin) != NULL) {
        TokenList* tokens = lex(input);

        char* output = parser(tokens);
        printf("%s", output);
    } else {
        printf("Error: Could not read input.\n");
    }
}