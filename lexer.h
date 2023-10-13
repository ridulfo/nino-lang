#pragma once

#include <stdlib.h>

/**
 * @brief The different types of tokens.
 */
enum TokenType {
    // keywords
    TOKEN_LET,
    TOKEN_FN,

    // types
    TOKEN_INT,

    // builtins
    TOKEN_PRINT,

    // separators
    TOKEN_LPAREN,
    TOKEN_RPAREN,
    TOKEN_COMMA,
    TOKEN_SEMICOLON,

    // identifiers
    TOKEN_IDENTIFIER,

    // operators
    TOKEN_ADD,
    TOKEN_SUB,
    TOKEN_MUL,
    TOKEN_DIV,

    // don't know what to call these
    TOKEN_ASSIGNMENT,
    TOKEN_ARROW,

    // equality
    TOKEN_EQUAL,
    TOKEN_NOTEQUAL,
};

/**
 * @brief The names of the different types of tokens.
 */
static char** TokenNames = (char*[]){
    "LET",
    "FN",

    "INT",

    "PRINT",

    "LPAREN",
    "RPAREN",
    "COMMA",
    "SEMICOLON",

    "IDENTIFIER",

    "ADD",
    "SUB",
    "MUL",
    "DIV",

    "ASSIGNMENT",
    "ARROW",

    "EQUAL",
    "NOTEQUAL",
};

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