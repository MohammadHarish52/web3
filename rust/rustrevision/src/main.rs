// this is stack and & points to reference so no error is thrown

fn main() {
    let name = String::from("Harish");
    // let new_name = name; ownership error
    let new_name = &name;
    // borrowship using &
    println!("{}", &name);
    println!("{}", new_name);
}
