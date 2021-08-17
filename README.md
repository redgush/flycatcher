# Flycatcher
This is the Rust implementation of the Flycatcher programming language.

## Roadmap
| Codebase | Crate | Description |
| - | - | - |
| `diagnostic` | `flycatcher-diagnostic` | A crate for dislaying diagnostic messages that are similar to Rustc's diagnostics to the console. |
| `lexer` | `flycatcher-lexer` | A lexical analyzer that can take an input string and iterate through it, providing the different tokens that were found in the input string. |
| `parser` | `flycatcher-parser` | Flycatcher's parser, takes tokens from the `lexer` crate and converts them into an AST tree. |