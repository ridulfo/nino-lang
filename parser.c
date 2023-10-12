#include <assert.h>
#include <stdio.h>
#include <string.h>

#include "lexer.h"

char* build_let(Token* token) {
    // let x = 5;
    assert(token->type == TOKEN_LET);
    token++;  // skip the LET token

    char* identifier = token->text;
    token++;  // consume the IDENTIFIER token

    assert(token->type == TOKEN_ASSIGNMENT);
    token++;  // skip the ASSIGNMENT token

    char* value = token->text;
    token++;  // consume the value token

    assert(token->type == TOKEN_END_STATEMENT);
    token++;  // consume the END_STATEMENT token

    char* output = malloc(100 * sizeof(char));
    if (output == NULL) {
        printf("Error: Could not allocate memory for output string.\n");
        return NULL;
    }

    sprintf(output,
            "  %%%s = alloca i32\n"
            "  store i32 %s, i32* %%%s\n",
            identifier, value, identifier);

    return output;
}

char* parser(TokenList* tokens) {
    char* output = malloc(1000 * sizeof(char));
    if (output == NULL) {
        printf("Error: Could not allocate memory for output string.\n");
        return NULL;
    }
    char* header_main_block =
        "define i32 @main() {\n"
        "entry:\n";

    char* footer_main_block =
        "  %return_value = load i32, i32* %return\n"
        "  ret i32 %return_value\n"
        "}\n"
        "\n";

    strcat(output, header_main_block);

    for (size_t i = 0; i < tokens->length; i++) {
        Token* current = &tokens->tokens[i];
        _print_token(current);
        if (current->type == TOKEN_LET) {
            char* let_output = build_let(current);
            strcat(output, let_output);
            free(let_output);
        }
    }

    strcat(output, footer_main_block);

    printf("\n%s", output);
    return output;
}

int main() {
    char* input = "let return = 5;";
    TokenList* tokens = lex(input);

    char* output = parser(tokens);

    char* output_file = "output.ll";
    FILE* fp = fopen(output_file, "w");
    if (fp == NULL) {
        printf("Error: Could not open file %s for writing.\n", output_file);
        return 1;
    }

    fprintf(fp, "%s", output);  // write output to file

    fclose(fp);  // close file pointer

    return 0;
}