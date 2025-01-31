fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("The value of x is: {}", x);
    let mut y = 8;
    println!("The value of y is: {}", y);
    y += 1;
    println!("The value of y is: {}", y);
    let z = 2;
    println!("The value of z is: {}", z);
    let w: u32 = 3;
    println!("The value of w is: {}", w);
    let a = 2.0;
    println!("The value of a is: {}", a);
    let b: f32 = 3.0;
    println!("The value of b is: {}", b);
    let c = add(z, 1);
    println!("The value of c is: {}", c);
    ownership_borrowing();
    print_user();
    process_message(Message::Quit);
    let result = divide((2.0, 0.0));
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn ownership_borrowing() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn divide((x, y): (f32, f32)) -> Result<f32, String> {
    if y == 0.0 {
        panic!("Cannot Divide by zero")
    }
    return Ok(x / y);
}

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
        "User: {}, Email: {}, Active: {}",
        user1.username, user1.email, user1.active
    );
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("fuck off"),
        Message::Move { x, y } => println!("Move to ({},{})", x, y),
        Message::Write(text) => println!("Text:{}", text),
    }
}
