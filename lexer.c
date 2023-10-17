#include "lexer.h"

#include <stddef.h>
#include <stdio.h>
#include <string.h>

Token* create_token(enum TokenType type, char* start, size_t length) {
    Token* token = malloc(sizeof(Token));

    token->type = type;
    token->length = length;
    token->text = malloc(length + 1);

    strncpy(token->text, start, length);
    token->text[length] = '\0';

    return token;
}

void print_token(Token* token) {
    printf("Value: %s, ", token->text);
    printf("Type: %s\n", TokenNames[token->type]);
}

// consumes all whitespace characters from the input
void consume_whitespace(char** input) {
    while (**input == ' ' || **input == '\n' || **input == '\t' || **input == '\r' || **input == '\n') {
        (*input)++;
    }
}

Token* parse_identifier(char** input) {
    size_t length = 0;
    char* start = *input;

    while (('a' <= **input && **input <= 'z') || ('0' <= **input && **input <= '9') || **input == '_') {
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
    } else if (strcmp(token->text, "mod") == 0) {
        token->type = TOKEN_MOD;
    } else if (strcmp(token->text, "true") == 0 || strcmp(token->text, "false") == 0) {
        token->type = TOKEN_LITERAL_BOOL;
    }

    return token;
}

Token* parse_number(char** input) {
    size_t length = 0;
    char* start = *input;
    char isFloat = 0;

    while ('0' <= **input && **input <= '9') {
        (*input)++;
        length++;
        if (**input == '.') {
            isFloat = 1;
            (*input)++;
            length++;
        }
    }
    Token* token = create_token(isFloat ? TOKEN_LITERAL_FLOAT : TOKEN_LITERAL_INT, start, length);
    return token;
}

Token* parse_string(char** input) {
    size_t length = 0;
    char* start = *input;
    (*input)++;

    // TODO escape characters
    while (**input != '"') {
        (*input)++;
        length++;
    }
    (*input)++;

    Token* token = create_token(TOKEN_LITERAL_STRING, start, length);
    return token;
}

Token* parse_type(char** input) {
    size_t length = 0;
    char* start = *input;
    while (('a' <= **input && **input <= 'z') || ('0' <= **input && **input <= '9')) {
        (*input)++;
        length++;
    }
    Token* token = create_token(TOKEN_TYPE, start, length);

    return token;
}

TokenList* lex(char* input) {
    Token* tokens = malloc(1000 * sizeof(Token));
    if (tokens == NULL) {
        printf("Lexer Error - Memory allocation failed\n");
        exit(1);
    }

    size_t tokenCount = 0;
    char* current = input;
    while (*current != '\0') {
        consume_whitespace(&current);

        if (*current == '(') {
            tokens[tokenCount++] = *create_token(TOKEN_LPAREN, current++, 1);

        } else if (*current == ')') {
            tokens[tokenCount++] = *create_token(TOKEN_RPAREN, current++, 1);

        } else if (*current == '[') {
            tokens[tokenCount++] = *create_token(TOKEN_LBRACKET, current++, 1);

        } else if (*current == '[') {
            tokens[tokenCount++] = *create_token(TOKEN_RBRACKET, current++, 1);

        } else if (*current == '{') {
            tokens[tokenCount++] = *create_token(TOKEN_LBRACE, current++, 1);

        } else if (*current == '}') {
            tokens[tokenCount++] = *create_token(TOKEN_RBRACE, current++, 1);

        } else if (*current == ',') {
            tokens[tokenCount++] = *create_token(TOKEN_COMMA, current++, 1);

        } else if (*current == '!') {
            if (*(current + 1) == '=') {
                tokens[tokenCount++] = *create_token(TOKEN_NOTEQUAL, current++, 2);
                current += 2;
            } else {
                tokens[tokenCount++] = *create_token(TOKEN_NOT, current++, 1);
            }

        } else if (*current == '=') {
            if (*(current + 1) == '=') {
                tokens[tokenCount++] = *create_token(TOKEN_EQUAL, current++, 2);
                current += 1;
            } else if (*(current + 1) == '>') {
                tokens[tokenCount++] = *create_token(TOKEN_ARROW, current++, 2);
                current += 1;
            } else {
                tokens[tokenCount++] = *create_token(TOKEN_ASSIGNMENT, current++, 1);
            }
        } else if (*current == '+') {
            tokens[tokenCount++] = *create_token(TOKEN_ADD, current++, 1);

        } else if (*current == '-') {
            tokens[tokenCount++] = *create_token(TOKEN_SUB, current++, 1);

        } else if (*current == '*') {
            tokens[tokenCount++] = *create_token(TOKEN_MUL, current++, 1);

        } else if (*current == '/') {
            tokens[tokenCount++] = *create_token(TOKEN_DIV, current++, 1);

        } else if (*current == '<') {
            if (*(current + 1) == '=') {
                tokens[tokenCount++] = *create_token(TOKEN_LETHAN, current++, 2);
                current += 2;
            } else {
                tokens[tokenCount++] = *create_token(TOKEN_LTHAN, current++, 1);
            }
        } else if (*current == '>') {
            if (*(current + 1) == '=') {
                tokens[tokenCount++] = *create_token(TOKEN_GETHAN, current++, 2);
                current += 2;
            } else {
                tokens[tokenCount++] = *create_token(TOKEN_GTHAN, current++, 1);
            }
        } else if (*current >= '0' && *current <= '9') {
            tokens[tokenCount++] = *parse_number(&current);

        } else if (*current >= 'a' && *current <= 'z') {
            tokens[tokenCount++] = *parse_identifier(&current);

        } else if (*current == '"') {
            tokens[tokenCount++] = *parse_string(&current);

        } else if (*current == ':') {
            tokens[tokenCount++] = *create_token(TOKEN_COLON, current++, 1);

            consume_whitespace(&current);
            tokens[tokenCount++] = *parse_type(&current);

        } else if (*current == ';') {
            tokens[tokenCount++] = *create_token(TOKEN_SEMICOLON, current++, 1);

        } else if (*current == '|') {
            tokens[tokenCount++] = *create_token(TOKEN_PIPE, current++, 1);

        } else if (*current == '?') {
            tokens[tokenCount++] = *create_token(TOKEN_QUESTION, current++, 1);

        } else {
            if (*current == '\0') break;

            printf("Lexer Error - Unknown character: %c. ASCII: %d.\n", *current, *current);
            exit(1);
        }
    }

    tokens[tokenCount] = (Token){TOKEN_EOF, "", 0};

    TokenList* tokenList = malloc(sizeof(TokenList));
    *tokenList = (TokenList){tokens, tokenCount};
    return tokenList;
}
