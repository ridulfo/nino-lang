# Grammar

```rust
program -> item*
item -> import | declaration | expression

import -> "import" "{" (identifier | identifier ",")* "}" | "import" identifier "from" string_literal

declaration -> "let" identifier ":" type "=" expression ";"

scoped_variables -> "" | "|" identifier ":" type "=" expression ";" scoped_variables

identifier -> [a-zA-Z_][a-zA-Z_0-9]*
type -> "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "f32" | "f64" | "bool" | "void" | array_type | "fn"
array_type -> "[" type "]"

integer_literal -> [0-9]+
float_literal -> [0-9]+.[0-9]+
string -> "\"" [a-zA-Z_0-9]* "\""
array_literal -> "[" (expression | expression ",")* "]"
function -> "(" (identifier | identifier ",")* ")" ":" type "=>" scoped_variables => expression ";"

expression -> equality
equality -> comparison ( ( "!=" | "==" ) comparison )*
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )*
term -> factor ( ( "-" | "+" ) factor )*
factor -> unary ( ( "/" | "*" | "mod" ) unary )*
unary -> ( "!" | "-" ) unary | primary
primary -> number | string | "true" | "false" | "nil" | "(" expression ")" | identifier | function | function_call | pattern_matching

function_call -> identifier "(" (expression | expression ",")* ")"
pattern_matching -> expression "?" "{" (pattern_match | pattern_match ",")* "}"
pattern_match -> expression "=>" expression

```
