// this is stack and & points to reference so no error is thrown
// mutable and immutable borrowing can't co exits

fn main() {
    let mut s = String::from("Harish");

    let r1 = &s;
    let r2 = &s;

    println!("{} {}", r1, r2);

    let r3 = &mut s;

    println!("{}", r3);
}

// fn main() {
//     let mut s = String::from("Harish");

//     let r1 = &s;
//     let r2 = &s;

//     let r3 = &mut s;  // error
//     println!("{} {}", r1, r2);

//     println!("{}", r3);
// }
