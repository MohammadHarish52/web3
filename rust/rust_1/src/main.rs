fn main() {
    println!("Hello, world!");
    // immutable variable
    let x = 5;
    println!("The value of x is: {}", x);
    // mutable variable
    let mut y = 8;
    println!("The value of y is: {}", y);
    y += 1;
    println!("The value of y is: {}", y);
    //Data types
    let z = 2; // i32
    println!("The value of z is: {}", z);
    let w: u32 = 3; // u32
    println!("The value of w is: {}", w);
    let a = 2.0; // f64
    println!("The value of a is: {}", a);
    let b: f32 = 3.0; // f32
    println!("The value of b is: {}", b);
    let c = add(z, 1);
    println!("The value of c is: {}", c);
    ownership_borrowing();
    // print user
    print_user();
    // Error handling
    let result = divide((2.0, 0.0));
}

// functions
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// ownership and borrowing
fn ownership_borrowing() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Borrowing
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Error handling
fn divide((x, y): (f32, f32)) -> Result<f32, String> {
    if y == 0.0 {
        panic!("Cannot Divide by zero")
    }
    return Ok(x / y);
}

// Intermediate rust

// Structs
struct User {
    username: String,
    email: String,
    active: bool,
}

fn print_user() {
    let user1 = User {
        username: String::from("Harish"),
        email: String::from("djramesh@gmail.com"),
        active: true,
    };
    println!(
        "User: {}, Email: {}, Active
        : {}",
        user1.username, user1.email, user1.active
    );
}
