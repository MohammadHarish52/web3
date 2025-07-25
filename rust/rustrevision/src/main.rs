fn calculate(a: i32, b: i32, op: char) -> Result<i32, String> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0 {
                Err("dicide by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err("Unsupported".to_string()),
    }
}

fn main() {
    let result = calculate(10, 3, '/');

    match result {
        Ok(value) => println!("Result:{}", value),
        Err(e) => println!("err{}", e),
    }
}
