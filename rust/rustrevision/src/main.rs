// this is stack and & points to reference so no error is thrown

fn main() {
    let a = String::from("Solana is amazing");
    let mut s = String::from("Btc is rocket");
    add_emoji(&mut s);
    let b = &a;
    print_length(b);
}

fn print_length(s: &String) {
    let length = s.len();
    print!("length of the string {}", length);
}

fn add_emoji(s: &mut String) {
    s.push_str(" 🚀");
    println!("muted String {}", s);
}
