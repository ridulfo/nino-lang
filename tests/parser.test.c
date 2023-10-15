#include "../parser.h"

#include <stdio.h>

#include "../lexer.h"

int main() {
    char input[1024];  // A buffer to store input
    if (fgets(input, sizeof(input), stdin) != NULL) {
        printf("Input: %s\n", input);
        printf("Lexing...\n");
        TokenList* tokens = lex(input);

        printf("Parsing...\n");
        ASTList* ast_list = parse(tokens);
        ast_list->items[ast_list->length - 1] = NULL; // to prevent unused warning
    } else {
        printf("Error: Could not read input.\n");
    }
}