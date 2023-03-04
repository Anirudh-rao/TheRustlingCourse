##  Variable Bindings

Rust provides type safety via static typing. Variable bindings can be type annotated when declared. However, in most cases, the compiler will be able to infer the type of the variable from the context, heavily reducing the annotation burden.

Values (like literals) can be bound to variables, using the let binding.

## Mutability

Variable bindings are immutable by default, but this can be overridden using the mut modifier.

## Scope and Shadowing

Variable bindings have a scope, and are constrained to live in a block. A block is a collection of statements enclosed by braces {}.

## Declare first

It's possible to declare variable bindings first, and initialize them later. However, this form is seldom used, as it may lead to the use of uninitialized variables.

## Freezing

When data is bound by the same name immutably, it also freezes. Frozen data can't be modified until the immutable binding goes out of scope