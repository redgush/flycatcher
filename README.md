# Flycatcher
Flycatcher is a high-level compiled programming language.  Flycatcher's main point is "constructs," which allow you to essentially change the programming language without any external modifications to the compiler.

We use Cranelift behind the scenes which compiles faster than LLVM, as well as produces more efficient debug builds in some cases.

> **note!** Flycatcher is being re-written as the previous code base here was experimental.  This rework will improve documentation and notes provided in the source code.

## Hello, world!
```flycatcher
#include "std/io"

@func main() {
    println("Hello, world!");
}
```