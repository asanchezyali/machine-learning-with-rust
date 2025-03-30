# Functions

Functions are essential building blocks in Rust programming. They allow you to encapsulate a block of code that can be
reused in multiple places. Functions are defined using the `fn` keyword followed by the function name, a list of
parameters enclosed in parentheses, and a return type. The function body is enclosed in curly braces `{}`.

## 📋 Basic function syntax
In Rust, functions are defined using the `fn` keyword:

```rust
fn say_hello() {
    println!("Hello, Rust world!");
}
```

In the example above, we define a function `say_hello()` that prints "Hello, Rust world!" to the console. We then call the
funtion `say_hello()` from the `main` function. The `main()` function is special - it's the entry point of your program.

## 📦 Functions with parameters

Functions can accept inputs called parameters:

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

In the example above, the `greet()` function accepts a parameter `name` of type `&str` (a string slice). We then call
the `greet()` function with different names.

## 📤 Functions with return values

Functions can return values using the `->` syntax:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // Note: no semicolon
}
```
Important: In Rust, the last expression in a function is implicitly returned. Therefore, you don't need to use the
`return` keyword. Just write the expression without a semicolon.

## 📦 Functions with multiple parameters
Functions can take multiple parameters:

```rust
fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}
```

In the example above, the `calculate_rectangle_area()` function takes two parameters `width` and `height` of type `f64`
(floating-point number) and returns the area of a rectangle.

## 📦 Functions with multiple return values
Rust doesn't directly support multiple return values, but you can return a tuple to achieve the same effect:

```rust
fn divide_and_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}
```

In the example above, the `divide_and_remainder()` function returns a tuple containing the quotient and remainder of the
division.

## 📦 Functions with no return value
If a function doesn't return a value, it implicitly return `()` (unit type):

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn explicit_no_return() -> () {
    println!("This function returns unit");
}
```

In the example above, the `explicit_no_return()` function returns `()` (unit type) because it doesn't return a value.

## 📝 Early returns with `return`
You can use the `return` keyword to return early from a function:

```rust
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    false
}
```

## 🔍 Function documentation
You can document your functions using doc comments (`///`):

```rust
/// Adds two numbers together
///
/// # Arguments
/// * `a` - The first number. An `i32` type number
/// * `b` - The second number. An `i32` type number
///
/// # Returns
/// The sum of the two numbers

/// # Examples
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## 🧩 Passing functions as arguments
Functions are first-class citizens in Rust, which means you can pass function as arguments to other functions:

```rust
fn apply_fn(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    f(a, b)
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    let result = apply_fn(add, 2, 3);
    println!("Result: {}", result);

    let result = apply_fn(multiply, 2, 3);
    println!("Result: {}", result);
}
```

## ⚡ Higher-order functions
Functions that take or return other functions are called higher-order functions. They are a powerful feature of Rust:

```rust
fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

fn main() {
    let double = create_multiplier(2);
    let triple = create_multiplier(3);

    println!("Double: {}", double(5));
    println!("Triple: {}", triple(5));
}
```

In the example above, the `create_multiplier()` function returns a closure that multiplies a number by a given factor.

## Examples in action
Here's complete example demonstrating various functions concepts:

```rust
fn say_hello() {
    println!("Hello, Rust world!");
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

fn divide_and_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    false
}

fn explicit_no_return() -> () {
    println!("This function returns unit");
}

fn apply_fn(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    f(a, b)
}

fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

fn main() {
    say_hello();
    greet("Alice");

    let sum = add(2, 3);
    println!("Sum: {}", sum);

    let area = calculate_rectangle_area(2.5, 3.5);
    println!("Area: {}", area);

    let (quotient, remainder) = divide_and_remainder(10, 3);
    println!("Quotient: {}, Remainder: {}", quotient, remainder);

    let is_even_number = is_even(5);
    println!("Is 5 even? {}", is_even_number);

    explicit_no_return();

    let result = apply_fn(add, 2, 3);
    println!("Result: {}", result);

    let double = create_multiplier(2);
    let triple = create_multiplier(3);
    println!("Double: {}", double(5));
    println!("Triple: {}", triple(5));
}
```

## 📚 Resources
- [Rust Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust Functions and Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
