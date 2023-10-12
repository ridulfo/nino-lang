#pragma once

#include <stdlib.h>

/**
 * @brief The different types of tokens.
 */
enum TokenType {
    TOKEN_LET,
    TOKEN_FN,
    TOKEN_PRINT,
    TOKEN_SEPARATOR,
    TOKEN_IDENTIFIER,
    TOKEN_INT,
    TOKEN_ASSIGNMENT,
    TOKEN_ARROW,
    TOKEN_OPERATOR,
    TOKEN_END_STATEMENT
};

/**
 * @brief The names of the different types of tokens.
 */
static char** TokenNames = (char*[]){
    "LET",
    "FN",
    "PRINT",
    "SEPARATOR",
    "IDENTIFIER",
    "INT",
    "ASSIGNMENT",
    "ARROW",
    "OPERATOR",
    "END_STATEMENT"};

/**
 * @brief Token object that contains the type and the text of the token.
 */
typedef struct Token {
    enum TokenType type;
    char* text;
    size_t length;
} Token;

/**
 * @brief A simple list of tokens.
 */
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