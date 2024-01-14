# Nino-lang Specification

## Types 
Nino has only got five data types, four primitive and one non-primitive. 

### Primitives
- Number
- Character
- Boolean
- Function

#### Number
Keyword: `num`.
A number is stored as a 64 bit floating point number according to the IEEE 754 specification.
This data type should be used for all numeric computations.

#### Character
Keyword: `char`.
A 8 bit integer used to store a single ASCII character.
Characters are what make up a string.

#### Boolean
Keyword: `bool`.
A type used to store `true` or `false`. Behind the scenes, it is use a byte.

#### Function
Keyword: `fn`.
Functions are different from the above types because they do not represent a value. Rather, they map one value to another through some sort of computation.
However, like other types, arrays can contain functions and they can also be passed to other functions.

### Non-primitives
- arrays

#### Array
An array is a collection of zero or more of any of the above types. It is the only non-primitive type in Nino.

## A program
A program is composed of zero or more items. An item is either the importing of a symbol found in another file, a declaration or an expression.

### Importing (TBD)
**How this language feature should work and what the syntax should be is not fully worked out yet.**

Symbols can be imported from other files using importing.

Option 1: 
`from "math.ni" import pow;`
Option 2:
`include "math.ni"`
Option 3:
`let pow = import("math.ni:pow")`

### Expression
Everything in Nino (except for a declaration) is an expresion, even `print`! Expressions are always evaluated immediately, no laziness.

### Declaration
Also known as binding, is when an expression is bound to a symbol. In order to do this the `let` keyword is used. This is followed by the symbol that the expression should be bound to. Here is the complete syntax:
```typescript
let x = 1;
let y = 1 + x;
```

### Match-expression
The language does not have `if` and `else` statements. Instead, the same feature is used for all types of conditionals. That feature is called match expression. This is its syntax:
```typescript
condition ? {
	expr_1a => expr_1b,
	expr_2a => expr_2b,
	expr_3
};
```
First is the `condition`, that is the expression that will be matched on. For example, if `condition` is equal to `expr_1a`, then the whole match-expression is evaluated to `expr_1b`. If that one does not match, then equality is checked with `expr_2b`, and so on. If non of them are equal to `condition`, then the match-expression is evaluated to `expr_3`. Also known as the default value.

This syntax can be used to replicate the behavior of an `if-else` statement like this:
```typescript
let result = condition ? {
	expr_1a => expr_1b,
	expr_2
};
```

## Is prime
```typescript
let is_prime_helper:fn = (x:i32, i:i32):bool => true ? {
    x==i => true,
    x mod i == 0 => false,
    is_prime_helper(x, i + 1)
};

let is_prime:fn = (x:i32):bool => x ? {
    1 => false,
    2 => true,
    is_prime_helper(x, 2)
};

print(is_prime(2));
```

## Arrays
```typescript
let message1 = "Hello world!"; // Optionally: message1:[char] = 
let message2 = ['H','e','l','l','o',' ','w','o','r','l','d','!'];
message1 == message2; // Returns true

let inc:fn = (x:num):num => x + 1;
let arr = [1, 2, 3];
let arr2 = arr M> inc;
arr2 == [2, 3, 4] // Returns true

let is_odd:fn = (x:num):num => x%2==0;
let numbers = [1, 2, 3]; // Optionally: numbers:[num] = 
let odds = numbers F> isOdd;

let sum_red:fn = (acc: num, curr: num): num => acc + curr;
let sum = numbers R> sum_red; 
```
