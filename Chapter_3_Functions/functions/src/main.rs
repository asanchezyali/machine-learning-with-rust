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