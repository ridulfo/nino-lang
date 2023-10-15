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

char* build_let(Declaration* declaration) {
    // let x = 5;

    char* output = malloc(100 * sizeof(char));
    if (output == NULL) {
        printf("Error: Could not allocate memory for output string.\n");
        return NULL;
    }


    char* identifier = declaration->identifier;
    char* value = declaration->expression->data.Literal.value;
    char* type = declaration->type;
    

    sprintf(output,
            "  %%%s = alloca %s\n"
            "  store %s %s, %s* %%%s\n",
            identifier, type, type, value, type, identifier);

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
        "entry:\n";

    char* footer_main_block =
        "  ret i32 0\n"
        "}\n"
        "\n";

    strcat(output, printing_header);
    strcat(output, header_main_block);

    ASTNode* node = ast_list->items[0];

    strcat(output, build_let(&node->data.Declaration));

    strcat(output, footer_main_block);

    return output;
}
