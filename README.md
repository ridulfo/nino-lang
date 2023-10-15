<div>
    <p align="center"><img src="logo.png" height="200px" width="200px" /></p>
    <h1 align="center">nino-lang</h1>
</div>

[![Run Tests](https://github.com/ridulfo/nino-lang/actions/workflows/on-main.yaml/badge.svg)](https://github.com/ridulfo/nino-lang/actions/workflows/on-main.yaml)

**Tiny speedy self-hosted compiled functional programming language**

Like the [Whippet](https://en.wikipedia.org/wiki/Whippet), this language [will have] the highest running speed of any [language] of its [size].

The goal is to create a small language that only has the essential features needed to be able to do pretty much anything. No bloat. This will make [self-hosting](<https://en.wikipedia.org/wiki/Self-hosting_(compilers)>) easier and faster.

The compiler will transpile source code into [LLVM IR](https://en.wikipedia.org/wiki/LLVM#Intermediate_representation) and then use [Clang](https://en.wikipedia.org/wiki/Clang) to compile it to native code. This will allow us to use the LLVM optimizer and get the best performance possible. Probably in the realm of C/C++/rust.

It will be a functional programming language with a syntax similar to [rust](<https://en.wikipedia.org/wiki/Rust_(programming_language)#Syntax_and_features>) and [oCaml](https://en.wikipedia.org/wiki/OCaml#Code_examples). The native types will be those of rust, and the level of side-effect pedanticness will be like oCaml. That is, side effects such as printing will not have to be wrapped in monads. There will be no member functions and variables.

It will not have a garbage collector (TBD 🤨).

## Current status

- 2023-10-13: Just finished defining the initial complete syntax. Next is to rewrite the lexer, parser and code generator to support the new syntax.
- 2023-10-15: Syntax has been reworked and a grammar definition can be found in [docs](docs/grammar.md). The lexer has been updated to support the new syntax and the parser has been completely rewritten as a recursive descent parser. A code generated has been implemented that can generate LLVM IR. The next steps are to implement more language features. See [milestones](#milestones) for more details.

```Rust

## Syntax (WIP)

Grammar definition can be found in [here](docs/grammar.md).

### Currently supported

```Rust
let x = 13;
let y = 14;
let z = x + y;

print(z);
```

**To run:**

1. `make parser`
1. `./build/parser`
1. `clang output.ll`
1. `./a.out`

### In the future...

```Rust
import {print} from "std/fmt";

let x:i32 = 10;
let y:i32 = 20;

fn add = (a:i32, b:i32):i32 => a + b;

let result:i32 = add(x, y);

print(result);
```

#### Only work with functions

```Rust
import {create_server, listen, Server, Request, Response} from "std/server";

// Callback function for /
fn index = (req:Request, res:Response):void => res("Hello World!")


let server: Server = create_server("localhost", 8080, {
    "/" => index
})

listen(server)
```

#### No if-statements, only matching.

This is still a work in progress and will need to be decided. There needs to be a very light-weight syntax.

`str` type is syntactic sugar for `[i8]`

```Rust
import {print} from "std/fmt";

let dog_breed:str = "Whippet";

let ability:str =
    dog_breed ? {
      "Whippet"=>"run",
      "Husky"=>"pull"
    }
```

#### is_prime

```Rust
import {sqrt, floor} from "std/math";

fn is_prime_helper = (x:i32, i:i32, sqrt_x_int:i32):bool =>
    true ? {
        i > sqrt_x_int => true,
        x mod i == 0 => false,
        true => is_prime_helper(x, i+2, sqrt_x_int)
    };

fn is_prime = (x:i32):bool =>
    | sqrt_x:f32 = sqrt(x);
    | sqrt_x_int:i32 = floor(sqrt_x);
    => true ? {
        x==1 => false,
        x==2 => true,
        x mod 2 == 0 => false,
        true => is_prime_helper(x, 3, sqrt_x_int)
    };
```

#### Built-in functions

**map**

```Rust
/** Caesar cipher
*
* Rotates lowercase strings by x
*/
fn rot = (x:i8, s:[i8]):[i8] =>
    map(s, (c:i8) => 'a' + (c - 'a' + x) mod 26);
```

## Milestones

- [x] Define complete syntax
- [x] Basic Lexer
- [x] Basic Parser
- [ ] built-in functions
  - [x] print
  - [ ] pattern-matching
  - [ ] map
  - [ ] filter
  - [ ] reduce
- [ ] arrays, strings
- [ ] self-host compiler
- [ ] [**SOLVE ADVENT OF CODE**](https://time-since.nicolo.io/#/20231201-000000?title=Advent+of+code)

## Requirements

- Clang
- make

## Tests

```bash
make test
```

## Limitations

What is this it, and what is it not?
The goal is not to create the next big language that everybody should use. This is more of an experiment in making a minimal compiled functional programming language.

I will work on this in my spare time. No guarantees can be made.
