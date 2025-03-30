# Understanding variables in Rust

Welcome to this beginner-friendly guide on variables in Rust! In this document, we'll explore how Rust handles variables
and what makes them special compared to other programming languages.

## 📌 Declaring variables
In Rust, you can declare a variable using the `let` keyword. Here's a simple example:

```rust
let x = 5;
```
By default, variables in Rust are immutable, meaning their values cannot be changed once assigned. If you try to reassign a value to `x`, you'll get an error:

```rust
x = 10; // This will cause a compilation error
```
## 🔄 Mutability
If you want to create a variable that can be changed, you must explicitly mark it as mutable using the `mut` keyword:

```rust
let mut y = 10;
y = 20; // This is allowed because y is mutable
```

## 📦 Variable types
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
- `array`: A fixed-size array
- `tuple`: A fixed-size collection of different types
  
## 🎭 Shadowing
Rust allows you to declare a new variable with the same name as an existing one, this is called shadowing. The new variable "shadows" the old one:

```rust
let count = 10;
let count = count + 1; // This shadows the previous count variable
println!("Count: {}", count); // Output: Count: 11
```

Shadowing is different from mutation because:
1. You use `let` again to create a new variable.
2. The original variable is not changed; instead, a new variable with the same name is created.
3. You can change the type of the variable when shadowing.
```rust
let text = "hello"; // text is a &str
let text = text.len(); // text is now an integer!
```

> [!CAUTION]
> Shadowing allows you to reuse variable names and change their types, which can be useful in certain situations. However, it can also lead to confusion if not used carefully.

## 🌟 Constants
Constants are similar to immutable variables but with a few differences:
- They are declared using the `const` keyword.
- They must have a type annotation.
- Value must be a constant expression, not a variable.
- Can be declared in any scope, including global scope.
- Cannot be shadowed.
- They are always immutable.
- They can be used in constant expressions, such as array sizes.
- They are evaluated at compile time, not runtime.

Here's an example of declaring a constant:

```rust
const MAX_POINTS: u32 = 100_000; // Constants are usually written in uppercase
```
You can use constants in your code just like variables:

```rust
fn main() {
    println!("The maximum points are: {}", MAX_POINTS);
}
```

## 🧩 Scope and blocks
Variables in Rust have block scope. A block is a collection of statements enclosed in curly braces `{}`. Variables declared inside a block are not accessible outside of it:

```rust
fn main() {
    let x = 5; // x is valid here
    {
        let y = 10; // y is valid here
        println!("x: {}, y: {}", x, y); // Output: x: 5, y: 10
    }
    println!("x: {}", x); // Output: x: 5
    // println!("y: {}", y); // This will cause a compilation error
}
```

## 📝 Examples

Here's a small program that demonstrates the concepts we've covered:

```rust
fn main() {
  // Immutable variable
  let x = 5;
  println!("The value of x is: {}", x);

  // Mutable variable
  let mut y = 10;
  println!("The value of y is: {}", y);
  y += 5;
  println!("The new value of y is: {}", y);

  // Shadowing
  let z = 20;
  let z = z + 10; // Shadowing
  println!("The value of z is: {}", z);

  // Constants
  const MAX_POINTS: u32 = 100_000;
  println!("The maximum points are: {}", MAX_POINTS);

  // Scope
  {
    let a = 30;
    println!("The value of a is: {}", a);
  }
  // println!("The value of a is: {}", a); // This will cause a compilation error

  // Type inference
  let b = 3.14; // Rust infers that b is of type f64
  println!("The value of b is: {}", b);
  // Explicit type annotation
  let c: i32 = 42; // c is explicitly declared as an i32
  println!("The value of c is: {}", c);
  // String and &str
  let string_var = String::from("Hello, Rust!"); // String
  let str_var: &str = "Hello, World!"; // &str
  println!("String variable: {}", string_var);
  println!("String slice variable: {}", str_var);

  // Boolean
  let is_rust_fun = true;
  println!("Is Rust fun? {}", is_rust_fun);
  
  // Character
  let char_var: char = 'R';
  println!("Character variable: {}", char_var);

  // Array
  let arr: [i32; 5] = [1, 2, 3, 4, 5];
  println!("Array variable: {:?}", arr);

  // Tuple
  let tuple: (i32, f64, char) = (42, 3.14, 'R');
  println!("Tuple variable: {:?}", tuple);
}
```

## 🎲 Type inference best practices
While Rust can infer types, it's often a good practice to be explicit in function signatures and when defining complex
structures to make your coder more readable.

## 🚫 The uninitialized variable trap
Unlike some languages, Rust doesn't allow you declare a variable without initializing it:

```rust
let uninitialized_var; // This will cause a compilation error
uninitialized_var = 10; // This line won't be reached

// To fix this, initialize the variable when declaring it
let initialized_var = 10; // This is valid
```

## 🎓 Advanced concept: the unit type
In Rust, the unit type `()` is a special type that represents an empty value. It's often used in functions that don't return a meaningful value. For example:

```rust
fn do_nothing() -> () {
    // This function does nothing and returns the unit type
}
fn main() {
    let result = do_nothing();
    println!("The result is: {:?}", result); // Output: The result is: ()
}
```
This is similar to `void` in other languages, but in Rust, `()` is a first-class type that can be used in expressions
and patterns.

## 🚀 Ready for more?
Now that you understand variables, you're ready to explore more complex Rust concept like:
- Data Types
- Functions
- Control Flow
- Ownership and Borrowing
- Structs and Enums

Happy coding!
```