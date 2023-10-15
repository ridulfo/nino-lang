#include <assert.h>
#include <stdio.h>
#include <string.h>

#include "lexer.h"
#include "parser.h"

char* printing_header =
    "@.int_str = private unnamed_addr constant [4 x i8] c\"%d\\0A\\00\"\n"
    "declare i32 @printf(i8*, ...)\n\n";

char* printing_function(char* identifier) {
    char* output = malloc(100 * sizeof(char));
    if (output == NULL) {
        printf("Error: Could not allocate memory for output string.\n");
        return NULL;
    }

    sprintf(output,
            "  \n"
            "  %%fmt_val = load i32, i32* %%%s\n"
            "  %%fmt = getelementptr inbounds [4 x i8], [4 x i8]* @.int_str, i32 0, i32 0\n"
            "  call i32 (i8*, ...) @printf(i8* %%fmt, i32 %%fmt_val)\n\n",
            identifier);

    return output;
}

char* build_print(Token* token) {
    // print x;
    assert(token->type == TOKEN_PRINT);
    token++;  // skip the PRINT token

    assert(token->type == TOKEN_LPAREN);
    token++;  // skip the SEPARATOR token

    assert(token->type == TOKEN_IDENTIFIER);
    char* identifier = token->text;
    token++;  // consume the IDENTIFIER token

    assert(token->type == TOKEN_RPAREN);
    token++;  // skip the SEPARATOR token

    assert(token->type == TOKEN_SEMICOLON);
    token++;  // consume the END_STATEMENT token

    char* output = malloc(100 * sizeof(char));
    if (output == NULL) {
        printf("Error: Could not allocate memory for output string.\n");
        return NULL;
    }

    return printing_function(identifier);
}

// Generate variable names for temporary variables
char* id_gen() {
    static int id = 0;
    char* output = malloc(100 * sizeof(char));
    if (output == NULL) {
        printf("Error: Could not allocate memory for output string.\n");
        return NULL;
    }
    sprintf(output, "%d", id);
    id++;
    return output;
}

/**
 * @brief Build an expression
 *
 * @param identifier the identifier given by the parent expression
 */
char* build_expression(char* identifier, Expression* expression, char* output) {
    if (expression->type == AST_INTEGER_LITERAL) {
        char* type = expression->data.Literal.type_name;
        char* value = expression->data.Literal.value;

        sprintf(output,
                "  %%%s = alloca %s\n"
                "  store %s %s, %s* %%%s\n\n",
                identifier, type, type, value, type, identifier);
        return identifier;
    }

    if (expression->type == AST_IDENTIFIER) {
        char* identifier = expression->data.Identifier.value;
        strcat(output, identifier);
        return identifier;
    }

    if (expression->type == AST_BINARY_OPERATION) {
        // The id to be used for the left and right expressions if they do not return a identifier
        char* left_expr_id = malloc(100 * sizeof(char));
        strcpy(left_expr_id, identifier);
        left_expr_id = strcat(left_expr_id, id_gen());

        char* right_expr_id = malloc(100 * sizeof(char));
        strcpy(right_expr_id, identifier);
        right_expr_id = strcat(right_expr_id, id_gen());

        char* left_ptr_id = build_expression(left_expr_id, expression->data.BinaryOperation.left, output);
        char* right_ptr_id = build_expression(right_expr_id, expression->data.BinaryOperation.right, output);

        char* left_load_id = malloc(100 * sizeof(char));
        strcpy(left_load_id, identifier);
        left_load_id = strcat(left_load_id, id_gen());

        char* right_load_id = malloc(100 * sizeof(char));
        strcpy(right_load_id, identifier);
        right_load_id = strcat(right_load_id, id_gen());

        if (output == NULL) {
            printf("Error: Could not allocate memory for output string.\n");

        }
        sprintf(output,
                "  %%%s = load i32, i32* %%%s\n"
                "  %%%s = load i32, i32* %%%s\n"
                "  %%%s = %s i32 %%%s, %%%s\n",
                left_load_id, left_ptr_id,
                right_load_id, right_ptr_id,
                identifier, "add", left_load_id, right_load_id);

    }
    return NULL;
}

char* build_let(Declaration* declaration) {
    // let x = 5;

    char* output = malloc(100 * sizeof(char));
    if (output == NULL) {
        printf("Error: Could not allocate memory for output string.\n");
        return NULL;
    }

    Expression* expression = declaration->expression;
    build_expression(declaration->identifier, expression, output);

    return output;
}

char* code_gen(ASTList* ast_list) {
    char* output = malloc(1000 * sizeof(char));
    if (output == NULL) {
        printf("Error: Could not allocate memory for output string.\n");
        return NULL;
    }
    char* header_main_block =
        "define i32 @main() {\n"
        "entry:\n\n";

    char* footer_main_block =
        "  ret i32 0\n"
        "}\n"
        "\n";

    strcat(output, printing_header);
    strcat(output, header_main_block);

    for (int i = 0; i < (int)ast_list->length; i++) {
        ASTNode* node = ast_list->items[i];
        if (node->type == AST_DECLARATION) {
            strcat(output, build_let(&node->data.Declaration));
        }
    }

    strcat(output, footer_main_block);

    return output;
}
