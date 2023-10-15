#pragma once
#include <stdlib.h>

#include "lexer.h"

enum ASTNodeType {
    AST_IDENTIFIER,
    AST_INTEGER_LITERAL,
    AST_FLOAT_LITERAL,
    AST_STRING_LITERAL,
    AST_ARRAY_LITERAL,
    AST_ELEMENT_LIST,
    AST_PROGRAM,
    AST_ITEM_SEQUENCE,
    AST_ITEM,
    AST_TYPE,
    AST_ARRAY_TYPE,
    AST_DECLARATION,
    AST_FUNCTION_DECLARATION,
    AST_SCOPED_VARIABLES,
    AST_FUNCTION_CALL,
    AST_ARGUMENT_LIST,
    AST_EXPRESSION,
    AST_UNARY_EXPRESSION,
    AST_BINARY_OPERATION,
    AST_PATTERN_MATCHING,
    AST_PATTERN_MATCHES,
    AST_PATTERN_MATCH,
    AST_IMPORT,
    AST_IMPORT_LIST
} ASTNodeType;

static char* ASTNodeNames[] = {
    "AST_IDENTIFIER",
    "AST_INTEGER_LITERAL",
    "AST_FLOAT_LITERAL",
    "AST_STRING_LITERAL",
    "AST_ARRAY_LITERAL",
    "AST_ELEMENT_LIST",
    "AST_PROGRAM",
    "AST_ITEM_SEQUENCE",
    "AST_ITEM",
    "AST_TYPE",
    "AST_ARRAY_TYPE",
    "AST_DECLARATION",
    "AST_FUNCTION_DECLARATION",
    "AST_SCOPED_VARIABLES",
    "AST_FUNCTION_CALL",
    "AST_ARGUMENT_LIST",
    "AST_EXPRESSION",
    "AST_UNARY_EXPRESSION",
    "AST_BINARY_OPERATION",
    "AST_PATTERN_MATCH",
    "AST_PATTERN_MATCHES",
    "AST_PATTERN_MATCH",
    "AST_IMPORT",
    "AST_IMPORT_LIST",

};

typedef struct Declaration {
    char* identifier;
    size_t identifier_length;

    char* type;
    size_t type_length;

    struct Expression* expression;

} Declaration;

typedef struct Literal {
    enum ASTNodeType type;
    char* type_name;
    size_t type_name_length;
    char* value;
    size_t length;
} Literal;

typedef struct Identifier {
    enum ASTNodeType type;
    char* value;
    size_t length;
} Identifier;

typedef struct BinaryOperation {
    char* operator;
    size_t operator_length;

    struct Expression* left;
    struct Expression* right;
} BinaryOperation;

typedef struct Expression {
    enum ASTNodeType type;
    union {
        struct Identifier Identifier;
        struct Literal Literal;
        struct BinaryOperation BinaryOperation;
    } data;

} Expression;

typedef struct ASTNode {
    enum ASTNodeType type;
    union {
        struct Declaration Declaration;
    } data;

} ASTNode;

typedef struct ASTList {
    ASTNode** items;
    size_t length;
} ASTList;

void print_node(ASTNode* node);
ASTList* parse(TokenList* tokens);