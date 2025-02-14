// syntax and basic data types

struct Product {
    name: String,
    price: u64,
}

fn calculate_total_price(products: Vec<Product>) -> u64 {
    let mut total: u64 = 0;
    for p in products {
        total += p.price;
    }
    total
}
fn main() {
    let product1 = Product {
        name: "product1".to_string(),
        price: 100,
    };
    let product2 = Product {
        name: "product2".to_string(),
        price: 200,
    };
    let product3 = Product {
        name: "product3".to_string(),
        price: 300,
    };
    let products = vec![product1, product2, product3];
    let total = calculate_total_price(products);
    println!("Total price: {}", total);
}
