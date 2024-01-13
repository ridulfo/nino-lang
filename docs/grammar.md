# Grammar

```rust
program -> item*
item -> import | declaration | expression

import -> "import" "{" (identifier | identifier ",")* "}" | "import" identifier "from" string_literal

declaration -> "let" identifier ":" type "=" expression ";"


identifier -> [a-zA-Z_][a-zA-Z_0-9]*
type -> "char" | "num" | "bool" | "fn" | array_type
array_type -> "[" type "]"

number_literal -> [0-9]+ | [0-9]+.[0-9]+
string_literal -> "\"" [a-zA-Z_0-9]* "\""
boolean_literal -> "true" | "false"
array_literal -> "[" (expression | expression ",")* "]"
function -> "(" (identifier | identifier ",")* ")" ":" type "=>" scoped_variables => expression ";"
scoped_variables -> "" | "|" identifier ":" type "=" expression ";" scoped_variables

expression -> equality
equality -> comparison ( ( "!=" | "==" ) comparison )*
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )*
term -> factor ( ( "-" | "+" ) factor )*
factor -> unary ( ( "/" | "*" | "mod" ) unary )*
unary -> ( "!" | "-" ) unary | primary
primary -> number_literal | string_literal | boolean_literal | "(" expression ")" | identifier | function | function_call | pattern_matching

function_call -> identifier "(" (expression | expression ",")* ")"
pattern_matching -> expression "?" "{" (pattern_match | pattern_match ",")* "}"
pattern_match -> expression "=>" expression
```
