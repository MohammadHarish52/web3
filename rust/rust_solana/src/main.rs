// ownership and borrowing

fn len_string(s: &String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("Big doobs");
    let le = len_string(&s);
    println!("The length of {}", le);
}
