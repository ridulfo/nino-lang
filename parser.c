#include <assert.h>
#include <stdio.h>
#include <string.h>

#include "lexer.h"

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

    assert(token->type == TOKEN_SEPARATOR);
    token++;  // skip the SEPARATOR token

    char* identifier = token->text;
    token++;  // consume the IDENTIFIER token
    
    assert(token->type == TOKEN_SEPARATOR);
    token++;  // skip the SEPARATOR token

    assert(token->type == TOKEN_END_STATEMENT);
    token++;  // consume the END_STATEMENT token

    char* output = malloc(100 * sizeof(char));
    if (output == NULL) {
        printf("Error: Could not allocate memory for output string.\n");
        return NULL;
    }

    return printing_function(identifier);
}

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
        "  ret i32 0\n"
        "}\n"
        "\n";

    strcat(output, printing_header);
    strcat(output, header_main_block);

    for (size_t i = 0; i < tokens->length; i++) {
        Token* current = &tokens->tokens[i];
        if (current->type == TOKEN_LET) {
            char* let_output = build_let(current);
            strcat(output, let_output);
            free(let_output);
        } else if (current->type == TOKEN_PRINT) {
            char* print_output = build_print(current);
            strcat(output, print_output);
            free(print_output);
        }
    }

    strcat(output, footer_main_block);

    printf("\n%s", output);
    return output;
}

int main() {
    char* input = "let x = 13;"
                  "let y = 14;"
                  "print(y);";
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