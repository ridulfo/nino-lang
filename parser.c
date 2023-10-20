#include "parser.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "lexer.h"

void next_token_any(Token** current) {
    (*current)++;
    print_token(*current);
}

// Gets the next token and checks if it is of the expected type.
void next_token(Token** current, enum TokenType type) {
    next_token_any(current);
    if ((*current)->type != type) {
        printf("Parser Error - Expected token %s, got %s\n", TokenNames[type], TokenNames[(*current)->type]);
        exit(1);
    }
}

enum TokenType peek_token(Token** current, size_t offset) {
    return ((*current) + offset)->type;
}

Expression* parse_expression(Token** current);

Expression* parse_function(Token** current) {
    Expression* node = malloc(sizeof(Expression));
    node->type = AST_FUNCTION;
    Function* function = malloc(sizeof(Function));
    FunctionParameter* parameters = malloc(10 * sizeof(FunctionParameter));  // Max number of function parameters
    function->parameters = parameters;

    next_token(current, TOKEN_IDENTIFIER);
    while ((*current)->type != TOKEN_RPAREN) {
        FunctionParameter* parameter = &parameters[function->num_parameters];
        parameter->identifier = (*current)->text;
        parameter->identifier_length = (*current)->length;
        next_token(current, TOKEN_COLON);
        next_token(current, TOKEN_TYPE);
        parameter->type = (*current)->text;
        parameter->type_length = (*current)->length;
        function->num_parameters++;
        next_token_any(current);
    }

    next_token(current, TOKEN_COLON);
    next_token(current, TOKEN_TYPE);

    function->return_type = (*current)->text;
    function->return_type_length = (*current)->length;

    next_token(current, TOKEN_ARROW);
    next_token_any(current);

    function->expression = parse_expression(current);

    node->data.Function = *function;

    return node;
}

Expression* parse_primary(Token** current) {
    Expression* expr = malloc(sizeof(Expression));
    switch ((*current)->type) {
        case TOKEN_IDENTIFIER:
            expr->type = AST_IDENTIFIER;
            expr->data.Identifier.value = (*current)->text;
            expr->data.Identifier.length = (*current)->length;
            break;
        case TOKEN_LITERAL_INT:
            expr->type = AST_INTEGER_LITERAL;
            expr->data.Literal.type_name = "i32";
            expr->data.Literal.type_name_length = 3;
            expr->data.Literal.value = (*current)->text;
            expr->data.Literal.length = (*current)->length;
            break;
        case TOKEN_LPAREN:
            if (peek_token(current, 1) == TOKEN_IDENTIFIER && peek_token(current, 2) == TOKEN_COLON && peek_token(current, 3) == TOKEN_TYPE) {
                expr = parse_function(current);
            }
            break;
        default:
            printf("Unexpected primary token %s\n", TokenNames[(*current)->type]);
            exit(1);
            break;
    }
    next_token_any(current);
    return expr;
}

Expression* parse_unary(Token** current) {
    Expression* node = parse_primary(current);
    return node;
}
Expression* parse_factor(Token** current) {
    Expression* node = parse_unary(current);

    while ((*current)->type == TOKEN_MUL || (*current)->type == TOKEN_DIV) {
        Token* operator= * current;
        next_token_any(current);
        Expression* right = parse_unary(current);
        Expression* left = node;
        node = malloc(sizeof(Expression));
        node->type = AST_BINARY_OPERATION;
        node->data.BinaryOperation.left = left;
        node->data.BinaryOperation.right = right;
        node->data.BinaryOperation.operator= operator->text;
        node->data.BinaryOperation.operator_length = operator->length;
    }

    return node;
}
Expression* parse_term(Token** current) {
    Expression* node = parse_factor(current);

    while ((*current)->type == TOKEN_ADD || (*current)->type == TOKEN_SUB) {
        Token* operator= * current;
        next_token_any(current);
        Expression* right = parse_factor(current);
        Expression* left = node;
        node = malloc(sizeof(Expression));
        node->type = AST_BINARY_OPERATION;
        node->data.BinaryOperation.left = left;
        node->data.BinaryOperation.right = right;
        node->data.BinaryOperation.operator= operator->text;
        node->data.BinaryOperation.operator_length = operator->length;
    }
    return node;
}

Expression* parse_comparison(Token** current) {
    Expression* node = parse_term(current);
    return node;
}

Expression* parse_equality(Token** current) {
    Expression* node = parse_comparison(current);
    return node;
}

Expression* parse_expression(Token** current) {
    Expression* node = parse_equality(current);
    return node;
}

ASTNode* parse_declaration(Token** current) {
    ASTNode* node = malloc(sizeof(ASTNode));
    node->type = AST_DECLARATION;
    next_token(current, TOKEN_IDENTIFIER);  // identifier

    Declaration* declaration = malloc(sizeof(Declaration));
    declaration->identifier = (*current)->text;
    declaration->identifier_length = (*current)->length;

    next_token(current, TOKEN_COLON);  // colon

    next_token(current, TOKEN_TYPE);  // type
    declaration->type = (*current)->text;
    declaration->type_length = (*current)->length;

    next_token(current, TOKEN_ASSIGNMENT);  // assignment

    next_token_any(current);  // beginning of expression

    declaration->expression = parse_expression(current);

    node->data.Declaration = *declaration;

    return node;
}

ASTNode* parse_print(Token** current) {
    ASTNode* node = malloc(sizeof(ASTNode));
    node->type = AST_PRINT;

    next_token(current, TOKEN_LPAREN);  // (
    next_token_any(current);            // beginning of expression

    node->data.Print.expression = parse_expression(current);

    next_token(current, TOKEN_SEMICOLON);  // ;

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

    Token* current = tokens->tokens;

    while (current->type != TOKEN_EOF) {
        if (current->type == TOKEN_LET) {
            items[items_length++] = parse_declaration(&current);
        } else if (current->type == TOKEN_PRINT) {
            // TODO: Handle this as any other function
            items[items_length++] = parse_print(&current);
        } else {
            printf("Unexpected token %s\n", TokenNames[current->type]);
            exit(1);
        }
        if (current->type == TOKEN_EOF) break;

        next_token_any(&current);
    }

    ASTList* ast_list = malloc(sizeof(ASTList));
    ast_list->length = items_length;
    ast_list->items = items;

    return ast_list;
}