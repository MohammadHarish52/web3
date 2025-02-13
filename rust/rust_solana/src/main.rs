// syntax and basic data types

fn sum_elements(numbers: Vec<u64>) {
    let mut total: u64 = 0;
    for i in numbers {
        total = total + i;
    }
    println!("Total: {}", total);
}

fn main() {
    let numbers = vec![3, 4, 5, 5, 6, 5];
    sum_elements(numbers);
}
