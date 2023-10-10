#include "lexer.h"

#include <stddef.h>
#include <stdio.h>
#include <string.h>

char** TokenNames = (char*[]){"KEYWORD", "SEPARATOR", "IDENTIFIER", "INT",
                              "OPERATOR"};

char** KeywordNames = (char*[]){"let"};

void set_token(Token* token, enum TokenType type, char* start, size_t length) {
    *token = (Token){type, strndup(start, length)};
}

void _print_token(Token* token) {
    printf("Value: %s, ", token->text);
    printf("Type: %s\n", TokenNames[token->type]);
}

// consumes all whitespace characters from the input
void consume_whitespace(char** input) {
    while (**input == ' ' || **input == '\n') {
        (*input)++;
    }
}

void parse_word(char** input, Token* token) {
    size_t length = 0;
    char* start = *input;
    while (**input >= 'a' && **input <= 'z') {
        (*input)++;
        length++;
    }
    set_token(token, TOKEN_IDENTIFIER, start, length);

    // check if the word is a keyword
    for (size_t i = 0; i < sizeof(KeywordNames) / sizeof(char*); i++) {
        if (strncmp(KeywordNames[i], start, length) == 0) {
            token->type = TOKEN_KEYWORD;
            break;
        }
    }
}

TokenList* lex(char* input) {
    printf("%s\n", input);
    Token* tokens = malloc(1000 * sizeof(Token));
    size_t tokenCount = 0;
    char* current = input;
    while (*current != '\0') {
        consume_whitespace(&current);

        if (*current == '(' || *current == ')' || *current == ',') {
            set_token(&tokens[tokenCount], TOKEN_SEPARATOR, current, 1);
            current++;

        } else if (*current == '=') {
            if (*(current + 1) == '>') {
                set_token(&tokens[tokenCount], TOKEN_OPERATOR, current, 2);
                current += 2;
            } else {
                set_token(&tokens[tokenCount], TOKEN_OPERATOR, current, 1);
                current++;
            }

        } else if (*current == '+' || *current == '-' || *current == '*' ||
                   *current == '/') {
            set_token(&tokens[tokenCount], TOKEN_OPERATOR, current, 1);
            current++;

        } else if (*current >= '0' && *current <= '9') {
            set_token(&tokens[tokenCount], TOKEN_INT, current, 1);
            current++;

        } else if (*current >= 'a' && *current <= 'z') {
            parse_word(&current, &tokens[tokenCount]);
        } else if (*current == ';') {
            set_token(&tokens[tokenCount], TOKEN_SEPARATOR, current, 1);
            current++;

        } else {
            printf("Unknown character: %c\n", *current);
            exit(1);
        }
        tokenCount++;
    }

    Token* usedTokens = realloc(tokens, tokenCount * sizeof(Token));
    if (usedTokens == NULL) {
        printf("Memory allocation failed\n");
        exit(1);
    }

    TokenList* tokenList = malloc(sizeof(TokenList));
    *tokenList = (TokenList){usedTokens, tokenCount};
    return tokenList;
}