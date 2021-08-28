# Flycatcher
Flycatcher is a high-level compiled programming language.  Flycatcher's main point is "constructs," which allow you to essentially change the programming language without any external modifications to the compiler.

We use the Cranelift code generator behind the scenes, which is a rather new code generator project made by the Bytecode Alliance, it compiles executables faster than LLVM, and it sometimes compiles more efficiently than LLVM as well.

> **note!** Flycatcher is being re-written with the intent to finish most of the initial goals set for it.  This will improve efficiency, functionality, syntax, etc.  The code will be written better and will be well documented, if you want to look through it.

## Hello, world!
```flycatcher
#include "std/io"

@func main() {
    println("Hello, world!");
}
```