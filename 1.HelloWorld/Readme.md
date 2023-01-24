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


## Formated Input

Printing is handled by a series of macros defined in `std::fmt `some of which include:

1. format!: write formatted text to String
2. print!: same as format! but the text is printed to the console (io::stdout).
3. println!: same as print! but a newline is appended.
4. eprint!: same as print! but the text is printed to the standard error (io::stderr).
5. eprintln!: same as eprint! but a newline is appended.



`std::fmt` contains many traits which govern the display of text. The base form of two important ones are listed below:

1. fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
2. fmt::Display: Uses the {} marker. Format text in a more elegant, user friendly fashion.

Here, we used fmt::Display because the std library provides implementations for these types. To print text for custom types, more steps are required.

Implementing the fmt::Display trait automatically implements the `ToString` trait which allows us to convert the type to String.


### Debug
All types which want to use std::fmt formatting traits require an implementation to be printable. Automatic implementations are only provided for types such as in the `std library`. All others must be manually implemented somehow.

The `fmt::Debug` trait makes this very straightforward. All types can derive (automatically create) the `fmt::Debug` implementation. This is not true for fmt::Display which must be manually implemented.

```
/ This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`.
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);

```

### Display
`fmt::Debug` hardly looks compact and clean, so it is often advantageous to customize the output appearance. This is done by manually implementing ``fmt::Display``, which uses the {} print marker. Implementing it looks like this


### Formatting
We've seen that formatting is specified via a format string:

1. format!("{}", foo) -> "3735928559"
2. format!("0x{:X}", foo) -> "0xDEADBEEF"
3. format!("0o{:o}", foo) -> "0o33653337357"

The same variable (foo) can be formatted differently depending on which argument type is used: X vs o vs unspecified.

This formatting functionality is implemented via traits, and there is one trait for each argument type. The most common formatting trait is Display, which handles cases where the argument type is left unspecified: {} for instance.
