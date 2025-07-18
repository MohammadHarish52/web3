// this is stack and & points to reference so no error is thrown

fn main() {
    let a = String::from("Solana is amazing");
    let b = &a;
    print_length(b);
}

fn print_length(s: &String) {
    let length = s.len();
    print!("length of the string {}", length);
}
