# Introduction To Rust Language

**Rust** is a modern systems programming language focusing on safety, speed, and concurrency. It accomplishes these goals by being memory safe without using garbage collection.

## Hello world in Rust

From the `HelloWorld.rs` File we can conclude the Following

1. `println!` is a macro that prints text to the console.

A binary can be generated using the Rust compiler: `rustc`.
```
$ rustc hello.rs

```

`rustc` will produce a hello binary that can be executed.
```
$ ./hello
```

**Hello World!** Can be seen at the output

## Comments
Any program requires comments, and Rust supports a few different varieties:

Regular comments which are ignored by the compiler:
1. // Line comments which go to the end of the line.
2. /* Block comments which go to the closing delimiter. */

Doc comments which are parsed into HTML library documentation:
1. /// Generate library docs for the following item.
2. //! Generate library docs for the enclosing item.