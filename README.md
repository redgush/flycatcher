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

# Goals with Flycatcher
- One of Flycatcher's main goals is to provide as many high-level abstractions as possible without draining the performance and efficiency of the application.
- The other main goal is to produce small executables.  Flycatcher doesn't rely on the C standard library, unlike programming languages such as Rust (and C of course).  Also, because of the way Flycatcher works, it only links libraries that are used.  Normally, languages like Rust link with their **entire** standard library, making for an unnecessarily large executable since there is unused code all over the place.  Instead of this, we link several small system-specific wrappers where needed, allowing for much smaller executables.
- Simplicity and consistency are important for Flycatcher, as it's meant to be a high level programming language.  It isn't as advanced as languages such as Rust, nor will it ever be, but it will (hopefully) be just as powerful as any other systems programming languages.