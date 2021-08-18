# Flycatcher
This is the Rust implementation of the Flycatcher programming language.

## About
Flycatcher is a programming language thats main goal is to be easy to use, while still being fast & efficient.  Of course, Flycatcher isn't in a "usable" stage of development, as only the lexer is complete and the parser is only partially finished.

Flycatcher is inspired by several other programming languages, including C/C++, JavaScript, Ruby, Python, Crystal and even a little bit of PHP.

### Examples
```c++
// Hello World Example
#include <io>

print("Hello, world!");
```

```c++
// Basic Function Example
#include <io>

@func my_function(my_argument) {
    print("Hello, " + my_argument.to_string() + "!");
}

my_function("world"); // => Hello, world!
my_function(true); // => Hello, true!
```

```c++
// Constructs Example
// A construct is similar to a class, but it can extend the
// way the programming language works in a way.
#include <io>

@construct my_construct {
    
    // This is called when the construct is,
    // well, constructed.
    public constructor(my_arg: uint64) {
        print(my_arg.to_string());
    }

}

my_object = @my_construct 42;
// => 42
```

### Goals
- The main goal with Flycatcher is to be safe to use, similar to Rust.  It will have useful checks to ensure high quality & safe code.
- Flycatcher should be easy to understand, yet still have the ability to do anything a lower level language, such as C can.

### Implementation
For the first revisions of Flycatcher, it will be transpiled to C or C++ (most likely C) code.  This is to iterate quicker with the Self Compiling Compiler™ which will be the second part of Flycatcher.

The self compiling compiler will compile to machine code rather than transpiling, it will be a much less rushed project, and more focus will go into managing efficiency of compilation and the outputted code.

## Roadmap
| Codebase | Crate | Description |
| - | - | - |
| `diagnostic` | `flycatcher-diagnostic` | A crate for dislaying diagnostic messages that are similar to Rustc's diagnostics to the console. |
| `flycatcher` | `flycatcher` | The command line tool for utilizing Flycatcher source. |
| `lexer` | `flycatcher-lexer` | A lexical analyzer that can take an input string and iterate through it, providing the different tokens that were found in the input string. |
| `parser` | `flycatcher-parser` | Flycatcher's parser, takes tokens from the `lexer` crate and converts them into an AST tree. |

## Progress
- Lexer, finished, stable, more features may be added.
- Parser, in progress, unstable