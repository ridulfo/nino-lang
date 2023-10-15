#include "parser.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "lexer.h"

// Gets the next token and checks if it is of the expected type.
void next_token(Token** current, enum TokenType type) {
    (*current)++;
    if ((*current)->type != type) {
        printf("Expected token %s, got %s\n", TokenNames[type], TokenNames[(*current)->type]);
        exit(1);
    }
}

void next_token_any(Token** current) {
    (*current)++;
}

Expression* parse_primary(Token* current) {
    Expression* expr = malloc(sizeof(Expression));
    switch (current->type) {
        case TOKEN_IDENTIFIER:
            expr->type = AST_IDENTIFIER;
            expr->data.Identifier.value = current->text;
            expr->data.Identifier.length = current->length;
            break;
        case TOKEN_LITERAL_INT:
            expr->type = AST_INTEGER_LITERAL;
            expr->data.Literal.value = current->text;
            expr->data.Literal.length = current->length;
            break;
        // case TOKEN_LITERAL_FLOAT:
        //     expr->type = AST_FLOAT_LITERAL;
        //     break;
        // case TOKEN_LITERAL_STRING:
        //     expr->type = AST_STRING_LITERAL;
        //     break;
        // case TOKEN_LITERAL_ARRAY:
        //     expr->type = AST_ARRAY_LITERAL;
        //     break;
        default:
            printf("Unexpected primary token %s\n", TokenNames[current->type]);
            exit(1);
            break;
    }
    return expr;
}

Expression* parse_unary(Token* current) {
    Expression* node = parse_primary(current);
    return node;
}
Expression* parse_factor(Token* current) {
    Expression* node = parse_unary(current);
    return node;
}
Expression* parse_term(Token* current) {
    Expression* node = parse_factor(current);

    next_token_any(&current);
    while (current->type == TOKEN_ADD || current->type == TOKEN_SUB) {
        Token* operator = current;
        next_token_any(&current);
        Expression* right = parse_factor(current);
        Expression* left = node;
        node = malloc(sizeof(Expression));
        node->type = AST_BINARY_OPERATION;
        node->data.BinaryOperation.left = left;
        node->data.BinaryOperation.right = right;
        node->data.BinaryOperation.operator = operator->text;
        node->data.BinaryOperation.operator_length = operator->length;
    }
    return node;
}

Expression* parse_comparison(Token* current) {
    Expression* node = parse_term(current);
    return node;
}

Expression* parse_equality(Token* current) {
    Expression* node = parse_comparison(current);
    return node;
}

Expression* parse_expression(Token* current) {
    Expression* node = parse_equality(current);
    return node;
}

ASTNode* parse_declaration(Token* current) {
    ASTNode* node = malloc(sizeof(ASTNode));
    node->type = AST_DECLARATION;

    next_token(&current, TOKEN_IDENTIFIER);  // identifier

    Declaration* declaration = malloc(sizeof(Declaration));
    declaration->identifier = current->text;
    declaration->identifier_length = current->length;

    next_token(&current, TOKEN_COLON);  // colon
    assert(current->type == TOKEN_COLON);

    next_token(&current, TOKEN_TYPE);  // type
    declaration->type = current->text;
    declaration->type_length = current->length;

    next_token(&current, TOKEN_ASSIGNMENT);  // assignment
    assert(current->type == TOKEN_ASSIGNMENT);

    next_token_any(&current);  // beginning of expression

    declaration->expression = parse_expression(current);

    node->data.Declaration = *declaration;

    return node;
}

void rec_print_expr_tree(Expression* node, int depth) {
    for (int i = 0; i < depth; i++) {
        printf("  ");
    }
    printf("%s\n", ASTNodeNames[node->type]);
    if (node->type == AST_BINARY_OPERATION) {
        for (int i = 0; i < depth + 1; i++) {
            printf("  ");
        }
        printf("Operator: %s\n", node->data.BinaryOperation.operator);
        rec_print_expr_tree(node->data.BinaryOperation.left, depth + 1);
        rec_print_expr_tree(node->data.BinaryOperation.right, depth + 1);
    } else if (node->type == AST_INTEGER_LITERAL) {
        for (int i = 0; i < depth + 1; i++) {
            printf("  ");
        }
        printf("Value: %s\n", node->data.Literal.value);
    } else if (node->type == AST_IDENTIFIER) {
        for (int i = 0; i < depth + 1; i++) {
            printf("  ");
        }
        printf("Value: %s\n", node->data.Identifier.value);
    }
}

void rec_print_ast_tree(ASTNode* node, int depth) {
    for (int i = 0; i < depth; i++) {
        printf("  ");
    }
    printf("%s\n", ASTNodeNames[node->type]);
    if (node->type == AST_DECLARATION) {
        for (int i = 0; i < depth + 1; i++) {
            printf("  ");
        }
        printf("Identifier: %s\n", node->data.Declaration.identifier);
        for (int i = 0; i < depth + 1; i++) {
            printf("  ");
        }
        printf("Type: %s\n", node->data.Declaration.type);
        rec_print_expr_tree(node->data.Declaration.expression, depth + 1);
    }
}

ASTList* parse(TokenList* tokens) {
    ASTNode** items = malloc(1000 * sizeof(ASTNode));
    size_t items_length = 0;

    Token* current = &tokens->tokens[0];

    while (current != tokens->tokens + tokens->length) {
        if (current->type == TOKEN_LET) {
            items[items_length++] = parse_declaration(current);
        }
        current++;
    }
    printf("Abstract Syntax Tree:\n");
    rec_print_ast_tree(items[0], 0);

    ASTList* ast_list = malloc(sizeof(ASTList));
    ast_list->length = items_length;
    ast_list->items = items;

    return ast_list;
}