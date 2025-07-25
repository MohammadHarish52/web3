fn s(text: &str) -> String {
    text.to_string()
}

fn main() {
    let mut tweets: Vec<String> = Vec::new();

    tweets.push(s("Yo hi"));
    tweets.push(s("kese ho"));
    tweets.push(s("Sab acha hoga"));

    for x in &tweets {
        println!("Tweet : {}", x)
    }
    tweets.pop();

    let l = tweets.len();
    println!("len:{}", l);
}
