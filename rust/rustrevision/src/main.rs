fn main() {
    println!("Hello, world!");
    // immutable variable
    let x = 6;
    println!("The value of x is: {}", x);
    // mutable variable
    let mut y = 9;
    println!("The value of y is: {}", y);
    y = 10;
    println!("The value of y is: {}", y);
    // const variable
    const PIE: f64 = 3.14;
    println!("The value of pie is: {}", PIE);
    // shadowing
    let z = 5;
    println!("The value of z is: {}", z);
    let z = z + 1; // shadowing
    println!("The value of z is: {}", z);
    let z = z * 2; // shadowing
    println!("The value of z is: {}", z);
    // data types
    let a: i32 = 10;
    let b: f64 = 3.14;
    let c: bool = true;
    let d: char = 'R';
    let e: &str = "Hello, Rust!";
    println!("a: {}, b: {}, c: {}, d: {}, e: {}", a, b, c, d, e);
    // tuples
    let tup: (i32, f64, char) = (10, 3.14, 'R');
    println!("Tuple: {:?}", tup);
    let (x, y, z) = tup; // destructuring
    println!("Destructured tuple: x: {}, y: {}, z: {}", x, y, z);
    // arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
    for i in arr.iter() {
        println!("Array element: {}", i);
    }
    // slices
    let slice: &[i32] = &arr[1..4]; // slice of the array
    println!("Slice: {:?}", slice);
    for i in slice.iter() {
        println!("Slice element: {}", i);
    }
    // functions
    let sum = add(5, 10);
    println!("Sum of 5 and 10 is: {}", sum);
}
// function to add two numbers
fn add(x: i32, y: i32) -> i32 {
    x + y
}
