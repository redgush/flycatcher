# Progress
This is the current progress made on the Flycatcher language.

> Currently, Flycatcher is being re-written, for optimization, stability and customization control.  More updates will come soon!

<!--
# Progress
This is the progress that we've made on the Flycatcher compiler, what we've finished, what's work in progress and what is still unstarted.

## Finished
- Lexer
- Parser

## In Progress
- The Flycatcher AST lowerer, which converts an AST tree into a HIR tree.  This also resolves imports/includes and ensures the semantic validity of the AST tree.

## To Do
- The HIR lowerer, which lowers a Flycatcher HIR tree into a typed HIR (THIR) tree.  This is where type verification occurs.
- The THIR lowerer, which lowers a Flycatcher THIR tree into a MIR module.  The MIR is as close as the compiler frontend gets to machine code, it doesn't provide abstractions for `if` and `while` statements, instead it uses Assembly-style conditional branches and jumps.  This pass is mostly for optimization and verification that the code is valid.
- Finally, the compiler's backend.  This will compile the MIR tree into an executable binary, using some sort of LIR (low-level intermediate representation).  The planned backend will use Cranelift, a code generator written in Rust.
- Clean up the source directory, moving all of the main compiler features to the `compiler` folder.  This is one of the least priorities, so this will most likely be done after the compiler is mostly stable.  This might not happen until the self-compiler (compiler for Flycatcher written in Flycatcher) is implemented.
-->
