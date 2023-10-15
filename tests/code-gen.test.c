#include "../code-gen.h"

#include <stdio.h>

#include "../lexer.h"
#include "../parser.h"

int main() {
    char* input = "let x: i32 = 2; let y: i32 = 3; let z: i32 = x + y;";
    printf("Input: %s\n", input);
    printf("Lexing...\n");
    TokenList* tokens = lex(input);

    printf("Parsing...\n");
    ASTList* ast_list = parse(tokens);

    printf("Code Generation...\n");
    char* code = code_gen(ast_list);
    printf("Output:\n%s\n", code);

    FILE* file = fopen("build/code.ll", "w");
    fprintf(file, "%s", code);
    fclose(file);
    return 0;
}