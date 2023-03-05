## From and Into

The From and Into traits are inherently linked, and this is actually part of its implementation. If you are able to convert type A from type B, then it should be easy to believe that we should be able to convert type B to type A.

## From

The From trait allows for a type to define how to create itself from another type, hence providing a very simple mechanism for converting between several types. There are numerous implementations of this trait within the standard library for conversion of primitive and common types.

For example we can easily convert a str into a String

```
let my_str = "hello";
let my_string = String::from(my_str)

```

## Into

The Into trait is simply the reciprocal of the From trait. That is, if you have implemented the From trait for your type, Into will call it when necessary.

Using the `Into` trait will typically require specification of the type to convert into as the compiler is unable to determine this most of the time. However this is a small trade-off considering we get the functionality for free.

## TryFrom and TryInto

Similar to From and Into, TryFrom and TryInto are generic traits for converting between types. Unlike From/Into, the TryFrom/TryInto traits are used for fallible conversions, and as such, return Results.

## To and from Strings

### Converting to String

To convert any type to a String is as simple as implementing the ToString trait for the type. Rather than doing so directly, you should implement the fmt::Display trait which automagically provides ToString and also allows printing the type as discussed in the section on print!.

### Parsing a String

One of the more common types to convert a string into is a number. The idiomatic approach to this is to use the parse function and either to arrange for type inference or to specify the type to parse using the 'turbofish' syntax. Both alternatives are shown in the following example.

This will convert the string into the type specified as long as the FromStr trait is implemented for that type. This is implemented for numerous types within the standard library. To obtain this functionality on a user defined type simply implement the FromStr trait for that type.