# Conditionals

Conditionals are essential for controlling flow of your program. Rust provides two main ways to handle conditionals
logic: traditional `if` statements and pattern matching with `match`. Let's explore both of these in detail.

## 🔍 If statements
The `if` statement is a fundamental control flow construct in Rust. It allows you to execute code conditionally based on
the evaluation of a boolean expression. The syntax is straightforward:

```rust
fn main() {
    let number = 7;
    if number < 5 {
        println!("The number is less than 5");
    } else if number == 5 {
        println!("The number is equal to 5");
    } else {
        println!("The number is greater than 5");
    }
}
```

In Rust, `if` is an expression, which means it can return a value:

```rust
fn main() {
    let number = 7;
    let result = if number < 5 {
        "The number is less than 5"
    } else if number == 5 {
        "The number is equal to 5"
    } else {
        "The number is greater than 5"
    };
    println!("{}", result);
}
```
Importantly, the branches of an `if` statement must return the same type. This is a key difference from some other
languages where branches can return different types. In Rust, the compiler will enforce this rule, and if the types
don't match, you'll get compilation errors. This ensure type safety and consistency in your code.

## 🧩 Pattern matching with `match`

Pattern matching is one of Rust's most powerfull features. The `match` expression allows you to compare a value against
a series of patterns and execute code based on which pattern matches. This is similar to switch statement in other
languages, but much more powerful and flexible. Here's a simple example:

```rust
fn main() {
    let dice_roll = 4;
    match dice_roll {
        1 => println!("You rolled a one!"),
        2 => println!("You rolled a two!"),
        3 => println!("You rolled a three!"),
        4 => println!("You rolled a four!"),
        5 => println!("You rolled a five!"),
        6 => println!("You rolled a six!"),
        _ => println!("That's not a valid roll!"), // The underscore (_) is a catch-all pattern
    }
}
```

In this example, the `match` expression checks the value of `dice_roll` against each of the patterns. If a pattern
matches, the corresponding block of code is executed. The underscore (_) pattern acts as a catch-all for any values that
don't match the previous patterns. This is similar to the `default` case in a switch statement.

### Match as an expression
Like `if`, `match` is also an expression in Rust. This means it can return a value. Here's an example:

```rust
fn main() {
    let number = 7;
    let result = match number {
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => "Greater than three",
    };
    println!("{}", result);
}
```

### Pattern ranges
