## Types

Rust provides several mechanisms to change or define the type of primitive and user defined types. The following sections cover:

1. Casting between primitive types
2. Specifying the desired type of literals
3. Using type inference
4. Aliasing types

## Literals

Numeric literals can be type annotated by adding the type as a suffix. As an example, to specify that the literal 42 should have the type i32, write 42i32.

The type of unsuffixed numeric literals will depend on how they are used. If no constraint exists, the compiler will use i32 for integers, and f64 for floating-point numbers.

There are some concepts used in the previous code that haven't been explained yet, here's a brief explanation for the impatient readers:

`std::mem::size_of_val` is a function, but called with its full path. Code can be split in logical units called `modules`. In this case, the `size_of_val` function is defined in the mem module, and the mem module is defined in the `std`crate. For more details, see modules and crates.

## Inference

The type inference engine is pretty smart. It does more than looking at the type of the value expression during an initialization. It also looks at how the variable is used afterwards to infer its type. 

## Aliasing

The type statement can be used to give a new name to an existing type. Types must have UpperCamelCase names, or the compiler will raise a warning. The exception to this rule are the primitive types: usize, f32, et