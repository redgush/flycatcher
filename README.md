# Flycatcher
Flycatcher is a general purpose, high-level, multi-paradigm, statically typed, compiled programming language.  The goals of Flycatcher are similar to that of Rust; create a safe, fast programming language that can offer the efficiency and ability of systems programming to programmers new and old.

> Flycatcher is not in a released alpha state yet, meaning it is not yet functional.  While some features may be finished, others may not be, such as the compiler's code generator.

# Goals
- Efficiently prevent the need to manually manage memory.  In the C language, explicit calls to `malloc`, `realloc` and `free` are very common, often necessary for even small programs.  The management of memory should happen at compile time, for maximum performance.
- It should be easy to learn, for programmers new and old.

## Examples
### Hello, world!
```flycatcher
@func main() {
    println("Hello, world!"); // => Hello, world!
}
```

### Duck Typing
Flycatcher is a statically "duck typed" programming language, which means "if it looks like a duck and quacks like a duck, it must be a duck."  It stems off of this concept quite a bit, for example:

```flycatcher
@type my_type = {
    value: uint64 // Unsigned 64-bit integer
}

@func example_function(obj: my_type) {
    println(obj.value);
}

@func main() {
    example_function({
        value: 42
    }); // => 42
}
```

### Statically Dispatched Dynamic Typing
> **This is not a confirmed feature of Flycatcher.**

This feature allows the compile time compilation of dynamic types, allows efficient dynamic typing, at the cost of executable size.

```flycatcher
// The `obj` parameter has no type annotation.
@func example_function(obj) {
    match obj {
        str: string => {
            // `obj` is a string.
            println(str);
        },
        str: Coerce<string> => {
            // `obj` can be implicitly converted into a string.
            println(str);
        },
        _ => {
            // No other types matched.
            println("Unknown type.");
        }
    }
}

@func main() {
    example_function("Hello, world!"); // => Hello, world!
    example_function(42); // => 42
}
```