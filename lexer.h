#pragma once

#include <stdlib.h>

enum TokenType {
    TOKEN_KEYWORD,
    TOKE_SEPARATOR,
    TOKEN_IDENTIFIER,
    TOKEN_INT,
    TOKEN_OPERATOR,
};

extern char** TokenNames;

typedef struct Token {
    enum TokenType type;
    char* start;
    size_t length;
} Token;

typedef struct TokenList {
    Token* tokens;
    size_t length;
} TokenList;


/**
 * @brief Lexes the input string and returns a list of tokens.
 * 
 * @param input The input string to lex.
 * @return TokenList* A pointer to the list of tokens.
 */
TokenList* lex(char* input);

// Debugging function
void _print_token(Token* token);