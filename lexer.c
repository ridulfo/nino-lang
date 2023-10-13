#include "lexer.h"

#include <stddef.h>
#include <stdio.h>
#include <string.h>

char** KeywordNames = (char*[]){"let"};

Token* create_token(enum TokenType type, char* start, size_t length) {
    Token* token = malloc(sizeof(Token));

    token->type = type;
    token->length = length;
    token->text = malloc(length + 1);

    strncpy(token->text, start, length);
    token->text[length] = '\0';

    return token;
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

Token* parse_word(char** input) {
    size_t length = 0;
    char* start = *input;
    while (**input >= 'a' && **input <= 'z') {
        (*input)++;
        length++;
    }
    Token* token = create_token(TOKEN_IDENTIFIER, start, length);

    // check if the word is a keyword
    if (strcmp(token->text, "let") == 0) {
        token->type = TOKEN_LET;
    } else if (strcmp(token->text, "fn") == 0) {
        token->type = TOKEN_FN;
    } else if (strcmp(token->text, "print") == 0) {
        token->type = TOKEN_PRINT;
    }

    return token;
}

Token* parse_number(char** input) {
    size_t length = 0;
    char* start = *input;
    while (**input >= '0' && **input <= '9') {
        (*input)++;
        length++;
    }
    Token* token = create_token(TOKEN_INT, start, length);
    return token;
}

TokenList* lex(char* input) {
    printf("%s\n", input);
    Token* tokens = malloc(1000 * sizeof(Token));
    if (tokens == NULL) {
        printf("Memory allocation failed\n");
        exit(1);
    }

    size_t tokenCount = 0;
    char* current = input;
    while (*current != '\0') {
        consume_whitespace(&current);

        if (*current == '(') {
            tokens[tokenCount] = *create_token(TOKEN_LPAREN, current, 1);
            current++;

        } else if (*current == ')') {
            tokens[tokenCount] = *create_token(TOKEN_RPAREN, current, 1);
            current++;

        } else if (*current == ',') {
            tokens[tokenCount] = *create_token(TOKEN_COMMA, current, 1);
            current++;
        } else if (*current == '!') {
            if (*(current + 1) == '=') {
                tokens[tokenCount] = *create_token(TOKEN_NOTEQUAL, current, 2);
                current += 2;
            } else {
                printf("Unknown character: %c\n", *current);
                exit(1);
            }

        } else if (*current == '=') {
            if (*(current + 1) == '=') {
                tokens[tokenCount] = *create_token(TOKEN_EQUAL, current, 2);
                current += 2;
            } else if (*(current + 1) == '>') {
                tokens[tokenCount] = *create_token(TOKEN_ARROW, current, 2);
                current += 2;
            } else {
                tokens[tokenCount] = *create_token(TOKEN_ASSIGNMENT, current, 1);
                current++;
            }

        } else if (*current == '+') {
            tokens[tokenCount] = *create_token(TOKEN_ADD, current, 1);
            current++;

        } else if (*current == '-') {
            tokens[tokenCount] = *create_token(TOKEN_SUB, current, 1);
            current++;

        } else if (*current == '*') {
            tokens[tokenCount] = *create_token(TOKEN_MUL, current, 1);
            current++;

        } else if (*current == '/') {
            tokens[tokenCount] = *create_token(TOKEN_DIV, current, 1);
            current++;

        } else if (*current >= '0' && *current <= '9') {
            tokens[tokenCount] = *parse_number(&current);

        } else if (*current >= 'a' && *current <= 'z') {
            tokens[tokenCount] = *parse_word(&current);

        } else if (*current == ';') {
            tokens[tokenCount] = *create_token(TOKEN_SEMICOLON, current, 1);
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
