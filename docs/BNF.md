# Program

```BNF
# Identifier
<identifier> ::= [a-zA-Z_][a-zA-Z_0-9]*

# Literal
<literal> ::= <integer_literal> | <float_literal> | <string_literal> | <array_literal>

## Number Literals
<integer_literal> ::= [0-9]+
<float_literal> ::= [0-9]+.[0-9]+

## String Literal
<string_literal> ::= "\"" [a-zA-Z_0-9]* "\""

## Array Literal
<array_literal> ::= "[" <element_list> "]"
<element_list> ::= <expression> | <expression> "," <element_list>

# Program
<program> ::= <item_sequence>

# Item Sequence
<item_sequence> ::= <item> | <item> <item_sequence>
<item> ::= <declaration> | <function_call>

# Types
<type> ::= "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "f32" | "f64" | "bool" | "void" | <array_type>
<array_type> ::= "[" <type> "]"

# Declaration
<declaration> ::= "let" <identifier> ":" <type> "=" <expression> ";"
<function_declaration> ::= "fn" <identifier> = "(" <argument_list> ")" ":" <type> "=>" <scoped_variables> => <expression> ";"
<scoped_variables> ::= "" | "|" <identifier> ":" <type> "=" <expression> ";" <scoped_variables> # optional

# Function
<function_call> ::= <identifier> "(" <argument_list> ")" ";"
<argument_list> ::= <expression> | <expression> "," <argument_list>

# Expression
<expression> ::= <identifier> | <literal> | <binary_operation> | <unary_expression> | <function_call> | <pattern_match>
<unary_expression> ::= "!" "(" <expression> ")" | "-" "(" <expression> ")"
<binary_operation> ::= <expression> <binary_operation> <expression>
<binary_operator>  ::= "+" | "-" | "*" | "/"

# Pattern-matching expression
<pattern_match> ::= <expression> "?" "{" <pattern_matches> "}"
<pattern_matches> ::= <pattern_match> | <pattern_match> "," <pattern_matches>
<pattern_match> ::= <expression> "=>" <expression>

# import
<import> ::= "import" "{" <import_list> "}" | "import" <identifier> "from" <string_literal>
<import_list> ::= <identifier> | <identifier> "," <import_list>
```
