#include "../lexer.h"

#include <assert.h>
#include <string.h>
#include <stdio.h>

void test_assignment() {
    printf("Testing assignment...\n");
    char* input = "let x = 5;";
    TokenList* tokens = lex(input);
    for (size_t i = 0; i < tokens->length; i++) {
        _print_token(&tokens->tokens[i]);
    }
    assert(tokens->length == 5);

    // "let"
    Token* token = &tokens->tokens[0];
    assert(token->type == TOKEN_KEYWORD);
    assert(token->length == 3);
    assert(strncmp(token->start, "let", 3) == 0);

    // "x"
    token = &tokens->tokens[1];
    assert(token->type == TOKEN_IDENTIFIER);
    assert(token->length == 1);
    assert(strncmp(token->start, "x", 1) == 0);

    // "="
    token = &tokens->tokens[2];
    assert(token->type == TOKEN_OPERATOR);
    assert(token->length == 1);
    assert(strncmp(token->start, "=", 1) == 0);

    // "5"
    token = &tokens->tokens[3];
    assert(token->type == TOKEN_INT);
    assert(token->length == 1);
    assert(strncmp(token->start, "5", 1) == 0);

    // ";"
    token = &tokens->tokens[4];
    assert(token->type == TOKE_SEPARATOR);
    assert(token->length == 1);
    assert(strncmp(token->start, ";", 1) == 0);

    printf("Passed!\n\n");
}

int main() {
    test_assignment();

    return 0;
}