fn get_tweet_by_id(id: u32) -> Option<String> {
    if id == 1 {
        return Some(String::from("This is the tweet"));
    }
    None
}

fn main() {
    let id = 1;
    match get_tweet_by_id(id) {
        Some(tweet) => println!("Tweet {}", tweet),
        None => println!("No tweet"),
    }
}
