# Custom Types in Rust

## Structures :

There are three types of structures ("structs") that can be created using the struct keyword:

1. Tuple structs, which are, basically, named tuples.
2. The classic C structs
3. Unit structs, which are field-less, are useful for generics.


## Enums:

The enum keyword allows the creation of a type which may be one of a few different variants. Any variant which is valid as a struct is also valid as an enum.


## use

The use declaration can be used so manual scoping isn't needed


## C-like

enum can also be used as C-like enums.

## constants
Rust has two different types of constants which can be declared in any scope including global. Both require explicit type annotation:

1. `const`: An unchangeable value (the common case).
2. `static`: A possibly mutable variable with 'static lifetime. 

The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is `unsafe`.