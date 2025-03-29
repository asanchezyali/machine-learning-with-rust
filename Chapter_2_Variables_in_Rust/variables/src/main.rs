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
