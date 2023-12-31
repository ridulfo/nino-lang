#pragma once

#include <stdlib.h>

#include "lexer.h"

typedef enum ASTNodeType {
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
    AST_FUNCTION,
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
    AST_IMPORT_LIST,
    AST_PRINT,

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
    "AST_FUNCTION",
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
    "AST_PRINT"};

typedef struct Declaration {
    char* identifier;
    size_t identifier_length;

    char* type;
    size_t type_length;

    struct Expression* expression;

} Declaration;

typedef struct FunctionParameter {
    char* identifier;
    size_t identifier_length;

    char* type;
    size_t type_length;
} FunctionParameter;

typedef struct Function {
    FunctionParameter* parameters;
    size_t num_parameters;

    char* return_type;
    size_t return_type_length;

    struct Expression* expression;

} Function;

typedef struct FunctionCall{
    char* identifier;
    size_t identifier_length;

    struct Expression* arguments;
    size_t num_arguments;
} FunctionCall;

typedef struct Match {
    struct Expression* expression;

    struct Expression* patterns;
    size_t num_patterns;

    struct Expression* values;
    size_t num_values;
} Match;

typedef struct Print {
    struct Expression* expression;
} Print;

typedef struct Literal {
    ASTNodeType type;
    char* type_name;
    size_t type_name_length;
    char* value;
    size_t length;
} Literal;

typedef struct Identifier {
    ASTNodeType type;
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
    ASTNodeType type;
    union {
        struct Identifier Identifier;
        struct Literal Literal;
        struct BinaryOperation BinaryOperation;
        struct Function Function;
        struct FunctionCall FunctionCall;
        struct Match Match;
    } data;

} Expression;

typedef struct ASTNode {
    ASTNodeType type;
    union {
        struct Declaration Declaration;
        struct Expression Expression;
        struct Print Print;
    } data;

} ASTNode;

typedef struct ASTList {
    ASTNode** items;
    size_t length;
} ASTList;

void print_node(ASTNode* node);
ASTList* parse(TokenList* tokens);

void rec_print_expr_tree(Expression* node, int depth);