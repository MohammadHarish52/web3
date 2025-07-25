use std::collections::HashMap;

fn s(text: &str) -> String {
    text.to_string()
}

fn main() {
    let mut tweet_map = HashMap::new();

    tweet_map.insert(1, s("hey"));
    tweet_map.insert(2, s("Yo yo yo"));
    tweet_map.insert(3, s("hani sengh"));

    for (i, v) in &tweet_map {
        println!("The key value pair is {} = {}", i, v)
    }

    match tweet_map.get(&2) {
        Some(tweet) => println!("tweet 2 is :{}", tweet),
        None => println!("Tweet not found"),
    }

    tweet_map.remove(&1);

    let tweet_count = tweet_map.len();
    println!("No of Tweeets left {}", tweet_count);
}
