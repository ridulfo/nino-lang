#include "../code-gen.h"

#include <stdio.h>

#include "../lexer.h"
#include "../parser.h"

int main() {
    char input[1024];  // A buffer to store input
    if (fgets(input, sizeof(input), stdin) != NULL) {
        TokenList* tokens = lex(input);

        ASTList* ast_list = parse(tokens);

        char* code = code_gen(ast_list);

        printf("%s\n", code);
    } else {
        printf("Error: Could not read input.\n");
    }

    return 0;
}