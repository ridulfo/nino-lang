#pragma once

#include <stdlib.h>

/**
 * @brief The different types of tokens.
 */
enum TokenType {
    // keywords
    TOKEN_LET,

    // types and values
    TOKEN_TYPE,
    TOKEN_LITERAL_INT,
    TOKEN_LITERAL_FLOAT,
    TOKEN_LITERAL_STRING,
    TOKEN_LITERAL_BOOL,
    TOKEN_FN,

    // builtins
    TOKEN_PRINT,
    TOKEN_MOD,

    // separators
    TOKEN_LPAREN,
    TOKEN_RPAREN,
    TOKEN_LBRACKET,
    TOKEN_RBRACKET,
    TOKEN_LBRACE,
    TOKEN_RBRACE,
    TOKEN_COMMA,
    TOKEN_COLON,
    TOKEN_SEMICOLON,
    TOKEN_QUOTE,
    TOKEN_PIPE,

    // identifiers
    TOKEN_IDENTIFIER,

    // operators
    TOKEN_ADD,
    TOKEN_SUB,
    TOKEN_MUL,
    TOKEN_DIV,
    TOKEN_NOT,

    // don't know what to call these
    TOKEN_ASSIGNMENT,
    TOKEN_ARROW,
    TOKEN_QUESTION,

    // equality
    TOKEN_EQUAL,
    TOKEN_NOTEQUAL,
    TOKEN_GTHAN,
    TOKEN_GETHAN,
    TOKEN_LTHAN,
    TOKEN_LETHAN,

    // end of file
    TOKEN_EOF
};

/**
 * @brief The names of the different types of tokens.
 */
static char** TokenNames = (char*[]){
    "LET",

    "TYPE",
    "LITERAL_INT",
    "LITERAL_FLOAT",
    "LITERAL_STRING",
    "LITERAL_BOOL",
    "FN",

    "PRINT",
    "MOD",

    "LPAREN",
    "RPAREN",
    "LBRACKET",
    "RBRACKET",
    "LBRACE",
    "RBRACE",
    "COMMA",
    "COLON",
    "SEMICOLON",
    "QUOTE",
    "PIPE",

    "IDENTIFIER",

    "ADD",
    "SUB",
    "MUL",
    "DIV",
    "NOT",

    "ASSIGNMENT",
    "ARROW",
    "QUESTION",

    "EQUAL",
    "NOTEQUAL",
    "GTHAN",
    "GETHAN",
    "LTHAN",
    "LETHAN",

    "EOF"

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

void print_token(Token* token);