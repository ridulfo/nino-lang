<div>
    <p align="center"><img src="logo.png" height="200px" width="200px" /></p>
    <h1 align="center">nino-lang</h1>
</div>

[![Run Tests](https://github.com/ridulfo/nino-lang/actions/workflows/on-main.yaml/badge.svg)](https://github.com/ridulfo/nino-lang/actions/workflows/on-main.yaml)

**Minimal interpreted functional programming language**

Like the [Whippet](https://en.wikipedia.org/wiki/Whippet), this language [will have] the highest running speed of any [language] of its [size].

The goal is to create a small language that only has the essential features needed to be able to do pretty much anything. No bloat. This will make [self-hosting](<https://en.wikipedia.org/wiki/Self-hosting_(compilers)>) easier and faster.

In this spirit, the under the hood type system is minimal. There are only 5 types:
- `num` - 64 bit floating point number - this is used for numbers
- `char` - 8 bit number - used to represent characters
- `bool` - 1-bit - used to represent true and false
- `fn` - functions are first-class citizens, this is their type
- `array` - an array of any type (even arrays)

The language is chill with side effects. There is no need to wrap anything in monads.

At the moment the language is interpreted, but the goal is to create a self-hosting compiler. TBD whether to compile to assembly or [LLVM IR](https://en.wikipedia.org/wiki/LLVM#Intermediate_representation).

There is no garbage collector (TBD 🤨).

## Quick start

**Compile the interpreter**

```bash
cargo build --release && mv target/release/ninoi .
```

**Run the example program**

```bash
./ninoi examples/is-prime.ni
```
<details>
    <summary>Output</summary>
    <pre>
Let's calculate primes!
10000019
10000079
10000103
10000121
10000139
10000141
10000169
10000189
Time in milliseconds:
47
    </pre>
</details>

### Bonus
There is an expression-to-AST-diagram program too!

```bash
cargo run --bin mermaid "1+2+3 + 1+2/3 + x mod y" > mermaid.md
```
<details>
    <summary>Output</summary>
    View this using a mermaid viewer.
    <img src="docs/mermaid.png"/>
</details>

## Documentation
Documentation can be found in the [docs](docs/).

There you can find the [language specification (WIP)](docs/language-specification.md) and the [grammar specification](docs/grammar.md).


### Examples

```rust
let is_prime_helper:fn = (x:num, i:num):bool => true ? {
    x==i => true,
    x mod i == 0 => false,
    is_prime_helper(x, i + 1)
};

let is_prime:fn = (x:num):bool => x ? {
    1 => false,
    2 => true,
    is_prime_helper(x, 2)
};

print(is_prime(2));
print(is_prime(3));
print(is_prime(4));
print(is_prime(5));
print(is_prime(6));
print(is_prime(7));
```

## Progress

- 2023-10-13: Just finished defining the initial complete syntax. Next is to rewrite the lexer, parser and code generator to support the new syntax.
- 2023-10-15: Syntax has been reworked and a grammar definition can be found in [docs](docs/grammar.md). The lexer has been updated to support the new syntax and the parser has been completely rewritten as a recursive descent parser. A code generated has been implemented that can generate LLVM IR. The next steps are to implement more language features. See [milestones](#milestones) for more details.
- 2023-10-17: Created compiler program
- 2023-10-22: Implemented declaring and calling functions. Function calls can be used as values in an expression. The next steps will need to be refactoring and adding unit tests.
- 2023-10-28: Re-wrote the lexer and parser to rust. Added tons of unit tests. Created an interpreter to run `.ni` files.
- 2023-10-29: Any programming language's most important features is correctness and safety. No need for more justification.
- 2023-10-04: Added tail-call optimization. More complex computations are now possible.
- 2024-01-13: Simplified type system to only have 5 types.
- 2024-01-14: Finished expression-to-AST-diagram, added more tests, added more support for arrays and improved printing.
- 2024-02-12: Interpreter now keeps symbols in [scoped-symbols](src/scoped_symbols.rs) leading to a 10x speed improvement compared to the initial symbol table.

## Milestones

- [x] Define complete syntax
- [x] numbers
- [x] characters
- [x] booleans
- [x] function declaration and call
- [x] arrays and strings
- [ ] built-in functions
    - [x] print
    - [x] matching
    - [ ] map
    - [ ] filter
    - [ ] reduce
- [ ] anonymous functions
- [ ] self-host compiler
- [ ] [**SOLVE ADVENT OF CODE**](https://time-since.nicolo.io/#/20231201-000000?title=Advent+of+code)

## Tests
In order to run the tests, simply run:

```bash
cargo test
```

## Limitations

What is this it, and what is it not?
The goal is not to create the next big language that everybody should use. This is more of an experiment in making a minimal compiled functional programming language.

I will work on this in my spare time. No guarantees can be made.
