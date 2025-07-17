// rust data types and varible assignment 1
fn main() {
    let tup: (&str, u8, bool) = ("Harish", 23, true);
    println!("name={} , age = {} , isdev = {}", tup.0, tup.1, tup.2);
    let dev = Token {
        name: String::from("solana"),
        symbol: String::from("sol"),
        supply: 678888888,
    };
    print_token(&dev);
}

// fn assignment and structs
struct Token {
    name: String,
    symbol: String,
    supply: u64,
}

fn print_token(token: &Token) {
    println!(
        "{} is Token whose symbol is {} and has a supply of {}",
        token.name, token.symbol, token.supply
    );
}
