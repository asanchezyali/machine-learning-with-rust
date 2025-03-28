# Understanding variables in Rust

Welcome to this beginner-friendly guide on variables in Rust! In this document, we'll explore how Rust handles variables
and what makes them special compared to other programming languages.

## Declaring Variables
In Rust, you can declare a variable using the `let` keyword. Here's a simple example:

```rust
let x = 5;
```
By default, variables in Rust are immutable, meaning their values cannot be changed once assigned. If you try to reassign a value to `x`, you'll get an error:

```rust
x = 10; // This will cause a compilation error
```
## Mutability
If you want to create a variable that can be changed, you must explicitly mark it as mutable using the `mut` keyword:

```rust
let mut y = 10;
y = 20; // This is allowed because y is mutable
```

## Variable types
Rust is a statically typed language, but it can infer types:
```rust
let a = 5; // Rust infers that a is of type i32
let b = 3.14; // Rust infers that b is of type f64
```
You can also explicitly specify the type of a variable:

```rust
let c: i32 = 42; // c is explicitly declared as an i32
let d: f64 = 2.718; // d is explicitly declared as an f64
```
Common types include:
- `i8`, `i16`, `i32`, `i64`, `i128`: Signed integers of various sizes
- `u8`, `u16`, `u32`, `u64`, `u128`: Unsigned integers of various sizes
- `f32`, `f64`: Floating-point numbers
- `bool`: Boolean values (true or false)
- `char`: A single Unicode character
- `String`: A growable string type
- `&str`: A string slice, which is a reference to a string
  
## Shadowing