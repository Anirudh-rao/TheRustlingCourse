# Primitives

Rust provides access to a wide variety of primitives. A sample includes:

1. Scalar Types
    1. `signed integers`: i8, i16, i32, i64, i128 and isize (pointer size)
    2. `unsigned integers`: u8, u16, u32, u64, u128 and usize (pointer size)
    3. floating point: f32, f64
    4. char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
    5. bool either true or false
    and the unit type (), whose only possible value is an empty tuple: ()

Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.

2.  Compound Types
    1. arrays like [1, 2, 3]
    2. tuples like (1, true)

Variables can always be type annotated. Numbers may additionally be annotated via a suffix or by default. Integers default to i32 and floats to f64. Note that Rust can also infer types from context.

## Literals and operators

Integers `1`, floats `1.2`, characters `'a'`, strings `"abc"`, booleans true and the unit type `()` can be expressed using literals.

Integers can, alternatively, be expressed using hexadecimal, octal or binary notation using these prefixes respectively: `0x`, `0o` or `0b`.

Underscores can be inserted in numeric literals to improve readability, e.g. `1_000` is the same as `1000`, and `0.000_001` is the same as `0.000001`.

We need to tell the compiler the type of the literals we use. For now, we'll use the u32 suffix to indicate that the literal is an unsigned 32-bit integer, and the i32 suffix to indicate that it's a signed 32-bit integer.

The operators available and their precedence in Rust are similar to other C-like languages.

## Tuples

A tuple is a collection of values of different types. Tuples are constructed using parentheses (), and each tuple itself is a value with type signature (T1, T2, ...), where T1, T2 are the types of its members. Functions can use tuples to return multiple values, as tuples can hold any number of values.
