#include "../lexer.h"

#include <assert.h>
#include <stdio.h>
#include <string.h>

void compare_tokens(Token* expected, Token* actual) {
    int failed = 0;
    if (expected->type != actual->type) {
        printf("Expected type: %s, Actual type: %s\n", TokenNames[expected->type], TokenNames[actual->type]);
        failed = 1;
    }
    if (strcmp(expected->text, actual->text) != 0) {
        printf("Expected text: %s, Actual text: %s\n", expected->text, actual->text);
        failed = 1;
    }
    if (expected->length != actual->length) {
        printf("Expected length: %zu, Actual length: %zu\n", expected->length, actual->length);
        failed = 1;
    }
    if (failed) {
        print_token(expected);
        assert(0);
    }
}

void test_variable_assignment() {
    printf("Testing assignment...\n");
    char* input = "let x:i32 = 55;";
    printf("Input: %s\n", input);

    TokenList* expected = malloc(sizeof(TokenList));
    Token expected_tokens[] = {
        {TOKEN_LET, "let", 3},
        {TOKEN_IDENTIFIER, "x", 1},
        {TOKEN_COLON, ":", 1},
        {TOKEN_TYPE, "i32", 3},
        {TOKEN_ASSIGNMENT, "=", 1},
        {TOKEN_LITERAL_INT, "55", 2},
        {TOKEN_SEMICOLON, ";", 1},
    };

    expected->length = 7;
    expected->tokens = malloc(expected->length * sizeof(Token));
    memcpy(expected->tokens, expected_tokens, expected->length * sizeof(Token));

    TokenList* tokens = lex(input);

    for (size_t i = 0; i < tokens->length; i++) {
        print_token(&tokens->tokens[i]);
    }
    assert(tokens->length == expected->length);

    for (size_t i = 0; i < tokens->length; i++) {
        compare_tokens(&expected->tokens[i], &tokens->tokens[i]);
    }

    printf("Passed!\n\n");
}

void test_function_declaration() {
    printf("Testing function declaration...\n");
    char* input =
        "fn is_prime = (x:i32):bool =>"
        "| let sqrt_x:f32 = sqrt(x);"
        "| let sqrt_x_int:i32 = floor(sqrt_x);"
        "=> true ? {"
        "    x==1 => false,"
        "    x==2 => true,"
        "    x mod 2 == 0 => false,"
        "    true => is_prime_helper(x, 3, sqrt_x_int)"
        "};";
    TokenList* expected = malloc(sizeof(TokenList));

    Token expected_tokens[] = {
        {TOKEN_FN, "fn", 2},
        {TOKEN_IDENTIFIER, "is_prime", 8},
        {TOKEN_ASSIGNMENT, "=", 1},
        {TOKEN_LPAREN, "(", 1},
        {TOKEN_IDENTIFIER, "x", 1},
        {TOKEN_COLON, ":", 1},
        {TOKEN_TYPE, "i32", 3},
        {TOKEN_RPAREN, ")", 1},
        {TOKEN_COLON, ":", 1},
        {TOKEN_TYPE, "bool", 4},
        {TOKEN_ARROW, "=>", 2},
        {TOKEN_PIPE, "|", 1},
        {TOKEN_LET, "let", 3},
        {TOKEN_IDENTIFIER, "sqrt_x", 6},
        {TOKEN_COLON, ":", 1},
        {TOKEN_TYPE, "f32", 3},
        {TOKEN_ASSIGNMENT, "=", 1},
        {TOKEN_IDENTIFIER, "sqrt", 4},
        {TOKEN_LPAREN, "(", 1},
        {TOKEN_IDENTIFIER, "x", 1},
        {TOKEN_RPAREN, ")", 1},
        {TOKEN_SEMICOLON, ";", 1},
        {TOKEN_PIPE, "|", 1},
        {TOKEN_LET, "let", 3},
        {TOKEN_IDENTIFIER, "sqrt_x_int", 10},
        {TOKEN_COLON, ":", 1},
        {TOKEN_TYPE, "i32", 3},
        {TOKEN_ASSIGNMENT, "=", 1},
        {TOKEN_IDENTIFIER, "floor", 5},
        {TOKEN_LPAREN, "(", 1},
        {TOKEN_IDENTIFIER, "sqrt_x", 6},
        {TOKEN_RPAREN, ")", 1},
        {TOKEN_SEMICOLON, ";", 1},
        {TOKEN_ARROW, "=>", 2},
        {TOKEN_LITERAL_BOOL, "true", 4},
        {TOKEN_QUESTION, "?", 1},
        {TOKEN_LBRACE, "{", 1},
        {TOKEN_IDENTIFIER, "x", 1},
        {TOKEN_EQUAL, "==", 2},
        {TOKEN_LITERAL_INT, "1", 1},
        {TOKEN_ARROW, "=>", 2},
        {TOKEN_LITERAL_BOOL, "false", 5},
        {TOKEN_COMMA, ",", 1},
        {TOKEN_IDENTIFIER, "x", 1},
        {TOKEN_EQUAL, "==", 2},
        {TOKEN_LITERAL_INT, "2", 1},
        {TOKEN_ARROW, "=>", 2},
        {TOKEN_LITERAL_BOOL, "true", 4},
        {TOKEN_COMMA, ",", 1},
        {TOKEN_IDENTIFIER, "x", 1},
        {TOKEN_MOD, "mod", 3},
        {TOKEN_LITERAL_INT, "2", 1},
        {TOKEN_EQUAL, "==", 2},
        {TOKEN_LITERAL_INT, "0", 1},
        {TOKEN_ARROW, "=>", 2},
        {TOKEN_LITERAL_BOOL, "false", 5},
        {TOKEN_COMMA, ",", 1},
        {TOKEN_LITERAL_BOOL, "true", 4},
        {TOKEN_ARROW, "=>", 2},
        {TOKEN_IDENTIFIER, "is_prime_helper", 15},
        {TOKEN_LPAREN, "(", 1},
        {TOKEN_IDENTIFIER, "x", 1},
        {TOKEN_COMMA, ",", 1},
        {TOKEN_LITERAL_INT, "3", 1},
        {TOKEN_COMMA, ",", 1},
        {TOKEN_IDENTIFIER, "sqrt_x_int", 10},
        {TOKEN_RPAREN, ")", 1},
        {TOKEN_RBRACE, "}", 1},
        {TOKEN_SEMICOLON, ";", 1},
    };

    expected->length = 69;
    expected->tokens = malloc(expected->length * sizeof(Token));
    memcpy(expected->tokens, expected_tokens, expected->length * sizeof(Token));

    TokenList* tokens = lex(input);

    for (size_t i = 0; i < tokens->length; i++) {
        print_token(&tokens->tokens[i]);
    }

    if (tokens->length != expected->length) {
        printf("Expected length: %zu, Actual length: %zu\n", expected->length, tokens->length);
        assert(0);
    }

    for (size_t i = 0; i < tokens->length; i++) {
        compare_tokens(&expected->tokens[i], &tokens->tokens[i]);
    }

    printf("Passed!\n\n");
}

int main() {
    test_variable_assignment();
    test_function_declaration();

    return 0;
}