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

## Syntax:

(very much a work in progress)

```Rust
import {print} from "std/fmt"

let x:i32 = 10;
let y:i32 = 20;

let add = (a:i32, b:i32):i32 => a + b;

let result = add(x, y);

print(result);
```

### Only work with functions
```Rust
import {create_server, listen, Request, Response} from "std/server"

// Callback function for /
let index = (req:Request, res:Response) => {
    res("Hello World!")
}

let server = create_server("localhost", 8080, {
    "/" => index
})

listen(server)
```

### No if-statements, only matching. 
This is still a work in progress and will need to be decided. There needs to be a very light-weight syntax.
```Rust
import {print} from "std/fmt"

let dog_breed:str = "Whippet";

let ability:str =
    dog_breed ? { 
      "Whippet"=>"run",
      "Husky"=>"pull"
    }   


```


## Milestones

- [x] Basic Lexer
- [ ] Basic Parser
- [ ] Functions, variables, i32, add, sub, mul, div
- [ ] i64, f32, f64
- [ ] match, map
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
